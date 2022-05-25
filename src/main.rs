extern crate core;

use std::time::{Duration, Instant};

use crate::energy_of_image::image_wrapper::ImageWrapper;

mod energy_of_image;

fn main() {
    let image = ImageWrapper::from_file("./images/surfer.jpg");
    let start = Instant::now();
    let energy = image.get_energy_image();
    let duration = start.elapsed();
    println!("time elapsed: {:?}", duration);
    let image_result = energy.save("./images/energy.jpg");
    match image_result {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }
}









