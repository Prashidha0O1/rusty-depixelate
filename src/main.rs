// Allow dead code warnings, as some parts of the code might not be used yet.
#![allow(dead_code)]

// Import necessary modules from the `image` crate.
use image::ImageReader;
 // It doesn't conflict here as we use `ImageReader` instead of `Reader`.
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba};

// Define a type alias for an image with RGBA color and 8-bit per channel.
type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

// Function to resize an image to new dimensions.
fn resizee(imge: &Image, new_dim: (u32, u32)) -> Image {
    let (old_width, old_height) = imge.dimensions(); // Get dimensions of the original image.
    let (new_width, new_height) = new_dim; // Unpack new dimensions.

    // Create a new image buffer with the new dimensions.
    let mut resized = ImageBuffer::new(new_width, new_height);
    
    // Iterate over the pixels in the resized image buffer.
    for (new_x, new_y, pixel) in resized.enumerate_pixels_mut() {
        // Calculate the corresponding old pixel position.
        let old_x = (new_x as f32 / new_width as f32 * old_width as f32) as u32;
        let old_y = (new_y as f32 / new_height as f32 * old_height as f32) as u32;

        // Get the pixel from the old image and assign it to the new position.
        if let Some(old_pixel) = imge.get_pixel_checked(old_x, old_y) {
            *pixel = *old_pixel;
        } else {
            println!("{} -> {}, {} -> {}", old_x, new_x, old_y, new_y);
        }
    }

    resized // Return the resized image.
}

// Function to apply a blurring effect by resizing the image and restoring it.
fn blurrify(imge: &DynamicImage, new_dim: (u32, u32)) -> Image {
    let old_dim = imge.dimensions(); // Get original image dimensions.
    let imge = imge.to_rgba8(); // Convert the dynamic image to RGBA format.

    // First resize to smaller dimensions to create a blur effect.
    let small = resizee(&imge, (old_dim.0 / new_dim.0, old_dim.1 / new_dim.1));
    small.save("blur-small.png").unwrap(); // Save the small image.

    // Resize the small image back to the original dimensions.
    let blurry = resizee(&small, old_dim);
    blurry.save("blurred-pixellated.png").unwrap(); // Save the blurred image.
    blurry // Return the blurred image.
}

// Main function to load, process, and save the image.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the image file and decode it into a DynamicImage.
    let imge = ImageReader::open("Wyoming.PNG")?.decode()?;

    // Apply the blurrify function with a given dimension.
    let imge_ = blurrify(&imge, (60, 60));

    // Save the final blurred image.
    imge_.save("Blurred.png")?;
    Ok(()) // Return OK if everything was successful.
}
