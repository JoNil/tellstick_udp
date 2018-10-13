extern crate byteorder;

use std::net::UdpSocket;
use std::io::Cursor;
use std::process::Command;
use byteorder::{BigEndian, ReadBytesExt};

const MAGIC: u32 = 0xae4fa983;

fn main() -> std::io::Result<()> {
        
	let socket = UdpSocket::bind("0.0.0.0:6319")?;

	let mut state = 0;

	loop {
	
        	let mut buf = [0; 10];
	        let (amt, _) = socket.recv_from(&mut buf)?;

		if amt != 6 {
			continue;
		} 

		let mut rdr = Cursor::new(&buf);

		if rdr.read_u32::<BigEndian>().unwrap() != MAGIC {
			continue;
		}

		let device = rdr.read_u8().unwrap();
		let mut on_off = rdr.read_u8().unwrap();

		if on_off == 2 {
			if state == 0 {
				on_off = 1;
			} else {
				on_off = 0;
			}
		}

		if on_off == 1 {
			Command::new("tdtool")
				.arg("--on")
				.arg(device.to_string())
				.output()
				.unwrap();
			state = 1;
		} else {
			Command::new("tdtool")
				.arg("--off")
				.arg(device.to_string())
				.output()
				.unwrap();
			state = 0;
		}
	}
}
