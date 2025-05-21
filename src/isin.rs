// See appendix A: https://ntrs.nasa.gov/api/citations/19960007721/downloads/19960007721.pdf
// https://clouds.eos.ubc.ca/~phil/courses/eosc582/html/find_bins.html

use crate::{bounds_checker::is_vector_within_bounds, satellites::Satellite};

const MIN_LON: f64 = -180.0;
const MAX_LON: f64 = 180.0;
const MIN_LAT: f64 = -90.0;
const MAX_LAT: f64 = 90.0;

#[derive(Debug)]
pub struct Isin {
    basebin: Vec<usize>,
    numbin: Vec<usize>,
    latbin: Vec<f64>,
    totbin: usize,
    numrows: usize,
}

impl Isin {
    /// Creates a new ISIN grid based on the resolution of the given satellite.
    ///
    /// The `sat` parameter determines the grid size by specifying the satellite
    /// whose resolution is used to calculate the number of rows in the grid.
    ///
    /// Predefined satellites have fixed resolutions, for example:
    /// - MODIS: 4320 rows
    /// - SeaWiFS: 2160 rows
    ///
    /// Alternatively, you can specify a custom resolution by using
    /// `Satellite::Custom(resolution)`, where `resolution` is the desired number of rows.
    ///
    /// This allows generating grids of different resolutions depending on
    /// the satellite source or custom requirements.
    ///
    /// # Arguments
    ///
    /// * `sat` - The satellite used to derive the grid resolution, either a predefined satellite or a custom resolution.
    ///
    /// # Example
    ///
    /// ```
    /// use l3bin::satellites::Satellite;
    /// use l3bin::isin::Isin;
    ///
    /// // Using a predefined satellite
    /// let isin_modis = Isin::new(Satellite::Modis);
    ///
    /// // Using a custom resolution of 1000 rows
    /// let isin_custom = Isin::new(Satellite::Custom(1000));
    /// ```
    pub fn new(sat: Satellite) -> Isin {
        let numrows = sat.resolution();
        let mut basebin: Vec<usize> = Vec::with_capacity(numrows);
        let mut numbin: Vec<usize> = Vec::with_capacity(numrows);
        let mut latbin: Vec<f64> = Vec::with_capacity(numrows);

        basebin.push(1);

        let pi_over_180 = std::f64::consts::PI / 180.0;

        for row in 0..numrows {
            let lat = ((row as f64 + 0.5) * 180.0 / numrows as f64) - 90.0;
            latbin.push(lat);

            let cos_lat = f64::cos(lat * pi_over_180);
            let num = (2.0 * numrows as f64 * cos_lat + 0.5) as usize;
            numbin.push(num);

            if row > 0 {
                basebin.push(basebin[row - 1] + numbin[row - 1]);
            }
        }

        let totbin = basebin[numrows - 1] + numbin[numrows - 1] - 1;

        Isin {
            basebin,
            numbin,
            latbin,
            totbin,
            numrows,
        }
    }

    /// Convert lat to row
    /// # Arguments
    /// * `lat` - A latitude value
    /// # Example
    /// ```
    /// use l3bin::satellites::Satellite;
    /// use l3bin::isin::Isin;
    /// let isin = Isin::new(Satellite::Modis);
    /// let row = isin.lat2row(45.0);
    /// println!("Row: {:?}", row);
    /// ```
    pub fn lat2row(&self, lat: f64) -> usize {
        assert!(is_vector_within_bounds(&[lat], MIN_LAT, MAX_LAT));

        let row = (90.0 + lat) * (self.numrows as f64) / 180.0;
        row as usize
    }

    /// Convert lonlat to bin
    /// # Arguments
    /// * `lon` - A vector of longitude values
    /// * `lat` - A vector of latitude values
    /// # Example
    /// ```
    /// use l3bin::satellites::Satellite;
    /// use l3bin::isin::Isin;
    /// let is = Isin::new(Satellite::Modis);
    /// let bin = is.lonlat2bin(&[45.0], &[45.0]);
    /// println!("Bin: {:?}", bin);
    /// ```
    pub fn lonlat2bin(&self, lon: &[f64], lat: &[f64]) -> Vec<usize> {
        assert!(is_vector_within_bounds(lon, MIN_LON, MAX_LON));
        assert!(is_vector_within_bounds(lat, MIN_LAT, MAX_LAT));

        let mut bin: Vec<usize> = Vec::with_capacity(lat.len());

        for i in 0..lat.len() {
            let row = self.lat2row(lat[i]);
            let mut col = ((lon[i] + 180.0) * (self.numbin[row] as f64 / 360.0)) as usize;

            if col >= self.numbin[row] {
                col = self.numbin[row] - 1;
            }

            bin.push(self.basebin[row] + col);
        }

        bin
    }

    /// Convert bin to lonlat
    /// # Arguments
    /// * `bin` - A vector of bin values
    ///   determines the number of rows in the ISIN grid.
    /// # Example
    /// ```
    /// use l3bin::satellites::Satellite;
    /// use l3bin::isin::Isin;
    /// let isin = Isin::new(Satellite::Modis);
    /// let lonlat = isin.bin2lonlat(&mut vec![245535, 245536, 247290, 249046, 249047, 250809]);
    /// println!("Lonlat: {:?}", lonlat);
    /// ```
    pub fn bin2lonlat(&self, bin: &[usize]) -> Vec<(f64, f64)> {
        assert!(bin.iter().all(|&b| b >= 1 && b <= self.totbin));

        let mut result: Vec<(f64, f64)> = Vec::with_capacity(bin.len());

        for bin_val in bin.iter() {
            let bin_val = if *bin_val < 1 { 1 } else { *bin_val };

            // Find the row using binary search
            let row = match self.basebin.binary_search(&bin_val) {
                Ok(r) => r,
                Err(r) => r - 1,
            };

            let lat = self.latbin[row];
            let lon = 360.0 * (bin_val as f64 - self.basebin[row] as f64 + 0.5)
                / self.numbin[row] as f64
                - 180.0;

            result.push((lon, lat));
        }

        result
    }

    /// Convert bin to bounds
    /// # Arguments
    /// * `bin` - A vector of bin values
    /// # Example
    /// ```
    /// use l3bin::satellites::Satellite;
    /// use l3bin::isin::Isin;
    /// let isin = Isin::new(Satellite::Modis);
    /// let bounds = isin.bin2bounds(&mut vec![245535, 245536, 247290, 249046, 249047, 250809]);
    /// println!("Bounds: {:?}", bounds);
    /// ```
    /// # Note
    /// The bounds are returned in the order north, south, west, east.
    pub fn bin2bounds(&self, bin: &[usize]) -> Vec<(f64, f64, f64, f64)> {
        assert!(bin.iter().all(|&b| b >= 1 && b <= self.totbin));

        let mut result: Vec<(f64, f64, f64, f64)> = Vec::with_capacity(bin.len());

        for bin_val in bin.iter() {
            let bin_val = if *bin_val < 1 { 1 } else { *bin_val };

            // Find the row using binary search
            let row = match self.basebin.binary_search(&bin_val) {
                Ok(r) => r,
                Err(r) => r - 1,
            };

            let north = self.latbin[row] + (90.0 / self.numrows as f64);
            let south = self.latbin[row] - (90.0 / self.numrows as f64);

            let lon = 360.0 * (bin_val as f64 - self.basebin[row] as f64 + 0.5)
                / self.numbin[row] as f64
                - 180.0;

            let west = lon - 180.0 / self.numbin[row] as f64;
            let east = lon + 180.0 / self.numbin[row] as f64;

            result.push((north, south, west, east));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::satellites::Satellite;

    #[test]
    fn test_lat2row() {
        let isin = Isin::new(Satellite::Modis);

        assert_eq!(isin.lat2row(0.0), isin.numrows / 2);
        assert_eq!(isin.lat2row(-90.0), 0);
        assert_eq!(isin.lat2row(90.0), isin.numrows);
        assert_eq!(isin.lat2row(45.0), 3240);
    }

    #[test]
    fn test_lonlat2bin_and_bin2lonlat() {
        let isin = Isin::new(Satellite::Modis);
        let lon = vec![0.0, 45.0, -45.0];
        let lat = vec![0.0, 45.0, -45.0];
        let bins = isin.lonlat2bin(&lon, &lat);
        let lonlat = isin.bin2lonlat(&bins);

        assert_eq!(lonlat.len(), 3);

        for ((l, a), (ol, oa)) in lon.iter().zip(lat.iter()).zip(lonlat.iter()) {
            assert!((l - ol).abs() < 2.0);
            assert!((a - oa).abs() < 2.0);
        }
    }

    #[test]
    fn test_bin2bounds() {
        let isin = Isin::new(Satellite::Modis);
        let bins = vec![isin.lonlat2bin(&[0.0], &[0.0])[0]];
        let bounds = isin.bin2bounds(&bins);
        assert_eq!(bounds.len(), 1);

        let (north, south, west, east) = bounds[0];

        assert!(north > south);
        assert!(east > west);
    }

    #[test]
    // https://github.com/sosoc/croc/blob/e91fcd64017e955922615244577fc8c803cb9a76/tests/testthat/test-bins.R
    fn test_bin2lonlat() {
        let isin = Isin::new(Satellite::Modis);

        let bins = vec![
            6308931, 8842288, 13611957, 21580540, 4792301, 21347245, 22447068, 15701664, 14948805,
            1468146,
        ];

        let expected: Vec<(f64, f64)> = vec![
            (94.38794233289644, -27.979166666666664),
            (-48.701065485454336, -14.8125),
            (-152.3903123903124, 8.395833333333329),
            (-14.143114852675893, 54.72916666666666),
            (142.32256203115986, -36.645833333333336),
            (95.85982382229031, 52.8125),
            (-179.68085106382978, 62.8125),
            (-98.6479217603912, 18.77083333333333),
            (-123.04097771387491, 14.979166666666671),
            (128.35497835497836, -61.22916666666667),
        ];

        let res = isin.bin2lonlat(&bins);

        assert_eq!(res, expected);
    }
}
