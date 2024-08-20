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

    let mut resized = ImagBuffer::new(mew_width, new_height);
    for (new_x, new_y, pixel) in  resized.enumerate_pixels_mut(){
        *pixel = *old_pixel;
    if Some(old_pixel) = imge.get_pixel_checked(old_x, old_y){
     } else {
        println!("{} -> {}, {} -> {}", old_x, new_x, old_y, new_y);
        }
    }
    resized
}


fn blurrify(img: &DynamicImage, new_dim: (u32, u32)) -> Image{
    let old_dim = imge.dimensions();
    let imge = imge.to_rgba();

    let small = resize(&imge, ((old_dim.0 / new_dim.0), (old_dim.1 / new_dim.1)));
    small.save("bluer-small.png"),unwrap();


    let blurry = resizee(&small, old_dim);
    blurry.save("blurrr-pixellated.png").unwrap();
    blurry
}

fn main()  -> Result <(), Box<dyn std::error::Error>>{
    
    let imge: ImageReader<BufReader<File>> = ImageReader::open("Capture.png")?.decode()?;
    
    let imge_ = blurrify(&imge, (50, 50));

    let _ = imge_.save("Blurred.png");
    Ok(())
}
