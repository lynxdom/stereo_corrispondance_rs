use opencv::core::{Mat, MatTraitConst};
use opencv::core::abs;
pub trait ComparisonTrait {
    fn comparison_method<T>(    left: &T,
                                right: &T,
                            ) -> opencv::Result<f64>
                        where
                            T: MatTraitConst;

    fn evaluation_criteria( result: &[f64] ) -> Vec<f64>;
}

fn max_f64(values: &[f64]) -> f64 {
    values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
}

fn min_f64(values: &[f64]) -> f64 {
    values
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
}

pub struct SSDComparison;

impl ComparisonTrait for SSDComparison{

    fn comparison_method<T>(
        left: &T,
        right: &T,
    ) -> opencv::Result<f64>
    where
        T: MatTraitConst,
    {
        assert_eq!(left.cols(), right.cols());
        assert_eq!(left.rows(), right.rows());

        let mut score: f64 = 0.0;

        for row in 0..left.rows() {
            for col in 0..left.cols() {
                let left_value = *left.at_2d::<u8>(row, col)? as f64;
                let right_value = *right.at_2d::<u8>(row, col)? as f64;

                let diff = left_value - right_value;
                score += diff * diff;
            }
        }

        Ok(score)
    }

    fn evaluation_criteria( comparison_results: &[f64] ) -> Vec<f64> {

        let result_count = comparison_results.len();
        let mut result_array: Vec<f64> = vec![0.0; result_count];

        let max_score = max_f64( comparison_results );
        let min_score = min_f64( comparison_results );
        
        let mut index = 0;
        for value in comparison_results {
            result_array[index] = 1.0 - ((*value - min_score) / (max_score - min_score));
            index += 1;
        }

        result_array
    }

}


pub struct SADComparison;

impl ComparisonTrait for SADComparison{

    fn comparison_method<T>(
        left: &T,
        right: &T,
    ) -> opencv::Result<f64>
    where
        T: MatTraitConst,
    {
        assert_eq!(left.cols(), right.cols());
        assert_eq!(left.rows(), right.rows());

        let mut score: f64 = 0.0;

        for row in 0..left.rows() {
            for col in 0..left.cols() {
                let left_value = *left.at_2d::<u8>(row, col)? as f64;
                let right_value = *right.at_2d::<u8>(row, col)? as f64;

                let diff = left_value - right_value;
                score += diff.abs();
            }
        }

        Ok(score)
    }

    fn evaluation_criteria( comparison_results: &[f64] ) -> Vec<f64> {

        let result_count = comparison_results.len();
        let mut result_array: Vec<f64> = vec![0.0; result_count];

        let max_score = max_f64( comparison_results );
        let min_score = min_f64( comparison_results );
        
        let mut index = 0;
        for value in comparison_results {
            result_array[index] = 1.0 - ((*value - min_score) / (max_score - min_score));
            index += 1;
        }

        result_array
    }

}