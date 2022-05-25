use image::{DynamicImage, GenericImageView, Rgba};

pub fn get_left(x: &u32, y: &u32, image: &DynamicImage) -> Option<Rgba<u8>> {
    if *x >= 1 {
        return Some(image.get_pixel(x - 1, *y));
    }
    None
}

pub fn get_top(x: &u32, y: &u32, image: &DynamicImage) -> Option<Rgba<u8>> {
    if *y >= 1 {
        return Some(image.get_pixel(*x, y - 1));
    }
    None
}

pub fn get_right(x: &u32, y: &u32, image: &DynamicImage) -> Option<Rgba<u8>> {
    if *x < image.width() - 1 {
        return Some(image.get_pixel(x + 1, *y));
    }
    None
}

pub fn get_bottom(x: &u32, y: &u32, image: &DynamicImage) -> Option<Rgba<u8>> {
    if *y < image.height() - 1 {
        return Some(image.get_pixel(*x, y + 1));
    }
    None
}