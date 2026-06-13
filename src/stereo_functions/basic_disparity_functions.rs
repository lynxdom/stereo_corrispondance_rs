use opencv::core::{Mat, MatTraitConst};

use crate::utilities::image_utilities::get_window;
use crate::utilities::comparison_utilities::ssd_comparison;

fn generate_disparity_map( _left_ : &Mat, 
                           _right_ : &Mat ) {

    
}

pub fn get_row_patch_score( source_ : &Mat,
                        target_ : &Mat,
                        radius_ : u32,
                        row_ : u32,
                        col_source_ : u32,
                        col_range_ : ( u32, u32) ) -> Result< Vec<f64>, opencv::Error > {
    let length = col_range_.1 - col_range_.0;
    let mut score = vec![0.0; length as usize];

    let source_patch = 
        get_window(&source_, 
        col_source_, 
        row_, 
        radius_)?;

    let col_start = col_range_.0;

    for x in 0 .. length {

        let target_patch = 
            get_window(&target_,
                       col_start + x,
                       row_, 
                       radius_)?;

        score[ x as usize ] = ssd_comparison( &source_patch,
                                              &target_patch )?;
        
    }

    Ok( score )
}