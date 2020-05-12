mod utils;

use wasm_bindgen::prelude::*;
extern crate rs8080_emulator as emulator;
use emulator::RS8080;
use emulator::{MemLimiter, WriteAction, DataBus};
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[link(wasm_import_module = "./modules/sounds.js")]
extern { 
    fn play_shot(); 
    fn play_player_die(); 
    fn play_invader_die(); 
    fn play_fleet1(); 
    fn play_fleet2(); 
    fn play_fleet3(); 
    fn play_fleet4(); 
    fn play_ufo_highpitch(); 
    fn stop_ufo_highpitch(); 
}

struct SpaceInvadersLimit {}
impl MemLimiter for SpaceInvadersLimit {
    fn check_write(&self, adr: u16, _: u8) -> WriteAction {
        if adr < 0x2000 {
            eprintln!("block: write mem < 0x2000");
            WriteAction::Ignore
        } else if adr >= 0x4000 {
            eprintln!("block: write mem >= 0x4000");
            WriteAction::Ignore
        } else {
            WriteAction::Allow
        }
    }
    fn check_read(&self, _: u16, read_byte: u8) -> u8 {
        read_byte
    }
}


#[wasm_bindgen]
pub struct Emulator {
    inner: RS8080<SpaceInvadersIO, SpaceInvadersLimit>,
}

impl fmt::Display for Emulator{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.inner)
    }
}

#[wasm_bindgen]
impl Emulator{
    pub fn new(rom : &[u8]) -> Self{
        let mut invaders_emu = Emulator{ inner: RS8080::new_with_limit(SpaceInvadersIO::new(), SpaceInvadersLimit{})};
        //let rom = include_bytes!("../roms/invaders.rom");
        invaders_emu.inner.load_to_mem(rom, 0);
        invaders_emu
    }

    pub fn emulate_next(&mut self) -> u32{
        self.inner.emulate_next().0 as u32
    }

    pub fn interrupt(&mut self, addr : u16){
        self.inner.call_interrupt(addr);
    }

    pub fn get_vram(&self) -> * const u8{
        unsafe{self.inner.get_mem().as_ptr().offset(0x2400)}
        //[0x2400..0x3FFF]
    }

    pub fn to_string(&self) -> String{
        format!("{}", &self.inner)
    }

    pub fn is_int_enabled(&self) -> bool{
        self.inner.int_enabled()
    }

    pub fn player1_start(&mut self, is_down : bool){
        if is_down{
            *self.inner.get_io_mut().port(1) |= 0b0000_0100;
        }else{
            *self.inner.get_io_mut().port(1) &= !0b0000_0100;
        }
    }

    pub fn player2_start(&mut self, is_down : bool){
        if is_down{
            *self.inner.get_io_mut().port(1) |= 0b0000_0010;
        }else{
            *self.inner.get_io_mut().port(1) &= !0b0000_0010;
        }
    }

    pub fn insert_coin(&mut self, is_down: bool){
        if is_down{
            *self.inner.get_io_mut().port(1) |= 0x1;
        }else{
            *self.inner.get_io_mut().port(1) &= !0x1;
        }
    }

    pub fn move_left(&mut self, is_down : bool){
        if is_down{
            *self.inner.get_io_mut().port(1) |= 0b0010_0000;
            *self.inner.get_io_mut().port(2) |= 0b0010_0000;
        }else{
            *self.inner.get_io_mut().port(1) &= !0b0010_0000;
            *self.inner.get_io_mut().port(2) &= !0b0010_0000;
        }
    }

    pub fn move_right(&mut self, is_down : bool){
        if is_down{
            *self.inner.get_io_mut().port(1) |= 0b0100_0000;
            *self.inner.get_io_mut().port(2) |= 0b0100_0000;
        }else{
            *self.inner.get_io_mut().port(1) &= !0b0100_0000;
            *self.inner.get_io_mut().port(2) &= !0b0100_0000;
        }
    }

    pub fn shot(&mut self, is_down : bool){
        if is_down{
            *self.inner.get_io_mut().port(1) |= 0b0001_0000;
            *self.inner.get_io_mut().port(2) |= 0b0001_0000;
        }else{
            *self.inner.get_io_mut().port(1) &= !0b0001_0000;
            *self.inner.get_io_mut().port(2) &= !0b0001_0000;
        }
    }
}

pub(crate) struct SpaceInvadersIO {
    ports: [u8; 6],
    shift0: u8,
    shift1: u8,
    shift_offset: u8,
}

impl SpaceInvadersIO {
    pub fn new() -> SpaceInvadersIO {
        SpaceInvadersIO {
            ports: [0; 6],
            shift0: 0,
            shift1: 0,
            shift_offset: 0,
        }
    }

    fn set_shift_offset(&mut self, offset: u8) {
        self.shift_offset = offset & 0x7;
    }

    fn shift(&self) -> u8 {
        (((self.shift0 as u16) << 8) | self.shift1 as u16).rotate_left(self.shift_offset as u32)
            as u8
    }
}

impl DataBus for SpaceInvadersIO {
    fn port_in(&mut self, port: u8) -> u8 {
        match port {
            0 => 0xf,
            1 => self.ports[1],
            3 => self.shift(),
            _ => self.ports[port as usize],
        }
    }

    fn port_out(&mut self, value: u8, port: u8) {
        match port {
            2 => {
                self.set_shift_offset(value);
            }
            3 => {
                unsafe{
                    if value & 0x1 > 0 && self.ports[3] & 0x1 == 0 {
                        play_ufo_highpitch();
                    } else if value & 0x1 == 0 && self.ports[3] & 0x1 > 0 {
                        stop_ufo_highpitch();
                    }
                    if value & 0x2 > 0 && self.ports[3] & 0x2 == 0 {
                        play_shot();
                    }
                    if value & (1 << 2) > 0 && self.ports[3] & (1 << 2) == 0 {
                        play_player_die();
                    }
                    if value & (1 << 3) > 0 && self.ports[3] & (1 << 3) == 0 {
                        play_invader_die();
                    }
                }

                self.ports[3] = value;
            }
            4 => {
                self.shift0 = self.shift1;
                self.shift1 = value;
            }
            5 => {
                unsafe{
                    if value & (1 << 0) > 0 && self.ports[5] & (1 << 0) == 0 {
                        play_fleet1();
                    }
                    if value & (1 << 1) > 0 && self.ports[5] & (1 << 1) == 0 {
                        play_fleet2();
                    }
                    if value & (1 << 2) > 0 && self.ports[5] & (1 << 2) == 0 {
                        play_fleet3();
                    }
                    if value & (1 << 3) > 0 && self.ports[5] & (1 << 3) == 0 {
                        play_fleet4();
                    }
                }
                self.ports[5] = value;
            }
            _ => {}
        }
    }

    fn port(&mut self, index: usize) -> &mut u8 {
        &mut self.ports[index]
    }
}
