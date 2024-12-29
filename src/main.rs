mod image_generator;
mod vector;
mod point;
mod ray;
mod object;
mod material;

use image_generator::ImageGenerator;
use vector::Vector;
use point::Point;
use rfd::FileDialog;

fn task1() {
    let width = 800;
    let height = 600;
    let output_file = "black_image.png";

    let generator = ImageGenerator::new(width, height);
    if let Err(e) = generator.generate_black_image(output_file) {
        eprintln!("Error generating black image: {}", e);
    } else {
        println!("Black image generated successfully: {}", output_file);
    }
}

fn main() {
    // task1();

    let v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(4.0, 5.0, 6.0);
    let result = v1 + v2;
    println!("{:?}", result); // Output: Vector { x: 5.0, y: 7.0, z: 9.0 }

    let p1 = Point::new(0.0, 0.0, 0.0);
    let p2 = Point::new(1.0, 1.0, 1.0);
    let result2 = p1 + v1;
    println!("{:?}", result2);


    let file = FileDialog::new()
        .add_filter("XML files", &["xml"])
        .pick_file();

    match file {
        Some(path) => println!("Selected file: {:?}", path),
        None => println!("No file selected"),
    }
}
