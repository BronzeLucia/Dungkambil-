
use super::super::super::types::{Data, Addr};
use super::super::super::ram::*;

#[derive(Debug)]
pub struct Oam {
    addr: Addr,
}


impl Oam {
    pub fn new() -> Self {
        Oam { addr: 0 }