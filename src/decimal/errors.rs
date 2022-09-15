use thiserror::Error;

/// Error codes related to [Decimal].
#[derive(Error, Debug)]
pub enum ErrorCode {
    #[error("Unable to parse input")]
    ParseError,
    #[error("Unable to parse empty input")]
    ParseErrorEmpty,
    #[error("Unable to parse non base 10 input")]
    ParseErrorBase10,
    #[error("Scale is different")]
    DifferentScale,
    #[error("Exceeds allowable range for value")]
    ExceedsRange,
    #[error("Exceeds allowable range for precision")]
    ExceedsPrecisionRange,
    #[error("Signed decimals not supported for this function")]
    SignedDecimalsNotSupported,
}
