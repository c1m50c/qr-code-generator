pub use ecl::ErrorCorrectionLevel;
pub use code::QRCode;
pub mod code;
pub mod ecl;


/// Enums representing a Pixel's state within a `QRCode`.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QRBit {
    /// Represents a `Light` Pixel within a `QRCode`.
    Light,

    /// Represents a `Dark` Pixel within a `QRCode`.
    Dark,
}


impl QRBit {
    /// Depending on the state of the [`QRBit`] returns a different variant of the specified type.
    /// 
    /// # Example
    /// ```rust
    /// let bit = QRBit::Light;
    /// let number = bit.as_type(1, 0);
    /// assert_eq!(number, 1);
    /// ```
    #[inline]
    pub fn as_type<T>(self, light: T, dark: T) -> T {
        return match self {
            QRBit::Light => light,
            QRBit::Dark => dark,
        };
    }
}