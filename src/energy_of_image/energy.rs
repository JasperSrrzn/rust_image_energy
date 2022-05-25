use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::energy_of_image::pixel::{get_bottom, get_left, get_right, get_top};

/// This returns the energy of the image, but scaled to visualize as an image
pub fn get_energy_image(image: &DynamicImage) -> DynamicImage {
    let mut max: u32 = 0;
    let mut min: u32 = u32::MAX;
    let mut energies = HashMap::new();
    let mut energy_image = image.clone();
    image.pixels().for_each(|(x, y, _)| {
        let energy = calculate_energy_at(&x, &y, &image);
        if energy > max {
            max = energy;
        }
        if energy < min {
            min = energy;
        }
        energies.insert((x, y), energy);
    });

    energies.iter().for_each(|((x, y), energy)| {
        let scaled_energy = scale(energy, max, min);
        let rgba = Rgba::from([scaled_energy, scaled_energy, scaled_energy, 255]);
        energy_image.put_pixel(*x, *y, rgba);
    });
    energy_image
}

/// This returns the energy of the image, not scaled.
pub fn get_energy_grid(image: &DynamicImage) -> HashMap<(u32, u32), u32> {
    let mut energies = HashMap::new();
    image.pixels().for_each(|(x, y, _)| {
        let energy = calculate_energy_at(&x, &y, &image);
        energies.insert((x, y), energy);
    });
    energies
}


fn calculate_energy_at(x: &u32, y: &u32, image: &DynamicImage) -> u32 {
    let left = get_left(x, y, &image);
    let right = get_right(x, y, &image);
    let top = get_top(x, y, &image);
    let bottom = get_bottom(x, y, &image);
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