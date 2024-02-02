use clap::Parser;

#[derive(Parser, Debug)]
pub struct ConfigParser {
    /// Width and height (width:height) of the bound.
    #[clap(short, long, default_value = "100.0:100.0")]
    pub bound: String,

    /// Seed of the noise generator.
    #[clap(short, long, default_value = "0")]
    pub seed: u32,

    /// Number of particles.
    /// The larger the value, the more the quality of the terrain is improved. [advanced]
    #[clap(short, long, default_value = "50000")]
    pub particle_num: usize,

    /// Scale of the fault.
    /// The larger the value, the more virtual faults effect the terrain. [advanced]
    #[clap(short, long, default_value = "35.0")]
    pub fault_scale: f64,

    /// Power of the erodibility distribution.
    /// The larger the value, the more the erodibility is concentrated on the lower side. [advanced]
    #[clap(short, long, default_value = "4.0")]
    pub erodibility_distribution_power: f64,

    /// JSON file of the colormap.
    /// If not specified, the grayscale colormap is used.
    #[clap(short, long, default_value = "")]
    pub colormap_json_file: String,

    /// Width and height (width:height) of the image.
    /// If -1 is specified, the aspect ratio is the same as the bound.
    #[clap(short, long, default_value = "1024:-1")]
    pub image_size: String,

    /// File name of the output image.
    #[clap(short, long, default_value = "terrain.png")]
    pub image_file: String,
}

impl ConfigParser {
    fn string_into_two_floats(s: &str) -> (f64, f64) {
        let v: Vec<&str> = s.split(':').collect();
        if v.len() != 2 {
            panic!("Invalid format: {}", s);
        }
        let x = v[0].parse().unwrap();
        let y = v[1].parse().unwrap();
        (x, y)
    }

    fn string_into_two_option_uints(s: &str) -> (Option<u32>, Option<u32>) {
        let v: Vec<&str> = s.split(':').collect();
        if v.len() != 2 {
            panic!("Invalid format: {}", s);
        }
        let x = if v[0] == "-1" {
            None
        } else {
            Some(v[0].parse().unwrap())
        };
        let y = if v[1] == "-1" {
            None
        } else {
            Some(v[1].parse().unwrap())
        };
        (x, y)
    }

    pub fn into_config(self) -> Config {
        let (bound_width, bound_height) = ConfigParser::string_into_two_floats(&self.bound);
        let (image_width, image_height) = ConfigParser::string_into_two_option_uints(&self.image_size);
        Config {
            bound_width,
            bound_height,
            seed: self.seed,
            particle_num: self.particle_num,
            fault_scale: self.fault_scale,
            erodibility_distribution_power: self.erodibility_distribution_power,
            colormap_json_file: if self.colormap_json_file.is_empty() {
                None
            } else {
                Some(self.colormap_json_file.clone())
            },
            image_width,
            image_height,
            image_file: self.image_file.clone(),
        }
    }
}

pub struct Config {
    pub bound_width: f64,
    pub bound_height: f64,
    pub seed: u32,
    pub particle_num: usize,
    pub fault_scale: f64,
    pub erodibility_distribution_power: f64,
    pub colormap_json_file: Option<String>,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_file: String,
}
