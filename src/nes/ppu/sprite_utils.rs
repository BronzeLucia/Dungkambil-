use super::super::types::Addr;
use super::super::Ram;
use super::super::Mmc;

pub type Sprite = Vec<Vec<u8>>;

pub type SpritePosition = (u8, u8);

#[derive(Debug)]
pub struct SpriteConfig {
    pub offset_addr_by_name_table: Option<u16>,
    pub offset_addr_by_background_table: u16,
    pub offset_addr_by_sprite_table: u16,
    pub is_horizontal_mirror: bool,
    pub is_background_enable: bool,
}

pub fn mirror_down_sprite_addr(addr: Addr, is_horizontal_mirror: bool) -> Addr {
    if !is_horizontal_mirror {
        return addr;
    }
    if (addr >= 0x0400 && addr < 0x0800) || addr >= 0x0C00 {
        return addr - 0x400 as Addr;
    }
    addr
}

pub fn get_block_id(po