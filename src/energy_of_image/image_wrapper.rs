use image::DynamicImage;
use image::io::Reader as ImageReader;

use crate::energy_of_image::energy::get_energy_image;

pub struct ImageWrapper {
    original: Option<DynamicImage>,
}

impl ImageWrapper {
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

    pub fn get_energy(&self) -> DynamicImage {
        match &self.original {
            Some(image) => {
                get_energy_image(image)
            }
            None => {
                panic!("No image was loaded to calculate energy")
            }
        }
    }
}
