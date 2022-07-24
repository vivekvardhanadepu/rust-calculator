mod args;
use args::Args;
use std::{fs::File, io::BufReader};
use image::{io::Reader, ImageFormat, DynamicImage, imageops::FilterType::Triangle, GenericImageView};

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
    let (img_1, img_2) = standardize_size(img_1, img_2);
    Ok(())
    // println!("Hello, world! {:?}", args);
}

fn find_img_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let img_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = img_reader.format().unwrap();
    let image: DynamicImage = img_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dimension(dim_1: (u32, u32), dim_2: (u32, u32),) -> (u32, u32) {
    let pix_1 = dim_1.0 * dim_1.1;
    let pix_2 = dim_2.0 * dim_2.1;
    return if pix_1 > pix_2 { dim_1 } else { dim_2 };
}

fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage){
    let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());
    println!("width: {}, height: {}\n", width, height);

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}