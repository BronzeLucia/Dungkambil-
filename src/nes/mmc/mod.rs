#[derive(Debug)]
pub struct Mmc {
    mapper: u8,
    bank: u8,
}

impl Mmc {
    pub fn new(mapper: u8, bank: u8) -> Self {
        Mmc { bank, mapper }
    }

    pub fn set_bank(&mut self, bank: u8) {
        self.bank = bank;
    }

    pub fn get_bank(&self) -> u8 {
        self.ban