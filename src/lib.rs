// See appendix A: https://ntrs.nasa.gov/api/citations/19960007721/downloads/19960007721.pdf
// https://clouds.eos.ubc.ca/~phil/courses/eosc582/html/find_bins.html

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
    /// Create a new ISIN grid
    /// # Arguments
    /// * `numrows` - The number of rows in the ISIN grid. MODIS is 4320, SeaWiFS is 2160.
    /// # Example
    /// ```
    /// let isin = l3bin::Isin::new(4320);
    /// ```
    pub fn new(numrows: usize) -> Isin {
        let mut basebin: Vec<usize> = Vec::with_capacity(numrows);
        let mut numbin: Vec<usize> = Vec::with_capacity(numrows);
        let mut latbin: Vec<f64> = Vec::with_capacity(numrows);

        basebin.push(1);

        for row in 0..numrows {
            latbin.push(((row as f64 + 0.5) * 180.0 / (numrows as f64)) - 90.0);
            numbin.push(
                (2.0 * numrows as f64 * f64::cos(latbin[row] * std::f64::consts::PI / 180.0) + 0.5)
                    as usize,
            );

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
    /// let isin = l3bin::Isin::new(4320);
    /// let row = isin.lat2row(45.0);
    /// println!("Row: {:?}", row);
    /// ```
    pub fn lat2row(&self, lat: f64) -> usize {
        assert_eq!(is_vector_within_bounds(&vec![lat], MIN_LAT, MAX_LAT), true);

        let row = (90.0 + lat) * (self.numrows as f64) / 180.0 + 1.0;
        row as usize
    }

    /// Convert lonlat to bin
    /// # Arguments
    /// * `lon` - A vector of longitude values
    /// * `lat` - A vector of latitude values
    /// # Example
    /// ```
    /// let is = l3bin::Isin::new(4320);
    /// let bin = is.lonlat2bin(vec![45.0], vec![45.0]);
    /// println!("Bin: {:?}", bin);
    /// ```
    pub fn lonlat2bin(&self, lon: Vec<f64>, lat: Vec<f64>) -> Vec<usize> {
        assert_eq!(is_vector_within_bounds(&lon, MIN_LON, MAX_LON), true);
        assert_eq!(is_vector_within_bounds(&lat, MIN_LAT, MAX_LAT), true);

        let mut bin: Vec<usize> = Vec::with_capacity(lat.len());

        for i in 0..lat.len() {
            let row = self.lat2row(lat[i]) - 1; // not sure why -1 is needed here
            let mut col = ((lon[i] + 180.0) * (self.numbin[row] as f64 / 360.0)) as usize;

            if col >= self.numbin[row] {
                col = self.numbin[row] - 1;
            }

            bin.push(self.basebin[row] + col as usize);
        }

        bin
    }

    /// Convert bin to lonlat
    /// # Arguments
    /// * `bin` - A vector of bin values
    /// determines the number of rows in the ISIN grid.
    /// # Example
    /// ```
    /// let isin = l3bin::Isin::new(4320);
    /// let lonlat = isin.bin2lonlat(&mut vec![245535, 245536, 247290, 249046, 249047, 250809]);
    /// println!("Lonlat: {:?}", lonlat);
    /// ```
    pub fn bin2lonlat(&self, bin: &[usize]) -> Vec<(f64, f64)> {
        assert_eq!(bin.iter().all(|&b| b >= 1 && b <= self.totbin), true);

        let mut result: Vec<(f64, f64)> = Vec::with_capacity(bin.len());

        for bin_val in bin.iter() {
            let mut row = self.numrows - 1;
            let bin_val = if *bin_val < 1 { 1 } else { *bin_val };

            while bin_val < self.basebin[row] {
                row -= 1;
            }
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
    /// let isin = l3bin::Isin::new(4320);
    /// let bounds = isin.bin2bounds(&mut vec![245535, 245536, 247290, 249046, 249047, 250809]);
    /// println!("Bounds: {:?}", bounds);
    /// ```
    /// # Note
    /// The bounds are returned in the order north, south, west, east.
    pub fn bin2bounds(&self, bin: &[usize]) -> Vec<(f64, f64, f64, f64)> {
        assert_eq!(bin.iter().all(|&b| b >= 1 && b <= self.totbin), true);

        let mut result: Vec<(f64, f64, f64, f64)> = Vec::with_capacity(bin.len());

        for bin_val in bin.iter() {
            let mut row = self.numrows - 1;
            let bin_val = if *bin_val < 1 { 1 } else { *bin_val };

            while bin_val < self.basebin[row] {
                row -= 1
            }

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

fn is_vector_within_bounds(numbers: &Vec<f64>, lower_bound: f64, upper_bound: f64) -> bool {
    numbers
        .iter()
        .all(|&num| num >= lower_bound && num <= upper_bound)
}
