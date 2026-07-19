use opencv::core::*;
use opencv::prelude::*;

use crate::utilities::comparison_utilities::{ComparisonTrait, SADComparison, SSDComparison};
use crate::utilities::image_utilities::get_window;

fn generate_disparity_map(_left_: &Mat, _right_: &Mat) {}

pub fn find_useable_area(
    image1_: &Mat,
    image2_: &Mat,
) -> Result<(i32, i32, i32, i32), opencv::Error> {
    assert!((image1_.rows() == image2_.rows()) && (image1_.cols() == image2_.cols()));

    let rows = image1_.rows();
    let cols: i32 = image1_.cols();

    let mut results1 = vec![0.0; rows as usize];
    let mut results2 = vec![0.0; rows as usize];

    for row in 0..rows {
        let row_data_img1 = image1_.row(row)?;
        let row_data_img2 = image2_.row(row)?;
        let sum = row_data_img1
            .iter::<u8>()?
            .filter(|(_, value)| *value != 0)
            .count();
        results1[row as usize] = sum as f64 / cols as f64;

        let sum = row_data_img2
            .iter::<u8>()?
            .filter(|(_, value)| *value != 0)
            .count();
        results2[row as usize] = sum as f64 / cols as f64;
    }

    let top = results1
        .iter()
        .zip(results2.iter())
        .position(|(a, b)| *a >= 0.80 && *b >= 0.80)
        .unwrap_or(0) as i32;

    let bottom = results1
        .iter()
        .zip(results2.iter())
        .rposition(|(a, b)| *a >= 0.80 && *b >= 0.80)
        .unwrap_or(rows as usize - 1) as i32;

    let mut results1 = vec![0.0; cols as usize];
    let mut results2 = vec![0.0; cols as usize];

    for col in 0..cols {
        let row_data_img1 = image1_.col(col)?;
        let row_data_img2 = image2_.col(col)?;
        let sum = row_data_img1
            .iter::<u8>()?
            .filter(|(_, value)| *value != 0)
            .count();
        results1[col as usize] = sum as f64 / rows as f64;

        let sum = row_data_img2
            .iter::<u8>()?
            .filter(|(_, value)| *value != 0)
            .count();
        results2[col as usize] = sum as f64 / rows as f64;
    }

    let left = results1
        .iter()
        .zip(results2.iter())
        .position(|(a, b)| *a >= 0.80 && *b >= 0.80)
        .unwrap_or(0) as i32;

    let right = results1
        .iter()
        .zip(results2.iter())
        .rposition(|(a, b)| *a >= 0.80 && *b >= 0.80)
        .unwrap_or(cols as usize - 1) as i32;

    Ok( ( left, top, right, bottom ) )
}

pub fn get_row_patch_score(
    source_: &Mat,
    target_: &Mat,
    radius_: u32,
    row_: u32,
    col_source_: u32,
    col_range_: (u32, u32),
) -> Result<Vec<f64>, opencv::Error> {
    let length = col_range_.1 - col_range_.0;
    let mut score = vec![0.0; length as usize];

    let source_patch = get_window(&source_, col_source_, row_, radius_)?;

    let col_start = col_range_.0;

    for x in 0..length {
        let target_patch = get_window(&target_, col_start + x, row_, radius_)?;

        score[x as usize] = SADComparison::comparison_method(&source_patch, &target_patch)?;
    }

    Ok(score)
}
