use image::io::Reader as ImageReader;
use std::path::Path;
use std::ffi::OsStr;


fn load_image(path:&str) -> Result<bool, &str>{
    let extension = validate_extension(path)?;
    //let img = ImageReader::open(path)?.decode()?;

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

fn main() {
    load_image("playbook.jpg");
    println!("Hello, world!");
}
