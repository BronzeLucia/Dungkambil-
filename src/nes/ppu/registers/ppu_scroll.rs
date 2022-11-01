
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
  