use rust_i2c::{Command, Connection};
use linux_embedded_hal::I2cdev;
use embedded_hal::i2c::I2c;

const I2C_BUS: &str = "/dev/i2c-0";
const FRAM_ADDR: u8 = 0x50;
const SOLAR_PANEL_MEMORY: [i32; 4] = [0x01, 0x02, 0x03,0x04];
const ATTENNA_MEMORY: [i32; 4] = [0x05,0x06,0x07,0x08];

enum Checks {
    SolarPanels,
    Attenna,
}

fn write_fram(bus: &mut I2cdev){
    match bus.write(FRAM_ADDR, &[0x10, 0x01]){
        Ok(_) => println!("ASGJHSGKAJHKSJAHGKJSHGKJA"),
        Err(e) => println!("FUCK THIS SHIT"),
    };
}


fn read_fram(){

    println!("GA")    


}

fn write_status(arr: &[i32;4]){
    


}

fn read_status(){
    

}

fn getaddresses(Part_to_Address: Checks){
    match Part_to_Address {
        Checks::SolarPanels => write_status(&SOLAR_PANEL_MEMORY),
        
        Checks::Attenna => write_status(&ATTENNA_MEMORY),

    }

}

fn main(){
    let mut i2c = match I2cdev::new(I2C_BUS){
        Ok(_) => println!("BANG"),
        Err(e) => println!("{:?}",e),
    };

}
