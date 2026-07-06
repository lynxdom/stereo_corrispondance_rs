use opencv::imgproc::GrabCutClasses;

use opencv::imgproc::*;

use opencv::core::Rect;
use opencv::core::Scalar;

use opencv::highgui; // GUI module for image display
use opencv::imgcodecs;
use opencv::prelude::*; // Import core traits // Module for reading/writing images

use std::env;

mod stereo_functions;
mod utilities;
mod visualization;

use crate::stereo_functions::basic_disparity_functions::*;
use crate::utilities::image_utilities::*;
use crate::visualization::ssd_visualization::create_plot;

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
            let gray_image1 = gray_image(&image1)?;
            let gray_image2 = gray_image(&image2)?;

            let usable_area = find_useable_area(&gray_image1, &gray_image2)?;

            let mut gray_clone1 = gray_image1.clone();
            let mut gray_clone2 = gray_image2.clone();

            rectangle(
                &mut gray_clone1,
                Rect::new(usable_area.0, usable_area.1, usable_area.2 - usable_area.0, usable_area.3 - usable_area.1),
                Scalar::new(100.0, 0.0, 0.0, 0.0),
                2,
                LINE_8,
                0,
            )?;

            rectangle(
                &mut gray_clone2,
                Rect::new(usable_area.0, usable_area.1, usable_area.2, usable_area.3),
                Scalar::new(100.0, 0.0, 0.0, 0.0),
                2,
                LINE_8,
                0,
            )?;

            let row_score =
                get_row_patch_score(&gray_image1, &gray_image2, 50, 100, 350, (50, 1000))?;

            rectangle(
                &mut gray_clone1,
                Rect::new(300, 50, 100, 100),
                Scalar::new(255.0, 0.0, 0.0, 0.0),
                2,
                LINE_8,
                0,
            )?;

            rectangle(
                &mut gray_clone2,
                Rect::new(0, 50, 1050, 100),
                Scalar::new(255.0, 0.0, 0.0, 0.0),
                2,
                LINE_8,
                0,
            )?;

            let (best_index, best_score) = row_score
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap();

            rectangle(
                &mut gray_clone2,
                Rect::new((best_index) as i32, 50, 100, 100),
                Scalar::new(45.0, 0.0, 0.0, 0.0),
                2,
                LINE_8,
                0,
            )?;

            // Display the image in a window
            highgui::imshow("Image Window1", &gray_clone1)?;
            highgui::imshow("Image Window2", &gray_clone2)?;

            //
            let image = get_window(&gray_image1, 350, 100, 50)?;

            highgui::imshow("Image Window1sub", &image)?;

            let plot = create_plot(&row_score)?;
            highgui::imshow("SAD Plot", &plot)?;

            // Wait for a key press indefinitely
            highgui::wait_key(0)?;
        }
    }

    Ok(())
}
