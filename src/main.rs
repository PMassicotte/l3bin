use l3bin::Isin;

fn main() {
    let isin = Isin::new(18);

    let res = isin.bin2bounds(&[367]);

    println!("{:?}", res);

    println!("{:?}", isin.lonlat2bin(&[78.0], &[-36.0]));
}
