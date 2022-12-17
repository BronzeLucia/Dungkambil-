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
const PAD_U: u8