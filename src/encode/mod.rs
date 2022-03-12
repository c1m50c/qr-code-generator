pub mod types;

use types::QRData;


/// Creates the positional anchors you see at the top right, left, and bottom right of a `QRCode`.
fn create_positional_anchors(data: &mut QRData) {
    let (width, height) = data.dimensions();

    // Places a light [`QRBit`] at the specified location.
    let mut place = |x: u32, y: u32| { data.place_light(x, y); };

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


pub fn generate(message: &[u8], version: u32) -> QRData {
    let mut qr_data = QRData::new(version);
    create_positional_anchors(&mut qr_data);

    return qr_data;
}