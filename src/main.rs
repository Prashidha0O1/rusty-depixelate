#![allow(dead_code)]
use image::ImageReader;
use std::io::BufReader;
use std::fs::File;
use image::io::Reader; // it is sort of conflicting with standard io read "std::io::Read"
use image::ImageError;
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};


type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn resizee(imge: &Image, new_dim: (u32, u32)) -> Image {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = new_dim;

    let resized = ImagBuffer::new(mew_width, new_height);
    for (new_x, new_y, pixel) in  resized.enumerate_pixels_mut(){
        *pixel = *old_pixel;
    if Some(old_pixel) = imge.get_pixel_checked(old_x, old_y){
     } else {
        println!("{} -> {}, {} -> {}", old_x, new_x, old_y, new_y);
        }
    }
    resized
}


fn main()  -> Result <(), Box<dyn std::error::Error>>{
    
    let imge: ImageReader<BufReader<File>> = ImageReader::open("Capture.png")?.decode()?;
    let (width, height) = imge.dimensions();

    println!("({}, {})", width, height);
    Ok(())
}
