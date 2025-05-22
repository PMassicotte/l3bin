# l3bin for rust

![CI](https://github.com/PMassicotte/l3bin/actions/workflows/rust.yml/badge.svg)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) [![Crates.io](https://img.shields.io/crates/v/l3bin.svg)](https://crates.io/crates/l3bin)

Support the NASA / GlobColour / CCI ISIN grid used for MODIS L3BIN satellite products.

## Usage

This simple example shows how to convert a latitude to a row and then convert a list of bin numbers to lon/lat coordinates based on the [MODIS ISIN grid](https://modis-land.gsfc.nasa.gov/MODLAnD_grid.html).

```rust
use l3bin::isin::Isin;
use l3bin::satellites;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let isin = Isin::new(satellites::Satellite::Modis);

    let row = isin.lat2row(45.0)?;
    println!("Row: {}", row);

    let coords = isin.bin2lonlat(&[245535, 245536, 247290, 249046, 249047, 250809])?;
    println!("{:#?}", coords);

    Ok(())
}
```

Which will output:

```
Row: 3240
[
    (
        -162.2057142857143,
        -78.3125,
    ),
    (
        -162.0,
        -78.3125,
    ),
    (
        -161.24145785876993,
        -78.27083333333333,
    ),
    (
        -161.31593874078277,
        -78.22916666666667,
    ),
    (
        -161.11174134997162,
        -78.22916666666667,
    ),
    (
        -161.3793103448276,
        -78.1875,
    ),
]
```

## Resources

- See appendix A: https://ntrs.nasa.gov/api/citations/19960007721/downloads/19960007721.pdf
- https://web.archive.org/web/20210413094158/https://clouds.eos.ubc.ca/~phil/courses/eosc582/html/find_bins.html
