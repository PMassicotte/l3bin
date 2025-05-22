use std::fmt;

#[derive(Debug)]
pub enum IsinError {
    InvalidBinRange { max_bin: usize },
    InvalidLatitude { min: f64, max: f64 },
    InvalidLongitude { min: f64, max: f64 },
}

impl fmt::Display for IsinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsinError::InvalidBinRange { max_bin } => {
                write!(
                    f,
                    "Bin value is out of range. Maximum allowed is {}.",
                    max_bin
                )
            }
            IsinError::InvalidLatitude { min, max } => {
                write!(f, "Latitude must be between {} and {}.", min, max)
            }
            IsinError::InvalidLongitude { min, max } => {
                write!(f, "Longitude must be between {} and {}.", min, max)
            }
        }
    }
}
