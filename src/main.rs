mod display;
use display::Display;
use display::LINE_BUFFER_SIZE;



fn main() {
    let mut display = Display::new("/dev/spidev0.0");
    let data: [u8; LINE_BUFFER_SIZE] = [0x00; LINE_BUFFER_SIZE];
    
    display.write_line(0,0,&data);
    display.flush();
    println!("Done...");
}

