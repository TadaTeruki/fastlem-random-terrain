use clap::Parser;
use colormap::ColorMap;
use config::{Config, ConfigParser};
use fastlem::core::{parameters::TopographicalParameters, traits::Model};
use fastlem::lem::generator::TerrainGenerator;
use fastlem::models::surface::terrain::Terrain2D;
use fastlem::models::surface::{builder::TerrainModel2DBulider, sites::Site2D};
use noise::{NoiseFn, Perlin};
use terrain_graph::edge_attributed_undirected::EdgeAttributedUndirectedGraph;

use crate::math::inversed_perlin_noise_curve;

mod colormap;
mod config;
mod math;

fn main() {
    let config = ConfigParser::parse().into_config();

    let bound_min = Site2D {
        x: -config.bound_width / 2.0,
        y: -config.bound_height / 2.0,
    };
    let bound_max = Site2D {
        x: config.bound_width / 2.0,
        y: config.bound_height / 2.0,
    };
    let bound_range = Site2D {
        x: config.bound_width,
        y: config.bound_height,
    };

    let colormap = match &config.colormap_json_filename {
        Some(colormap_json_filename) => {
            ColorMap::new_from_json_file(colormap_json_filename).unwrap()
        }
        None => ColorMap::default(),
    };

    let image_width = match &config.image_width {
        Some(width) => *width,
        None => {
            if let Some(height) = &config.image_height {
                *height * bound_range.x as u32 / bound_range.y as u32
            } else {
                0u32
            }
        }
    };

    let image_height = match &config.image_height {
        Some(height) => *height,
        None => {
            if let Some(width) = &config.image_width {
                *width * bound_range.y as u32 / bound_range.x as u32
            } else {
                0u32
            }
        }
    };

    if image_width == 0 || image_height == 0 {
        panic!("Both image_width and image_height must be specified.");
    }

    let terrain = generate_terrain(&config, bound_min, bound_max, bound_range);
    write_to_image(
        &config.output_filename,
        bound_min,
        bound_range,
        image_width,
        image_height,
        &terrain,
        colormap,
    );
}

fn generate_terrain(
    config: &Config,
    bound_min: Site2D,
    bound_max: Site2D,
    bound_range: Site2D,
) -> Terrain2D {
    // Seed of the noise generator.
    // You can generate various terrains by changing the seed.
    let seed = config.seed;

    // Noise generator
    let perlin = Perlin::new(seed);

    let num = config.particle_num;

    println!("creating a model...");

    let model = TerrainModel2DBulider::from_random_sites(num, bound_min, bound_max)
        .relaxate_sites(10)
        .unwrap()
        .add_edge_sites(None, None)
        .unwrap()
        .build()
        .unwrap();

    println!("distributing params...");

    let sites = model.sites().to_vec();

    // fault
    let fault_scale = config.fault_scale;

    let get_fault = |site: &Site2D| -> (f64, f64) {
        let scale = 100.0;
        let modulus = octaved_perlin(&perlin, site.x / scale, site.y / scale, 3, 0.5, 2.0).abs()
            * 2.0
            * fault_scale;
        let direction_x = octaved_perlin(
            &perlin,
            (site.x + bound_range.x) / scale,
            (site.y + bound_range.y) / scale,
            4,
            0.6,
            2.2,
        ) * 2.0;
        let direction_y = octaved_perlin(
            &perlin,
            (site.x - bound_range.x) / scale,
            (site.y - bound_range.y) / scale,
            4,
            0.6,
            2.2,
        ) * 2.0;
        (direction_x * modulus, direction_y * modulus)
    };

    let apply_fault = |site: &Site2D| -> Site2D {
        let fault = get_fault(site);
        let fault_x = site.x + fault.0;
        let fault_y = site.y + fault.1;
        Site2D {
            x: fault_x,
            y: fault_y,
        }
    };

    let land_bias = -(inversed_perlin_noise_curve(config.land_ratio) - 0.5);

    let base_is_outlet = {
        sites
            .iter()
            .map(|site| {
                let site = &apply_fault(site);
                let persistence_scale = 50.;
                let noise_persistence = octaved_perlin(
                    &perlin,
                    site.x / persistence_scale,
                    site.y / persistence_scale,
                    2,
                    0.5,
                    2.0,
                )
                .abs()
                    * 0.7
                    + 0.3;
                let plate_scale = 50.;
                let noise_plate = octaved_perlin(
                    &perlin,
                    site.x / plate_scale,
                    site.y / plate_scale,
                    8,
                    noise_persistence,
                    2.4,
                ) * 0.5
                    + 0.5;
                let continent_scale: f64 = 200.;
                let noise_continent = octaved_perlin(
                    &perlin,
                    site.x / continent_scale,
                    site.y / continent_scale,
                    3,
                    0.5,
                    1.8,
                ) * 0.7
                    + 0.5;
                noise_plate > noise_continent - land_bias
            })
            .collect::<Vec<bool>>()
    };

    let start_index = (num + 1..sites.len()).collect::<Vec<_>>();
    let graph = model.graph();

    let is_outlet = determine_outlets(&sites, base_is_outlet, start_index, graph).unwrap();

    println!("generating...");

    let erodibility_distribution_power = config.erodibility_distribution_power;
    let parameters = {
        sites
            .iter()
            .enumerate()
            .map(|(i, site)| {
                let site = &apply_fault(site);
                let erodibility_scale = 75.0;
                let noise_erodibility = (1.0
                    - octaved_perlin(
                        &perlin,
                        site.x / erodibility_scale,
                        site.y / erodibility_scale,
                        5,
                        0.7,
                        2.2,
                    ) * 2.0)
                    .abs()
                    .powf(erodibility_distribution_power)
                    * 0.5
                    + 0.1;

                TopographicalParameters::default()
                    .set_erodibility(noise_erodibility)
                    .set_is_outlet(is_outlet[i])
            })
            .collect::<Vec<TopographicalParameters>>()
    };

    TerrainGenerator::default()
        .set_model(model)
        .set_parameters(parameters)
        .generate()
        .unwrap()
}

fn write_to_image(
    output_filename: &str,
    bound_min: Site2D,
    bound_range: Site2D,
    image_width: u32,
    image_height: u32,
    terrain: &Terrain2D,
    colormap: ColorMap,
) {
    println!("writing...");

    let mut image_buf = image::RgbImage::new(image_width, image_height);

    for imgx in 0..image_width {
        for imgy in 0..image_height {
            let x =
                bound_min.x + bound_range.x * ((imgx as f64 + 0.5) / (image_width as f64 + 1.0));
            let y =
                bound_min.y + bound_range.y * ((imgy as f64 + 0.5) / (image_height as f64 + 1.0));
            let site = Site2D { x, y };
            let elevation = terrain.get_elevation(&site);
            if let Some(elevation) = elevation {
                image_buf.put_pixel(imgx, imgy, image::Rgb(colormap.get_color(elevation)));
            }
        }
    }
    image_buf.save(output_filename).unwrap();
}

fn octaved_perlin(
    perlin: &Perlin,
    x: f64,
    y: f64,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
) -> f64 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        value += perlin.get([x * frequency, y * frequency, 0.0]) * amplitude;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= lacunarity;
    }

    value / max_value
}

fn determine_outlets(
    sites: &Vec<Site2D>,
    base_is_outlet: Vec<bool>,
    start_index: Vec<usize>,
    graph: &EdgeAttributedUndirectedGraph<f64>,
) -> Option<Vec<bool>> {
    let random_start_index = if start_index.is_empty() {
        None
    } else {
        Some(start_index[0])
    };
    let mut queue = start_index
        .into_iter()
        .filter(|i| base_is_outlet[*i])
        .collect::<Vec<_>>();
    let mut is_outlet = if !queue.is_empty() {
        let mut is_outlet = vec![false; sites.len()];
        while let Some(i) = queue.pop() {
            if is_outlet[i] {
                continue;
            }
            is_outlet[i] = true;
            graph.neighbors_of(i).iter().for_each(|(j, _)| {
                if !is_outlet[*j] && base_is_outlet[*j] {
                    queue.push(*j);
                }
            });
        }
        is_outlet
    } else {
        vec![false; sites.len()]
    };

    if is_outlet.iter().any(|&b| b) {
        Some(is_outlet)
    } else if let Some(i) = random_start_index {
        is_outlet[i] = true;
        Some(is_outlet)
    } else {
        None
    }
}
