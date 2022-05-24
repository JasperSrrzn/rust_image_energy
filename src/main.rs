extern crate core;
use crate::energy_of_image::image_wrapper::ImageWrapper;
mod energy_of_image;

fn main() {
    let image = ImageWrapper::from_file("./images/surfer.jpg");
    let energy = image.get_energy_image();
    let image_result = energy.save("./images/energy.jpg");
    match image_result {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }
}









