use std::ops::{Index, IndexMut};
use std::vec::Vec;

use super::QRBit;


/// Struct for describing a `QRCode`.
pub struct QRData {
    /// Version of the `QRCode`, specifies the `dimensions`.
    version: u32,

    /// Dimensions of the `QRCode`, derived from the `version`.
    dimensions: (u32, u32),

    /// Pixels of the `QRCode` described by [`QRBit`]s, size determined by the `dimensions` field.
    data: Vec<QRBit>,
}


impl QRData {
    /// Returns the estimated index of the [`QRBit`] at the `position` within the `data` field.
    #[inline]
    const fn position_as_idx(x: u32, y: u32, max_y: u32) -> usize {
        return (y + (x * max_y)) as usize;
    }

    /// Returns a [`bool`] determining if a position is out of bounds.
    #[inline]
    fn is_out_of_bounds(&self, x: u32, y: u32) -> bool {
        return Self::position_as_idx(x, y, self.dimensions().1) >= self.data.len();
    }

    /// Returns a [`bool`] determining if a position is within bounds.
    #[inline]
    fn is_in_bounds(&self, x: u32, y: u32) -> bool {
        return Self::position_as_idx(x, y, self.dimensions().1) < self.data.len();
    }
}


impl QRData {
    /// Constructs a new [`QRData`], derives all other fields based on the passed through `version`.
    /// The [`QRData`] `version` represents the `dimensions` of the [`QRData`],
    /// as an example `version` 1 will be 21x21 in size.
    /// 
    /// # Example
    /// ```rust
    /// let qr_data = QRData::new(2);
    /// assert_eq!(qr_data.dimensions(), (25, 25));
    /// ```
    #[inline]
    pub fn new(version: u32) -> Self {
        // NOTE: This is checked when receiving the `-v` flag in the main function.
        debug_assert!(version <= 40 && version >= 1, "Invalid Version");

        let size = 21 + (4 * (version - 1));
        let (x, y) = (size, size);

        let mut data = Vec::with_capacity((x * y) as _);

        for _ in 0 .. x {
            for _ in 0 .. y {
                data.push(QRBit::Dark);
            }
        }

        return Self {
            version,
            dimensions: (x, y),
            data,
        };
    }

    /// Clears the `data` field, resetting all values to a `Dark` [`QRBit`].
    /// 
    /// # Example
    /// ```rust
    /// let mut qr_data = QRData::new(1);
    /// qr_data.place_light(0, 0);
    /// qr_data.clear();
    /// 
    /// assert_eq!(qr_data[(0, 0)], QRBit::Dark);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.data = self.data.iter()
            .map(|_| QRBit::Dark)
            .collect();
    }

    /// Returns a clone of the [`QRData`]'s `version` field.
    /// 
    /// # Example
    /// ```rust
    /// let qr_data = QRData::new(3);
    /// assert_eq!(qr_data.version(), 3);
    /// ```
    #[inline]
    pub const fn version(&self) -> u32 {
        return self.version;
    }

    /// Returns a clone of the [`QRData`]'s `dimensions` field.
    /// 
    /// # Example
    /// ```rust
    /// let qr_data = QRData::new(3);
    /// assert_eq!(qr_data.dimensions(), (29, 29));
    /// ```
    #[inline]
    pub const fn dimensions(&self) -> (u32, u32) {
        return self.dimensions;
    }

    /// Places a `Light` [`QRBit`] at the specified position.
    /// 
    /// # Example
    /// ```rust
    /// let mut qr_data = QRData::new(1);
    /// qr_data.place_light(0, 0);
    /// 
    /// assert_eq!(qr_data[(0, 0)], QRBit::Light);
    /// ```
    #[inline]
    pub fn place_light(&mut self, x: u32, y: u32) {
        return self[(x, y)] = QRBit::Light;
    }

    /// Places a `Dark` [`QRBit`] at the specified position.
    /// 
    /// # Example
    /// ```rust
    /// let mut qr_data = QRData::new(1);
    /// qr_data.place_light(0, 0);
    /// qr_data.place_dark(0, 0);
    /// 
    /// assert_eq!(qr_data[(0, 0)], QRBit::Dark);
    /// ```
    #[inline]
    pub fn place_dark(&mut self, x: u32, y: u32) {
        return self[(x, y)] = QRBit::Dark;
    }
}


impl Index<(u32, u32)> for QRData {
    type Output = QRBit;

    fn index(&self, position: (u32, u32)) -> &Self::Output {
        let (x, y) = position;
        return &self.data[Self::position_as_idx(x, y, self.dimensions.1)];
    }
}


impl IndexMut<(u32, u32)> for QRData {
    fn index_mut(&mut self, position: (u32, u32)) -> &mut Self::Output {
        let (x, y) = position;
        return &mut self.data[Self::position_as_idx(x, y, self.dimensions.1)];
    }
}