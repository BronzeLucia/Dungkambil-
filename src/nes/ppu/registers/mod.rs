
mod oam;
mod ppu_addr;
mod ppu_data;
mod ppu_scroll;

use super::super::types::{Addr, Data};
use super::super::Ram;
use super::palette::*;
use super::PpuCtx;
// use super::super::helper::*;
use self::oam::Oam;
use self::ppu_addr::PpuAddr;
use self::ppu_data::PpuData;
use self::ppu_scroll::PpuScroll;

#[derive(Debug)]
pub struct Registers {
    pub ppu_ctrl1: Data,
    pub ppu_ctrl2: Data,
    pub ppu_status: Data,
    pub oam: Oam,
    pub ppu_addr: PpuAddr,
    pub ppu_data: PpuData,
    pub ppu_scroll: PpuScroll,
}

// PPU power up state
  // see. https://wiki.nesdev.com/w/index.php/PPU_power_up_state
  //
  // Memory map
  /*
  | addr           |  description               |
  +----------------+----------------------------+
  | 0x0000-0x0FFF  |  Pattern table#0           |
  | 0x1000-0x1FFF  |  Pattern table#1           |
  | 0x2000-0x23BF  |  Name table                |
  | 0x23C0-0x23FF  |  Attribute table           |
  | 0x2400-0x27BF  |  Name table                |
  | 0x27C0-0x27FF  |  Attribute table           |
  | 0x2800-0x2BBF  |  Name table                |
  | 0x2BC0-0x2BFF  |  Attribute table           |
  | 0x2C00-0x2FBF  |  Name Table                |
  | 0x2FC0-0x2FFF  |  Attribute Table           |
  | 0x3000-0x3EFF  |  mirror of 0x2000-0x2EFF   |
  | 0x3F00-0x3F0F  |  background Palette        |
  | 0x3F10-0x3F1F  |  sprite Palette            |
  | 0x3F20-0x3FFF  |  mirror of 0x3F00-0x3F1F   |
  */

pub trait PpuRegisters {
    fn read<P: PaletteRam>(&mut self, addr: Addr, ctx: &mut PpuCtx<P>) -> Data;

    fn write<P: PaletteRam>(&mut self, addr: Addr, data: Data, ctx: &mut PpuCtx<P>);

    fn is_sprite_8x8(&self) -> bool;

    fn clear_vblank(&mut self);

    fn set_vblank(&mut self);

    fn set_sprite_hit(&mut self);

    fn clear_sprite_hit(&mut self);

    fn get_sprite_table_offset(&self) -> Addr;

    fn get_background_table_offset(&self) -> Addr;

    fn get_ppu_addr_increment_value(&self) -> usize;

    fn get_name_table_id(&self) -> Data;

    fn get_scroll_x(&self) -> Data;

    fn get_scroll_y(&self) -> Data;

    fn is_irq_enable(&self) -> bool;

    fn is_background_enable(&self) -> bool;

    fn is_sprite_enable(&self) -> bool;

    fn is_background_masked(&self) -> bool;

    fn is_sprite_masked(&self) -> bool;
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            ppu_ctrl1: 0,
            ppu_ctrl2: 0,
            ppu_status: 0,
            oam: Oam::new(),
            ppu_addr: PpuAddr::new(),