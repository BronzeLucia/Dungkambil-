
use super::super::super::types::Data;

#[derive(Debug)]
enum Enable {
    X,
    Y,
}

#[derive(Debug)]
pub struct PpuScroll {
    x: Data,
    y: Data,
    enable: Enable,
}

impl PpuScroll {
    pub fn new() -> Self {
        PpuScroll {
            x: 0,
            y: 0,
            enable: Enable::X,
        }
    }

    pub fn enable_x(&mut self) {
        self.enable = Enable::X;
    }    

    pub fn ge