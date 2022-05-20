use std::collections::HashMap;

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use crate::energy_of_image::pixel::{get_left, get_top, get_right, get_bottom};

pub fn get_energy_image(image: &DynamicImage) -> DynamicImage {
    let mut max: u32 = 0;
    let mut min: u32 = u32::MAX;
    let mut energies = HashMap::new();
    let mut energy_image = image.clone();
    image.pixels().for_each(|pixel: (u32, u32, Rgba<u8>)| {
        let energy = calculate_energy_at(&pixel, &image);
        if energy > max {
            max = energy;
        }
        if energy < min {
            min = energy;
        }
        energies.insert((pixel.0, pixel.1), energy);
    });

    energies.iter().for_each(|((x, y), energy)| {
        let scaled_energy = scale(energy, max, min);
        let rgba = Rgba::from([scaled_energy, scaled_energy, scaled_energy, 255]);
        energy_image.put_pixel(*x, *y, rgba);
    });
    energy_image
}


fn calculate_energy_at(pixel: &(u32, u32, Rgba<u8>), image: &DynamicImage) -> u32 {
    let left = get_left(&pixel, &image);
    let right = get_right(&pixel, &image);
    let top = get_top(&pixel, &image);
    let bottom = get_bottom(&pixel, &image);
    let dx = calculate_energy(left, right);
    let dy = calculate_energy(top, bottom);
    dx + dy
}

fn calculate_energy(option_pixel1: Option<Rgba<u8>>, option_pixel2: Option<Rgba<u8>>) -> u32 {
    match option_pixel1 {
        Some(pixel1) => {
            match option_pixel2 {
                Some(pixel2) => {
                    let rgb1 = pixel1.0;
                    let rgb2 = pixel2.0;
                    let dr: u32 = ((rgb1[0] as i32 - rgb2[0] as i32) * (rgb1[0] as i32 - rgb2[0] as i32)) as u32;
                    let dg: u32 = ((rgb1[1] as i32 - rgb2[1] as i32) * (rgb1[1] as i32 - rgb2[1] as i32)) as u32;
                    let db: u32 = ((rgb1[2] as i32 - rgb2[2] as i32) * (rgb1[2] as i32 - rgb2[2] as i32)) as u32;
                    dr + dg + db
                }
                None => {
                    let rgb1 = pixel1.0;
                    let dr: u32 = (rgb1[0] as u32) * (rgb1[0] as u32);
                    let dg: u32 = (rgb1[1] as u32) * (rgb1[1] as u32);
                    let db: u32 = (rgb1[2] as u32) * (rgb1[2] as u32);
                    dr + dg + db
                }
            }
        }
        None => {
            match option_pixel2 {
                Some(pixel2) => {
                    let rgb2 = pixel2.0;
                    let dr: u32 = (rgb2[0] as u32) * (rgb2[0] as u32);
                    let dg: u32 = (rgb2[1] as u32) * (rgb2[1] as u32);
                    let db: u32 = (rgb2[2] as u32) * (rgb2[2] as u32);
                    dr + dg + db
                }
                None => {
                    panic!("No neighbours")
                }
            }
        }
    }
}

fn scale(energy: &u32, max: u32, min: u32) -> u8 {
    (255 * (energy - min) / (max - min)) as u8
}