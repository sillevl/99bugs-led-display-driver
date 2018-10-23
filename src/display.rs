extern crate spidev;
use std::io::prelude::*;
use self::spidev::{Spidev, SpidevOptions, SPI_MODE_0};
use std::usize;

pub const PANELLINESIZE: usize = 32;
pub const PANELLINECOUNT: usize = 32;
pub const PANELCOUNT: usize = 6;
pub const LEDSPERPIXEL: usize = 3;

pub const PANELLINEBUFFERSIZE: usize = PANELLINESIZE * LEDSPERPIXEL;

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

    pub fn write_line(&mut self, panel: u8, line: u8, data: &[u8; PANELLINEBUFFERSIZE]) {
        const HEADER_SIZE: usize = 3;
        
        let mut buffer = Vec::with_capacity(HEADER_SIZE + PANELLINEBUFFERSIZE);
        let header = [0x01, panel, line];
        buffer.extend_from_slice(&header);
        buffer.extend_from_slice(&data[..]);

        self.spi.write(&buffer).unwrap();
    }

    pub fn write_image(_data: i8) {

    }

    pub fn flush(&mut self) {
        self.spi.write(&[0x08]).unwrap();
    }
}