extern crate chrono;

mod display;
use display::Display;
use display::FRAME_BUFFER_SIZE;


use chrono::prelude::*;

use std::thread;



fn main() {
    // benchmark();
    let mut display = Display::new("/dev/spidev0.0");
    let data: [u8; FRAME_BUFFER_SIZE] = [0xFF; FRAME_BUFFER_SIZE];
    let data2: [u8; FRAME_BUFFER_SIZE] = [0x00; FRAME_BUFFER_SIZE];
    let short_data: [u8; 100] = [0xFF; 100]; 
    display.write_frame(&data);
    display.flush();
    thread::sleep_ms(500);
    display.write_frame(&data2);
    display.flush();
    thread::sleep_ms(500);
    display.write_frame(&data);
    display.flush();
    thread::sleep_ms(500);
    display.write_frame(&data2);
    display.flush();
    thread::sleep_ms(500);
    println!("Done...");
}

fn benchmark() {
    let mut display = Display::new("/dev/spidev0.0");
    let data: [u8; FRAME_BUFFER_SIZE] = [0x00; FRAME_BUFFER_SIZE];
    let data2: [u8; FRAME_BUFFER_SIZE] = [0xFF; FRAME_BUFFER_SIZE];

    const REPS: u32 = 100;

    let mut counter = 0;

    let start = get_unix_timestamp_ms();

    for x in 0..REPS {
        display.write_frame(&data2);
        display.flush();    
        counter +=1;

        thread::sleep_ms(500);

        display.write_frame(&data);
        display.flush();
        counter += 1;

        thread::sleep_ms(500);
    }

    let end = get_unix_timestamp_ms();
    
    println!("Time per frame: {:?} (repetitions {:?})", (end - start) / counter, counter);

    println!("Done...");
}

pub fn get_unix_timestamp_ms() -> i64 {
    let now = Utc::now();
    let seconds: i64 = now.timestamp();
    let nanoseconds: i64 = now.nanosecond() as i64;
    (seconds * 1000) + (nanoseconds / 1000 / 1000)
}
