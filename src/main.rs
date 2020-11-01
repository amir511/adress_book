// use sqlite::{Connection, Result};
mod types;
use std::env;
use types::AddressRecord;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut name = String::from("");
    let mut mobile = String::from("");
    let mut address = String::from("");
    let mut good_arguments = true;
    for arg in &args[1..] {
        if arg.starts_with("--name=") {
            name = arg[7..].to_string();
        } else if arg.starts_with("--mobile=") {
            mobile = arg[10..].to_string();
        } else if arg.starts_with("--address=") {
            address = arg[10..].to_string();
        } else{
            println!("Bad Arguments!");
            good_arguments = false;
        }
    }
    if good_arguments{
        let address_record =  AddressRecord::new(name, mobile, address);
        println!("Name: {}", address_record.name);
        println!("Mobile: {}", address_record.mobile);
        println!("Address: {}", address_record.address);

    }
}
