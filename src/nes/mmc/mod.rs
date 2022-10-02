#[derive(Debug)]
pub struct Mmc {
    mapper: u8,
    bank: u8,
}

impl Mmc {
    pub fn n