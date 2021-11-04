extern crate spidev;

use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use std::usize;

pub const PANEL_LINE_SIZE:  usize = 32;
pub const PANEL_LINE_COUNT: usize = 32;
pub const PANEL_COUNT:      usize = 6;
pub const LEDS_PER_PIXEL:   usize = 3;

pub const LINE_BUFFER_SIZE:   usize = PANEL_LINE_SIZE * LEDS_PER_PIXEL;
pub const PANEL_BUFFER_SIZE:  usize = LINE_BUFFER_SIZE * PANEL_LINE_COUNT;
pub const FRAME_BUFFER_SIZE:  usize = PANEL_BUFFER_SIZE * PANEL_COUNT;

// Commands
const LINE_MODE:    u8 = 0x01;
const FRAME_MODE:   u8 = 0x03;
const RESET:        u8 = 0x08;
const FLUSH:        u8 = 0x20;

pub struct Display {
    spi: Spidev
}

impl Display {

    pub fn new(device: &str) -> Display {
        let options = SpidevOptions::new()
            .mode(SpiModeFlags::SPI_MODE_0)
            .bits_per_word(8)
            .max_speed_hz(16_000_000)
            .build();
        let mut spidev = Spidev::open(&device).unwrap();
        spidev.configure(&options).unwrap();
        Display { spi: spidev }
    }

    pub fn write_line(&mut self, panel: u8, line: u8, data: &[u8; LINE_BUFFER_SIZE]) {
        const HEADER_SIZE: usize = 3;
        
        let mut buffer = Vec::with_capacity(HEADER_SIZE + LINE_BUFFER_SIZE);
        let header = [LINE_MODE, panel, line];
        buffer.extend_from_slice(&header);
        buffer.extend_from_slice(&data[..]);

        self.spi.write(&buffer).unwrap();
    }

    pub fn write_frame(&mut self, data: &[u8]) {
        const HEADER_SIZE: usize = 1;

        let mut buffer = Vec::with_capacity(HEADER_SIZE + FRAME_BUFFER_SIZE);
        let header = [FRAME_MODE];
        buffer.extend_from_slice(&header);
        buffer.extend_from_slice(&data[..]);

        self.spi.write(&buffer).unwrap();
    }

    pub fn flush(&mut self) {
        self.spi.write(&[FLUSH]).unwrap();
    }

    pub fn reset(&mut self) {
        self.spi.write(&[RESET]).unwrap();
    }

}
