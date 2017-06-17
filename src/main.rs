extern crate serial;

use std::env;
use std::io;
use std::time::Duration;

use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    let mut port = serial::open("COM6").unwrap();
        interact(&mut port).unwrap();
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.set_timeout(Duration::from_millis(1000)));

    let mut buf: Vec<u8> = (0..255).collect();

    try!(port.write(&buf[..]));
    try!(port.read(&mut buf[..]));

    println!("Reading");
    println!("{:?}",buf );

    let mut buf2: Vec<u8> = (0..1).collect();
    loop{
        try!(port.read(&mut buf2));
        println!("{:?}",&buf2)
    }

    Ok(())
}