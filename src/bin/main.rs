// pub mod isin;
use l3bin::Isin;

fn main() {
    let isin = Isin::new(18);

    let res = isin.bin2bounds(&[367]);

    println!("{:?}", res);
}
