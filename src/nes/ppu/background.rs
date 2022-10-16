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

#[derive