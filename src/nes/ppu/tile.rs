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
    pub fn new<P: PaletteRam>(
        vram: &Ram,
        cram: &Ram,
        palette: &P,
        position: &SpritePosition,
        config: &SpriteConfig,
        mmc: &Mmc,
    ) -> Self {
        // INFO see. http://hp.vector.co.jp/authors/VA042397/nes/ppu.html
        let block_id = get_block_id(position);
        let sprite_id = get_sprite_id(&vram, position, config);
       