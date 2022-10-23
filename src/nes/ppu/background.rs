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
        tile: (u8, u8),
        scroll: (u8, u8),
        config: &mut SpriteConfig,
        mmc: &Mmc,
    ) {
        // INFO: Horizontal offsets range from 0 to 255. "Normal" vertical offsets range from 0 to 239,
        // while values of 240 to 255 are treated as -16 through -1 in a way, but tile data is incorrectly
        // fetched from the attribute table.
        let clamped_tile_y = tile.1 % 30;
        let table_id_offset = if (tile.1 / 30) % 2 == 0 { 0 } else { 2 };
        // background of a line.
        // Build viewport + 1 tile for background scroll.
        for x in 0..(