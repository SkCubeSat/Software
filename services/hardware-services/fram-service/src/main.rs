use rust_i2c::{Command, Connection};
use linux_embedded_hal::I2cdev;
use embedded_hal::i2c::I2c;

const I2C_BUS: &str = "/dev/i2c-2";
const FRAM_ADDR: u8 = 0x50;
const SOLAR_PANEL_MEMORY: [u8; 4] = [0x01, 0x02, 0x03,0x04];
const ATTENNA_MEMORY: [u8; 4] = [0x05,0x06,0x07,0x08];
const ERROR_MEMORY: [u8; 4] = [0x00,0x00,0x00,0x00];

enum Checks {
    SolarPanels,
    Attenna,
}

fn write_fram(bus: &mut I2cdev, address: u8, data: u8){
    match bus.write(FRAM_ADDR, &[address, data]){
        Ok(_) => println!(""),
        Err(e) => println!(""),
    };
}


fn read_fram(bus: &mut I2cdev, mem_loc: u8) -> u8{
    match bus.read(FRAM_ADDR, &[mem_loc]){
        Ok(_) => return bus.read(FRAM_ADDR, &[mem_loc]),
        Err(e) => return 0x00,
    };

}

fn write_status(arr: [u8;4],status:bool){
    let mut data: u8;

    if status{
        data = 0x01
    }
    else{
        data = 0x00
    }
    
    for location in arr{
        write_fram(i2c, location, data)
    }

}

fn getaddresses(part_to_address: Checks) -> Result<[u8;4],String>{
    let address: [u8;4];

    match part_to_address {
        Checks::SolarPanels => address = SOLAR_PANEL_MEMORY,
        
        Checks::Attenna => address = ATTENNA_MEMORY,
        
        _ => return Err("Cannot get address".to_string()),
    };
    Ok(
    address
    )
        
}

fn check_sum(arr: [u8; 4]) -> u8{
    let mut equal: u8 = 0;
    let mut non_equal: u8 = 0;
    let mut first: u8 = read_fram(arr[0]);
    let mut data: u8;
    
    for address in arr{
        
        data = read_fram(address);
        

        if first == data{
            equal += 1;
        }
        else {
            non_equal += 1;
        }

    }

    if equal >= non_equal{

        equal/non_equal

    }
    else{

        non_equal/equal   
    
    }

}    

fn main(){
    let mut i2c = match I2cdev::new(I2C_BUS){
        Ok(_) => println!("BANG"),
        Err(e) => println!("{:?}",e),
    };

}
