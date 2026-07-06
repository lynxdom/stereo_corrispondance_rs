use opencv::boxed_ref::BoxedRef;
use opencv::core::*;
use opencv::imgproc;

pub fn get_window<'img>(
    image_: &'img Mat,
    x_: u32,
    y_: u32,
    r_: u32,
) -> opencv::Result<BoxedRef<'img, Mat>> {
    let left = (x_ - r_) as i32;
    let top = (y_ - r_) as i32;
    let size = (2 * r_ + 1) as i32;

    let rect = Rect::new(left, top, size, size);

    Mat::roi(image_, rect)
}

pub fn gray_image(image_: &Mat) -> Result<Mat, opencv::Error> {
    let mut gray_image = Mat::default();

    imgproc::cvt_color(
        &image_,
        &mut gray_image,
        imgproc::COLOR_BGR2GRAY,
        0,
        AlgorithmHint::ALGO_HINT_DEFAULT,
    )?;

    Ok(gray_image)
}
