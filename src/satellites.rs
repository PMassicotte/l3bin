use std::fmt;

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
    InvalidResolution,
}

impl fmt::Display for SatelliteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SatelliteError::InvalidResolution => write!(f, "Resolution must be divisible by 360"),
        }
    }
}

impl std::error::Error for SatelliteError {}

impl Satellite {
    pub fn with_resolution(resolution: usize) -> Result<Self, SatelliteError> {
        if resolution % 360 == 0 {
            Ok(Satellite::Custom(resolution))
        } else {
            Err(SatelliteError::InvalidResolution)
        }
    }

    pub fn resolution(&self) -> usize {
        match self {
            Satellite::Custom(resolution) => *resolution,
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
    fn test_with_resolution_valid() {
        assert_eq!(Satellite::with_resolution(360), Ok(Satellite::Custom(360)));
    }

    #[test]
    fn test_with_resolution_invalid() {
        assert_eq!(
            Satellite::with_resolution(400),
            Err(SatelliteError::InvalidResolution)
        )
    }

    #[test]
    fn test_resolution_modis() {
        assert_eq!(Satellite::Modis.resolution(), 4320);
    }

    #[test]
    fn test_resolution_seawifs() {
        assert_eq!(Satellite::Seawifs.resolution(), 2160);
    }

    #[test]
    fn test_resolution_custom() {
        assert_eq!(Satellite::Custom(720).resolution(), 720);
    }
}
