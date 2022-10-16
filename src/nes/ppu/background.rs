// use std::cell::Cell;

use super::super::mmc::*;
use super::super::ram::Ram;
use super::super::types::{Addr, Data};
use super::palette::*;
use super::sprite_utils::*;
use super::tile::Tile;

#[derive(Debug)]
pub struct BackgroundCtx {
    pub tile: Tile,
    pub scroll_x: Data,
    pub scroll_y: Data,
    pub is_enabled: bool,
}

pub type BackgroundField = Vec<BackgroundCtx>;

#[derive(Debug)]
pub struct Background(pub BackgroundField);

const TILE_PER_LINE: u8 = 32;

impl Background {
    pub fn new() -> Self {
        Background(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0 = Vec::new();
    }

    pub fn build_line<P: PaletteRam>(
        &mut self,
        vram: &Ram,
        cram: &Ram,
        palette: &P,
    