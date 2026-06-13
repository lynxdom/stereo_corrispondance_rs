use plotters::prelude::*;
use opencv::core::*;
use opencv::imgproc;

use opencv::core::*;

fn cv_err<E: std::fmt::Display>(e: E) -> opencv::Error {
    opencv::Error::new(StsError, format!("{}", e))
}

pub fn create_plot(values_: &Vec<f64>) -> opencv::Result<Mat> {
    let width: u32 = 900;
    let height: u32 = 400;

    let mut buffer = vec![255u8; (width * height * 3) as usize];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height))
            .into_drawing_area();

        root.fill(&WHITE).map_err(cv_err)?;

        let min_value = values_
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);

        let max_value = values_
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        let y_min = min_value;
        let y_max = if min_value == max_value {
            max_value + 1.0
        } else {
            max_value
        };

        let mut chart = ChartBuilder::on(&root)
            .caption("SSD scores", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..values_.len(), y_min..y_max)
            .map_err(cv_err)?;

        chart
            .configure_mesh()
            .x_desc("Candidate position")
            .y_desc("SSD score")
            .draw()
            .map_err(cv_err)?;

        chart
            .draw_series(LineSeries::new(
                values_.iter().enumerate().map(|(i, value)| (i, *value)),
                &BLUE,
            ))
            .map_err(cv_err)?;

        root.present().map_err(cv_err)?;
    }

    let rgb_flat = Mat::from_slice(&buffer)?;
    let rgb_image = rgb_flat.reshape(3, height as i32)?;

    let mut bgr_image = Mat::default();

    imgproc::cvt_color(
        &rgb_image,
        &mut bgr_image,
        imgproc::COLOR_RGB2BGR,
        0,
        AlgorithmHint::ALGO_HINT_DEFAULT,
    )?;

    Ok(bgr_image)
}