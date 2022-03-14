pub mod types;

use types::QRData;


/// Creates the positional anchors you see at the top right, left, and bottom right of a `QRCode`.
fn create_positional_anchors(data: &mut QRData) {
    let (width, height) = data.dimensions();

    // Places a light [`QRBit`] at the specified location.
    let mut place = |x: u32, y: u32| { data.place_light(x, y); };

    // Top Left Indicator
    (1 .. 6).for_each(|i| place(i, 1));
    (1 .. 6).for_each(|i| place(i, 5));
    (0 .. 8).for_each(|i| place(i, 7));
    (2 .. 6).for_each(|i| place(1, i));
    (2 .. 6).for_each(|i| place(5, i));
    (0 .. 7).for_each(|i| place(7, i));

    // Top Right Indicator
    (width - 6 .. width - 1).for_each(|i| place(i, 1));
    (width - 6 .. width - 1).for_each(|i | place(i, 5));
    (width - 8 .. width).for_each(|i| place(i, 7));
    (2 .. 6).for_each(|i| place(width - 6, i));
    (2 .. 6).for_each(|i| place(width - 2, i));
    (0 .. 8).for_each(|i| place(width - 8, i));

    // Bottom Left Indicator
    (1 .. 6).for_each(|i| place(i, height - 2));
    (1 .. 6).for_each(|i| place(i, height - 6));
    (0 .. 8).for_each(|i| place(i, height - 8));
    (height - 6 .. height - 2).for_each(|i| place(1, i));
    (height - 6 .. height - 2).for_each(|i| place(5, i));
    (height - 7  .. height).for_each(|i| place(7, i));
}


pub fn generate(message: &[u8], version: u32) -> QRData {
    let mut qr_data = QRData::new(version);
    create_positional_anchors(&mut qr_data);

    return qr_data;
}