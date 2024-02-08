/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorMapEntry {
    color: [u8; 3],
    elevation: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorMap {
    colormap: Vec<ColorMapEntry>,
}

impl Default for ColorMap {
    fn default() -> Self {
        let colormap = vec![
            ColorMapEntry {
                color: [0, 0, 0],
                elevation: 0.0,
            },
            ColorMapEntry {
                color: [255, 255, 255],
                elevation: 100.0,
            },
        ];
        ColorMap { colormap }
    }
}

impl ColorMap {
    pub fn new_from_json_file<P: AsRef<Path>>(filename: P) -> Result<ColorMap, serde_json::Error> {
        let file = File::open(filename).expect("Failed to open json file");
        let reader = BufReader::new(file);
        let color_vec: Vec<ColorMapEntry> = serde_json::from_reader(reader)?;
        Ok(ColorMap {
            colormap: color_vec,
        })
    }

    pub fn get_color(&self, elevation: f64) -> [u8; 3] {
        let color_index = {
            let mut i = 0;
            while i < self.colormap.len() {
                if elevation < self.colormap[i].elevation {
                    break;
                }
                i += 1;
            }
            i
        };

        if color_index == 0 {
            self.colormap[0].color
        } else if color_index == self.colormap.len() {
            self.colormap[self.colormap.len() - 1].color
        } else {
            let color_a = &self.colormap[color_index - 1];
            let color_b = &self.colormap[color_index];

            let prop = (elevation - color_a.elevation) / (color_b.elevation - color_a.elevation);
            blend_color(color_a.color, color_b.color, prop)
        }
    }
}

fn blend_color(color_a: [u8; 3], color_b: [u8; 3], prop: f64) -> [u8; 3] {
    [
        (color_a[0] as f64 + (color_b[0] as f64 - color_a[0] as f64) * prop) as u8,
        (color_a[1] as f64 + (color_b[1] as f64 - color_a[1] as f64) * prop) as u8,
        (color_a[2] as f64 + (color_b[2] as f64 - color_a[2] as f64) * prop) as u8,
    ]
}
