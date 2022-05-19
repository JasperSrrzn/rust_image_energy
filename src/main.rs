use image::io::Reader as ImageReader;

fn main() {
    let result = ImageReader::open("./images/france.jpg");
    match result {
        Ok(reader) => {
            let image_result = reader.decode();
            match image_result {
                Ok(image) => {
                    println!("The image width is {}", image.width())
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






