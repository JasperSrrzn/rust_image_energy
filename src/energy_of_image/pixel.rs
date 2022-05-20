use image::{DynamicImage, GenericImageView, Rgba};

pub fn get_left(pixel: &(u32, u32, Rgba<u8>), image: &DynamicImage) -> Option<Rgba<u8>> {
    if pixel.0 >= 1 {
        return Some(image.get_pixel(pixel.0 - 1, pixel.1));
    }
    None
}

pub fn get_top(pixel: &(u32, u32, Rgba<u8>), image: &DynamicImage) -> Option<Rgba<u8>> {
    if pixel.1 >= 1 {
        return Some(image.get_pixel(pixel.0, pixel.1 - 1));
    }
    None
}

pub fn get_right(pixel: &(u32, u32, Rgba<u8>), image: &DynamicImage) -> Option<Rgba<u8>> {
    if pixel.0 < image.width() - 1 {
        return Some(image.get_pixel(pixel.0 + 1, pixel.1));
    }
    None
}

pub fn get_bottom(pixel: &(u32, u32, Rgba<u8>), image: &DynamicImage) -> Option<Rgba<u8>> {
    if pixel.1 < image.height() - 1 {
        return Some(image.get_pixel(pixel.0, pixel.1 + 1));
    }
    None
}