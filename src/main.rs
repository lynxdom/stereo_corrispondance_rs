use opencv::core;
use opencv::imgproc;
use opencv::prelude::*; // Import core traits
use opencv::highgui;    // GUI module for image display
use opencv::imgcodecs;  // Module for reading/writing images


use std::env;

fn gray_image( image : Mat ) -> Result<Mat, opencv::Error> {
    let mut gray_image = Mat::default();

    imgproc::cvt_color( 
        &image, 
        &mut gray_image, 
        imgproc::COLOR_BGR2GRAY, 
        0,
        core::AlgorithmHint::ALGO_HINT_DEFAULT,
    )?;

    Ok( gray_image )
}

fn main() -> opencv::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.iter().count() < 3 {
        println!("usage : command [image1] [image2]");
    } else {
        let image1_path = args[1].as_str();
        let image2_path = args[2].as_str();

        // Load an image from file
        let image1 = imgcodecs::imread(image1_path, imgcodecs::IMREAD_COLOR)?;
        let image2 = imgcodecs::imread(image2_path, imgcodecs::IMREAD_COLOR)?;

        // Check if the image was successfully loaded
        if image1.empty() || image2.empty() {
            println!("Error: Could not load images.");
        } else {

            let gray_image1 = gray_image( image1 )?;
            let gray_image2 = gray_image( image2 )?;

            // Display the image in a window
            highgui::imshow("Image Window1", &gray_image1)?;
            highgui::imshow("Image Window2", &gray_image2)?;
            
            // Wait for a key press indefinitely
            highgui::wait_key(0)?;
        }
    }

    Ok(())
}
