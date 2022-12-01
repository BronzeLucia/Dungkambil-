use self::super::super::Mmc;
use self::super::palette::*;
use self::super::sprite_utils::*;
use self::super::Ram;

#[derive(Debug)]
pub struct Tile {
    pub sprite: Sprite,
    pub palette: PaletteList,
}

impl Tile {
    pub f