
pub mod background;
mod palette;
mod registers;
mod sprite;
mod sprite_utils;
pub mod tile;

use self::super::mmc::Mmc;
use self::super::ram::Ram;
pub use self::background::*;
pub use self::palette::*;
use self::registers::*;
pub use self::sprite::*;
pub use self::sprite_utils::*;
pub use self::tile::*;
use super::types::{Addr, Data};

#[derive(Debug)]
pub struct PpuConfig {
    pub is_horizontal_mirror: bool,
}

#[derive(Debug)]
pub struct PpuCtx<P: PaletteRam> {
    pub palette: P,
    pub vram: Box<Ram>,
    pub cram: Box<Ram>,
    pub sprite_ram: Box<Ram>,
}

const CYCLES_PER_LINE: usize = 341;

#[derive(Debug)]
pub struct Ppu {
    pub cycle: usize,
    pub line: usize,
    pub registers: Registers,
    pub ctx: PpuCtx<Palette>,
    pub sprites: SpritesWithCtx,
    pub background: Background,
    pub config: PpuConfig,
}

impl Ppu {
    pub fn new(character_ram: Vec<u8>, config: PpuConfig) -> Ppu {
        println!("cram length 0x{:x}", character_ram.len());
        Ppu {
            cycle: 0,
            line: 0,
            registers: Registers::new(),
            ctx: PpuCtx {
                palette: Palette::new(),
                vram: Box::new(Ram::new(vec![0; 0x2000])),
                cram: Box::new(Ram::new(character_ram)),
                sprite_ram: Box::new(Ram::new(vec![0; 0x0100])),
            },
            sprites: Vec::new(),
            background: Background::new(),
            config,
        }
    }

    pub fn read(&mut self, addr: Addr) -> Data {
        self.registers.read(addr, &mut self.ctx)
    }

    pub fn write(&mut self, addr: Addr, data: Data) {
        self.registers.write(addr, data, &mut self.ctx);
    }

    // The PPU draws one line at 341 clocks and prepares for the next line.
    // While drawing the BG and sprite at the first 256 clocks,
    // it searches for sprites to be drawn on the next scan line.
    // Get the pattern of the sprite searched with the remaining clock.
    pub fn run(&mut self, cycle: usize, nmi: &mut bool, mmc: &Mmc) -> bool {
        let cycle = self.cycle + cycle;
        if cycle < CYCLES_PER_LINE {
            self.cycle = cycle;
            return false;
        }

        if self.line == 0 {
            self.background.clear();
        }

        if self.has_sprite_hit(cycle) {
            self.registers.set_sprite_hit();
        }

        self.cycle = cycle - CYCLES_PER_LINE;
        self.line = self.line + 1;

        let scroll_x = self.registers.get_scroll_x();
        let scroll_y = self.registers.get_scroll_y();
        if self.line <= 240 && self.line % 8 == 0 && scroll_y <= 240 {
            let mut config = SpriteConfig {
                offset_addr_by_name_table: None,
                offset_addr_by_background_table: self.registers.get_background_table_offset(),
                offset_addr_by_sprite_table: self.registers.get_sprite_table_offset(),
                is_horizontal_mirror: self.config.is_horizontal_mirror,
                is_background_enable: self.registers.is_background_enable(),
            };
            let tile_x = ((scroll_x as usize
                + (self.registers.get_name_table_id() % 2) as usize * 256)
                / 8) as u8;
            let tile_y = self.get_scroll_tile_y();
            self.background.build_line(
                &self.ctx.vram,
                &self.ctx.cram,
                &self.ctx.palette,
                (tile_x, tile_y),
                (scroll_x, scroll_y),
                &mut config,
                &mmc,
            );
        }

        if self.line == 241 {
            self.registers.set_vblank();
            self.registers.clear_sprite_hit();
            if self.registers.is_irq_enable() {
                *nmi = true;
            }
        }

        if self.line >= 262 {
            self.registers.clear_vblank();
            self.registers.clear_sprite_hit();
            *nmi = false;
            self.line = 0;
            self.sprites = build_sprites(
                &self.ctx.cram,
                &self.ctx.sprite_ram,
                &self.ctx.palette,
                self.registers.get_sprite_table_offset(),
                self.registers.is_sprite_8x8(),
                &mmc,
            );
            return true;
        }
        false
    }

    pub fn transfer_sprite(&mut self, addr: Addr, data: Data) {
        let addr = addr + self.registers.oam.get_addr();
        self.ctx.sprite_ram.write(addr % 0x100, data);
    }

    fn get_scroll_tile_y(&self) -> Data {
        ((self.registers.get_scroll_y() as usize
            + self.line
            + ((self.registers.get_name_table_id() / 2) as usize * 240))
            / 8) as Data
    }

    fn has_sprite_hit(&self, cycle: usize) -> bool {
        let y = self.ctx.sprite_ram.read(0) as usize;
        let x = self.ctx.sprite_ram.read(3) as usize;
        // (y == self.line) && self.registers.is_sprite_enable()
        (y == self.line) && x <= cycle && self.registers.is_sprite_enable()
    }
}