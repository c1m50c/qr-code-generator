use image::{GrayImage, Luma};


fn create_positional_indicators(img: &mut GrayImage) {
    let (width, height) = img.dimensions();

    // Places a white pixel at the specified location.
    let mut place = |x: u32, y: u32| {
        img.put_pixel(x, y, Luma([ 255 ]));
    };

    // Top Left Indicator
    for i in 1 .. 6 { place(i, 1); } for i in 2 .. 6 { place(1, i); }
    for i in 1 .. 6 { place(i, 5); } for i in 2 .. 6 { place(5, i); }
    for i in 0 .. 8 { place(i, 7); } for i in 0 .. 7 { place(7, i); }

    // Top Right Indicator
    for i in width - 6 .. width - 1 { place(i, 1); } for i in 2 .. 6 { place(width - 6, i); }
    for i in width - 6 .. width - 1 { place(i, 5); } for i in 2 .. 6 { place(width - 2, i); }
    for i in width - 8 .. width { place(i, 7); } for i in 0 .. 8 { place(width - 8, i); }

    // Bottom Left Indicator
    for i in 1 .. 6 { place(i, height - 2); } for i in height - 6 .. height - 2 { place(1, i); }
    for i in 1 .. 6 { place(i, height - 6); } for i in height - 6 .. height - 2 { place(5, i); }
    for i in 0 .. 8 { place(i, height - 8); } for i in height - 7  .. height { place(7, i); }
}


pub fn generate(message: String, version: u32) -> GrayImage {
    let size: u32 = 21 + (4 * (version - 1));

    let mut image = GrayImage::new(size, size);
    create_positional_indicators(&mut image);

    return image;
}