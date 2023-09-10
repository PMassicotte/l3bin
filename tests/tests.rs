// Implement tests for isin
#[cfg(test)]
use l3bin::Isin;
mod tests {
    use super::*;

    //check modis resturn 4320 rows
    // #[test]
    // fn test_modis_rows() {
    //     let isin = Isin::new(4320);
    //     assert_eq!(isin.numrows, 4320);
    // }

    //check viirs resturn 4060 rows
    // #[test]
    // fn test_seawifs_rows() {
    //     let isin = Isin::new(2160);
    //     assert_eq!(isin.numrows, 2160);
    // }

    // Check lonlat fails if lon is out of bounds
    #[test]
    #[should_panic]
    fn test_lonlat2bin_lon_out_of_bounds() {
        let isin = Isin::new(4320);
        let lon = vec![181.0, 0.0];
        let lat = vec![0.0, 0.0];
        isin.lonlat2bin(lon, lat);
    }

    // Check lonlat fails if lat is out of bounds
    #[test]
    #[should_panic]
    fn test_lonlat2bin_lat_out_of_bounds() {
        let isin = Isin::new(4320);
        let lon = vec![0.0, 0.0];
        let lat = vec![91.0, 0.0];
        isin.lonlat2bin(lon, lat);
    }

    // Check lat2row fails if lat is out of bounds
    #[test]
    #[should_panic]
    fn test_lat2row_lat_out_of_bounds() {
        let isin = Isin::new(4320);
        isin.lat2row(91.0);
    }

    // Check bin2lonlat fails if bin is out of bounds
    // #[test]
    // #[should_panic]
    // fn test_bin2lonlat_bin_out_of_bounds() {
    //     let isin = Isin::new(4320);
    //     let mut bin = vec![isin.totbin + 1];
    //     isin.bin2lonlat(&mut bin);
    // }

    // #[test]
    // fn test_constrain_lat_lon() {
    //     assert_eq!(Isin::constrain_lat(90.0), 90.0);
    //     assert_eq!(Isin::constrain_lat(91.0), 90.0);
    //     assert_eq!(Isin::constrain_lat(-91.0), -90.0);
    //     assert_eq!(Isin::constrain_lat(-90.0), -90.0);
    //
    //     // assert_eq!(Isin::constrain_lon(180.0), 180.0);
    //     assert_eq!(Isin::constrain_lon(181.0), 180.0);
    //     assert_eq!(Isin::constrain_lon(-181.0), -180.0);
    //     assert_eq!(Isin::constrain_lon(-180.0), -180.0);
    // }
}
