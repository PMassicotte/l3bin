#[derive(Debug)]
pub enum IsinError {
    InvalidBinRange { max_bin: usize },
}
