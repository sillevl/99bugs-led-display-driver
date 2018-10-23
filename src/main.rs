mod display;
use display::Display;
use display::PANEL_BUFFER_SIZE;



fn main() {
    let mut display = Display::new("/dev/spidev0.0");
    let data: [u8; PANEL_BUFFER_SIZE] = [0x00; PANEL_BUFFER_SIZE];
    
    display.write_panel(0,&data);
    display.flush();
    println!("Done...");
}

