
#[derive(Debug)]
pub struct Ram {
    pub field: Vec<u8>,
}

impl Ram {
    pub fn new(buf: Vec<u8>) -> Ram {
        Ram { field: buf }