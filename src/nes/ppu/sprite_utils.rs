use super::super::types::Addr;
use super::super::Ram;
use super::super::Mmc;

pub type Sprite = Vec<Vec<u8>>;

pub type SpritePosition = (u8, u8);

#[derive(Debug)]
pub struct SpriteConfig {
    pub offset_addr_by_name_table: Option<u16>,
    pub offset_addr_by_background_table: u16,
    pub offset_addr_by_spri