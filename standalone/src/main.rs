extern crate rustynes;
extern crate sdl2;

use sdl2::{Sdl};
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::pixels::{Color};
use sdl2::render::{WindowCanvas};
use sdl2::rect::{Point};

use std::time::{Duration, SystemTime};

use std::env;
use std::fs;
use rustynes::nes;
use rustynes::nes::Context;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 224;

const PAD_A: u8      = 0x01;
const PAD_B: u8      = 0x02;
const PAD_SELECT: u8 = 0x04;
const PAD_START: u8  = 0x08;
const PAD_U: u8      = 0x10;
const PAD_D: u8      = 0x20;
const PAD_L: u8      = 0x40;
const PAD_R: u8      = 0x80;

fn keycode_to_pad(key: Keycode) -> u8 {
    match key {
        Keycode::X => PAD_A,
        Keycode::Z => PAD_B,
        Keycode::A => PAD_SELECT,
        Keycode::S => PAD_START,
        Keycode::Up => PAD_U,
        Keycode::Down => PAD_D,
        Keycode::Left => PAD_L,
        Keycode::Right => PAD_R,
        _ => 0,
    }
}

pub struct App {
    sdl_context: Sdl,
    canvas: WindowCanvas,

    ctx: Option<Context>,
}

impl App {
    pub fn new() -> App {
        let sdl_context = sdl2::init().un