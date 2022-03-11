use image::GrayImage;


pub fn generate(message: String, version: u32) -> GrayImage {
    let size: u32 = 21 + (4 * (version - 1));
    let image = GrayImage::new(size, size);

    return image;
}