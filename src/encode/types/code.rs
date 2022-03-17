use super::{QRBit, ErrorCorrectionLevel};
use std::ops::{Index, IndexMut};
use std::vec::Vec;


/// Data representation of the final state during encoding,
/// [`QRCode`] represents a 2D Square Matrix of pixels, that can be parsed into an output file.
pub struct QRCode {
    /// Version of the [`QRCode`], determines the values of the `dimensions` field.
    version: u32,

    /// Coresponds to the `version` field, size of the [`QRCode`] in pixels.
    dimensions: (u32, u32),

    /// Level of data bytes than can be restored if the code is damaged.
    ecl: ErrorCorrectionLevel,

    /// Represents the `pixels` within the [`QRCode`],
    /// can be directly accessed through square-bracket notation on the [`QRCode`] with a position.
    pixels: Vec<QRBit>,
}


impl QRCode {
    /// Returns the estimated index of the [`QRBit`] at the position within the `pixels` field.
    /// 
    /// # Example
    /// ```rust
    /// let positional_index = QRCode::position_as_index(1, 2, 21);
    /// assert_eq!(positional_index, 23);
    /// ```
    #[inline]
    const fn position_as_index(x: u32, y: u32, max_y: u32) -> usize {
        return (y + (x * max_y)) as usize;
    }
}


impl QRCode {
    /// Constructs a new [`QRCode`], `version` determines the `dimensions` and `pixels` fields.
    /// 
    /// # Example
    /// ```rust
    /// let qr_code = QRCode::new(1);
    /// 
    /// assert_eq!(qr_code.version(), 1);
    /// assert_eq!(qr_code.dimensions(), (21, 21));
    /// ```
    #[inline]
    pub fn new(version: u32) -> Self {
        let version_dimensions = 21 + (4 * (version - 1));
        let dimensions = (version_dimensions, version_dimensions);

        let mut pixels = Vec::with_capacity((dimensions.0 * dimensions.1) as _);

        for _ in 0 .. dimensions.0 {
            for _ in 0 .. dimensions.1 {
                pixels.push(QRBit::Dark);
            }
        }
        
        return Self {
            version,
            dimensions,
            ecl: ErrorCorrectionLevel::Medium,
            pixels,
        }
    }

    /// Returns a copy of the `version` field within the [`QRCode`].
    /// 
    /// # Example
    /// ```rust
    /// let qr_code = QRCode::new(3);
    /// assert_eq!(qr_code.version(), 3);
    /// ```
    #[inline]
    pub const fn version(&self) -> u32 { self.version }

    /// Returns a copy of the `dimensions` field within the [`QRCode`].
    /// 
    /// # Example
    /// ```rust
    /// let qr_code = QRCode::new(1);
    /// assert_eq!(qr_code.dimensions(), (21, 21));
    /// ```
    #[inline]
    pub const fn dimensions(&self) -> (u32, u32) { self.dimensions }

    /// Places a light [`QRBit`] at the specified position within the `pixels` field.
    /// 
    /// # Example
    /// ```rust
    /// let mut qr_code = QRCode::new(1);
    /// qr_code.place_light(0, 0);
    /// 
    /// assert_eq!(qr_code[(0, 0)], QRBit::Light);
    /// ```
    #[inline]
    pub fn place_light(&mut self, x: u32, y: u32) { self[(x, y)] = QRBit::Light; }

    /// Places a dark [`QRBit`] at the specified position within the `pixels` field.
    /// 
    /// # Example
    /// ```rust
    /// let mut qr_code = QRCode::new(1);
    /// 
    /// // The QRCode is entirely comprised of dark pixels at construction.
    /// qr_code.place_light(0, 0);
    /// qr_code.place_dark(0, 0);
    /// 
    /// assert_eq!(qr_code[(0, 0)], QRBit::Dark);
    /// ```
    #[inline]
    pub fn place_dark(&mut self, x: u32, y: u32) { self[(x, y)] = QRBit::Dark; }
}


impl Index<(u32, u32)> for QRCode {
    type Output = QRBit;
    
    #[inline]
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        return &self.pixels[Self::position_as_index(x, y, self.dimensions.1)];
    }
}


impl IndexMut<(u32, u32)> for QRCode {
    #[inline]
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        return &mut self.pixels[Self::position_as_index(x, y, self.dimensions.1)];
    }
}