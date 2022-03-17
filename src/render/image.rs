use super::super::encode::QRCode;
use image::{GrayImage, Luma};


/// Generates an `Image` of a [`QRCode`], generated from the provided `message` and `version`.
pub fn generate_image(code: QRCode) -> GrayImage {
    let (width, height) = code.dimensions();

    let mut image = GrayImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            image.put_pixel(
                x, y, code[(x, y)].as_type(
                    Luma([255]), Luma([0])
                )
            );
        }
    }

    return image;
}