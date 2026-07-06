use opencv::core::*;
use opencv::imgproc;
use plotters::prelude::*;

use opencv::boxed_ref::BoxedRef;

use opencv::core::*;

fn cv_err<E: std::fmt::Display>(e: E) -> opencv::Error {
    opencv::Error::new(StsError, format!("{}", e))
}

pub fn create_composite_plot_image<MatType>(
    plot: &MatType,
    target_row: &MatType,
    source_row: &MatType,
) -> opencv::Result<Mat>
where
    MatType: MatTraitConst,
{
    let height: i32 = plot.rows() + target_row.rows() + source_row.rows();
    let width: i32 = plot.cols().max(target_row.cols()).max(source_row.cols());

    let mut result_image = Mat::new_rows_cols_with_default(
        height,
        width,
        opencv::core::CV_32FC3,
        opencv::core::Scalar::all(0.0),
    )?;

    let mut current_row = 0;

    // control scope to ensure result_image is properly released
    // before returned.
    {
        let mut copy_into_result = |src: &MatType, y_offset: i32| -> opencv::Result<()> {
            let rect = opencv::core::Rect::new(0, y_offset, src.cols(), src.rows());

            let mut roi = result_image.roi_mut(rect)?;
            src.copy_to(&mut roi)?;

            Ok(())
        };

        copy_into_result(source_row, current_row)?;
        current_row += plot.rows();

        copy_into_result(target_row, current_row)?;
        current_row += target_row.rows();

        copy_into_result(plot, current_row)?;
    }

    Ok(result_image)
}

pub fn create_plot(values_: &Vec<f64>) -> opencv::Result<Mat> {
    let width: u32 = values_.len() as u32 + 200;
    let height: u32 = 400;

    let mut buffer = vec![255u8; (width * height * 3) as usize];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();

        root.fill(&WHITE).map_err(cv_err)?;

        let min_value = values_.iter().copied().fold(f64::INFINITY, f64::min);

        let max_value = values_.iter().copied().fold(f64::NEG_INFINITY, f64::max);

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
