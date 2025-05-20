use l3bin::isin::Isin;
use l3bin::satellites::Satellite;

fn main() {
    let isin = Isin::new(Satellite::Seawifs);

    let binnum = 367;
    let bounds = isin.bin2bounds(&[binnum]);
    let lonlat = isin.bin2lonlat(&[binnum]);

    println!("SeaWiFS ISIN grid boundaries for bin {binnum}: {bounds:#?}");

    println!("Lon/lat for bin {binnum} for Seawifs: {lonlat:?}");

    let (lon, lat) = lonlat[0];

    println!(
        "Lon/lat: {:?} has bin number {:?}",
        (lon, lat),
        isin.lonlat2bin(&[lon], &[lat])
    );
}
