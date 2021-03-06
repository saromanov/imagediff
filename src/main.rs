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
    let mut img1 = load_image(first_image).unwrap().into_bytes();
    let mut img2 = load_image(second_image).unwrap().into_bytes();
    compare_images(&img1, &img2);
    let count = diff_count(img1, img2);
    println!("{}", diff_count)
}

fn compare_images<T>(img1: &Vec<T>, img2: &Vec<T>){
    if img1.len() != img2.len() {
        println!("size of the images is not equal")
    }
}

fn diff_count<T>(img1:&Vec<T>, img2: &Vec<T>) -> u32 {
    img1.iter().zip(img2.iter()).filter(|&(a, b)| a != b).count()
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
