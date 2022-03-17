/// Depending on the level, this allows for more data to be recovered if the [`QRCode`] is damaged.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum ErrorCorrectionLevel {
    /// 7% of data bytes can be restored using this level.
    Low = 0,

    /// 15% of data bytes can be restored using this level.
    Medium = 1,

    /// 25% of data bytes can be restored using this level.
    Quartile = 2,

    /// 30% of data bytes can be restored using this level.
    High = 3,
}


impl ErrorCorrectionLevel {
    /// Returns the percent of data bytes than can be restored with the [`ErrorCorrectionLevel`].
    /// 
    /// # Example
    /// ```rust
    /// let percent = ErrorCorrectionLevel::High.as_percent();
    /// assert_eq!(percent, 30);
    /// ```
    #[inline]
    pub fn as_percent(self) -> u8 {
        return match self {
            ErrorCorrectionLevel::Low => 7,
            ErrorCorrectionLevel::Medium => 15,
            ErrorCorrectionLevel::Quartile => 25,
            ErrorCorrectionLevel::High => 30,
        };
    }
}