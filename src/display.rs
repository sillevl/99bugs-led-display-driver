extern crate spidev;
use std::io::prelude::*;
use self::spidev::{Spidev, SpidevOptions, SPI_MODE_0};
use std::usize;

pub const PANEL_LINE_SIZE: usize = 32;
pub const PANEL_LINE_COUNT: usize = 32;
pub const PANEL_COUNT: usize = 6;
pub const LEDS_PER_PIXEL: usize = 3;

pub const LINE_BUFFER_SIZE: usize = PANEL_LINE_SIZE * LEDS_PER_PIXEL;
pub const PANEL_BUFFER_SIZE: usize = LINE_BUFFER_SIZE * PANEL_LINE_COUNT;

// enum Command {
//     Line,
//     Flush,
// }

// fn value(command: Command) -> u8 {
//     match command {
//         Command::Line => 0x01,
//         Command::Flush => 0x08,
//     }
// }


pub struct Display {
    spi: Spidev
}

impl Display {

    pub fn new(device: &str) -> Display {
        let options = SpidevOptions::new()
            .mode(SPI_MODE_0)
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
        let header = [0x01, panel, line];
        buffer.extend_from_slice(&header);
        buffer.extend_from_slice(&data[..]);

        self.spi.write(&buffer).unwrap();
    }

    pub fn write_panel(&mut self, panel: u8, data: &[u8; PANEL_BUFFER_SIZE]) {
        for line in 0..PANEL_LINE_COUNT {
            let start = line * PANEL_LINE_SIZE;
            let end = start + LINE_BUFFER_SIZE;
            let line_data: [u8; LINE_BUFFER_SIZE] = data[start..end];
            self.write_line(panel, line as u8, &line_data);
        }
    }

    pub fn write_image(_data: i8) {

    }

    pub fn flush(&mut self) {
        self.spi.write(&[0x08]).unwrap();
    }
}