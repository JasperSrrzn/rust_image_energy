use std::collections::HashMap;
use image::DynamicImage;
use image::io::Reader as ImageReader;

use crate::energy_of_image::energy::{get_energy_grid, get_energy_image};

pub struct ImageWrapper {
    original: Option<DynamicImage>,
}

impl ImageWrapper {

    /// loads the wrapper with an image stored at the path argument
    pub fn from_file(path: &str) -> Self {
        let result = ImageReader::open(path);
        match result {
            Ok(reader) => {
                let image_result = reader.decode();
                match image_result {
                    Ok(image) => {
                        ImageWrapper {
                            original: Some(image)
                        }
                    }
                    Err(e) => {
                        panic!("{}", e)
                    }
                }
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }

    /// get the energy grid
    pub fn get_energy(&self) -> HashMap<(u32,u32), u32> {
        match &self.original {
            Some(image) => {
                get_energy_grid(image)
            },
            None =>{
                panic!("No image was loaded to calculate energy")
            }
        }
    }

    /// get the energy image
    pub fn get_energy_image(&self) -> DynamicImage {
        match &self.original {
            Some(image) => {
                get_energy_image(image)
            },
            None => {
                panic!("No image was loaded to calculate energy")
            }
        }
    }

    /// get the image out of the wrapper
    pub fn get_image(&self) -> &DynamicImage {
        match &self.original {
            Some(image) => image,
            None => {
                panic!("No image was loaded yet!")
            }
        }
    }
}
