mod args;
use args::Args;
use std::{fs::File, io::BufReader};
use image::{io::Reader, ImageFormat, DynamicImage};

#[derive(Debug)]
enum ImageDataErr {
    DiffFormats
}

fn main() -> Result<(), ImageDataErr> {
    let args = Args::new();
    let (img_1, img1_format) = find_img_from_path(args.image_1);
    let (img_2, img2_format) = find_img_from_path(args.image_2);

    if img1_format != img2_format {
        return Err(ImageDataErr::DiffFormats);
    }
    Ok(())
    // println!("Hello, world! {:?}", args);
}

fn find_img_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let img_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = img_reader.format().unwrap();
    let image: DynamicImage = img_reader.decode().unwrap();
    (image, image_format)
}