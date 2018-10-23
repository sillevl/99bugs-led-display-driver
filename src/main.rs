mod display;
use display::Display;
use display::PANELLINEBUFFERSIZE;



fn main() {
    let mut display = Display::new("/dev/spidev0.0");
    let data: [u8; PANELLINEBUFFERSIZE] = [0x00; PANELLINEBUFFERSIZE];
    
    display.write_line(0,0,&data);
    display.flush();
    println!("Done...");
}

