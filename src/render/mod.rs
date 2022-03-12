use image::{GrayImage, Luma};
use crate::encode;


/// Encodes the `message` as a `QRCode` than transforms it into an image.
pub fn as_image(message: &[u8], version: u32) -> GrayImage {
    let qr_data = encode::generate(message, version);
    let (width, height) = qr_data.dimensions();

    let mut image = GrayImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let luma = qr_data[(x, y)].as_type(
                Luma([ 255 ]), Luma([ 0 ])
            );

            image.put_pixel(x, y, luma);
        }
    }

    return image;
}