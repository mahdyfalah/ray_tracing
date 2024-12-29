use image::{ImageBuffer, Rgb};

pub struct ImageGenerator {
    width: u32,
    height: u32,
}

impl ImageGenerator {
    /// Creates a new ImageGenerator instance.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Generates a black image and saves it as a PNG file.
    pub fn generate_black_image(
        &self,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a black image buffer with `u8` pixel values
        let black_pixel = [0u8, 0u8, 0u8]; // RGB values for black
        let image = ImageBuffer::from_fn(self.width, self.height, |_x, _y| Rgb(black_pixel));

        // Save the image
        image.save(output_path)?;
        Ok(())
    }
}
