use thiserror::Error;

/// Crate-specific error enum.
#[derive(Debug, Error)]
pub enum Error {
    /// [std::io::Error]
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// [std::num::ParseFloatError]
    #[error(transparent)]
    ParseFloat(#[from] std::num::ParseFloatError),

    /// The time unit code is invalid.
    #[error("The pof time unit code is invalid: {0}")]
    PofTimeUnit(u8),

    /// The time info code is invalid.
    #[error("The pof time info code is invalid: {0}")]
    PofTimeInfo(u8),

    /// Error returned when trying to extrapolate with only one point in the source.
    #[error("Cannot interpolate in a source with only one point")]
    OnePoint,

    /// The time value is below the minimum time of the source.
    #[error("Time value is below minimum of the source: {0}")]
    TimeBelowMinimum(f64),

    /// The time value is above the maximum time of the source.
    #[error("Time value is above the maximum of the source: {0}")]
    TimeAboveMaximum(f64),
}
