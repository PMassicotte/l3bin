use std::fmt;

// TODO: Rename to Grid?
// Like Grid::Modis...
#[derive(Debug, PartialEq)]
pub enum Satellite {
    Custom(usize),
    Czcs,
    Meris,
    Modis,
    Seawifs,
    Sentinel3,
    Viirs,
}

#[derive(Debug, PartialEq)]
pub enum SatelliteError {
    InvalidNumLatitudeRows,
}

impl fmt::Display for SatelliteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SatelliteError::InvalidNumLatitudeRows => {
                write!(f, "The number of latitudes/rows must be divisible by 360")
            }
        }
    }
}

impl std::error::Error for SatelliteError {}

impl Satellite {
    pub fn with_num_latitude_rows(num_rows: usize) -> Result<Self, SatelliteError> {
        if num_rows % 360 == 0 {
            Ok(Satellite::Custom(num_rows))
        } else {
            Err(SatelliteError::InvalidNumLatitudeRows)
        }
    }

    pub fn num_latitude_rows(&self) -> usize {
        match self {
            Satellite::Custom(num_rows) => *num_rows,
            Satellite::Czcs => 1080,
            Satellite::Meris => 2160,
            Satellite::Modis => 4320,
            Satellite::Seawifs => 2160,
            Satellite::Sentinel3 => 4320,
            Satellite::Viirs => 4320,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_num_rows_valid() {
        assert_eq!(
            Satellite::with_num_latitude_rows(360),
            Ok(Satellite::Custom(360))
        );
    }

    #[test]
    fn test_with_num_rows_invalid() {
        assert_eq!(
            Satellite::with_num_latitude_rows(400),
            Err(SatelliteError::InvalidNumLatitudeRows)
        )
    }

    #[test]
    fn test_modis() {
        assert_eq!(Satellite::Modis.num_latitude_rows(), 4320);
    }

    #[test]
    fn test_seawifs() {
        assert_eq!(Satellite::Seawifs.num_latitude_rows(), 2160);
    }

    #[test]
    fn test_num_rows_custom() {
        assert_eq!(Satellite::Custom(720).num_latitude_rows(), 720);
    }
}
