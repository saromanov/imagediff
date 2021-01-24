use image::io::Reader as ImageReader;
use image::{ImageError, DynamicImage};
use std::path::Path;
use std::ffi::OsStr;
use std::env;


fn load_image(path:&str) -> Result<DynamicImage,ImageError> {
    let extension = validate_extension(path);
    ImageReader::open(path)?.decode()
}

// provides validate of extension
fn validate_extension(path: &str) -> Result<bool, &str> {
    let valid_extensions = vec!["jpg", "jpeg", "png"];
    let path_extension = Path::new(path)
        .extension().and_then(OsStr::to_str);
    match path_extension {
        Some(extension) => Ok(valid_extensions.into_iter().any(|x| x == extension)),
        _ => Err("unable to get extension"),
    }
}

fn diff(first_image: &str, second_image: &str) {
    let img1 = load_image(first_image).unwrap();
    let img2 = load_image(second_image).unwrap();
   /* if img1.into_bytes().len() != img2.into_bytes().len() {
        println!("size of the images is not equal")
    }*/
    let mut img1_data = img1.into_bytes();
    let mut img2_data = img2.into_bytes();
    let diff_count = img1_data.iter().zip(img2_data.iter()).filter(|&(a, b)| a != b).count();
    println!("{}", diff_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return
    }
    let first_image = &args[1];
    let second_image = &args[2];
    diff(first_image, second_image);
}
