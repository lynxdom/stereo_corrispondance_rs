use opencv::core::{Mat, MatTraitConst};

pub fn ssd_comparison( left_ : &Mat,
                       right_ : &Mat ) -> opencv::Result<f64> {

    assert_eq!(left_.cols(), right_.cols());
    assert_eq!(left_.rows(), right_.rows());

    let mut score : f64 = 0.0;

    for x in 0 .. left_.rows() {
        for y in 0 .. left_.cols() {
            let left_value = *left_.at_2d::<u8>(y, x)? as f64;
            let right_value = *right_.at_2d::<u8>(y, x)? as f64;

            let diff = left_value - right_value;
            score += diff * diff;
        }
    }

    Ok(score)
}