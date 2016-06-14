/*  This file is part of TitanRS.
    Copyright (C) 2016  rivaldealer <comedowntous@outlook.com>

    TitanRS is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    TitanRS is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with TitanRS.  If not, see <http://www.gnu.org/licenses/>.*/

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread;


fn main() {
    match TcpListener::bind("127.0.0.1:8484") {
        Ok(listen) => {
            println!("Server listening on port 8484.");
            start(listen);
        },
        Err(e) => println!("unlucky. {}", e),
    }
}

fn start(listen: TcpListener) {
    for stream in listen.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                    });
            }
            Err(err) => println!("Failed to accept connection {}", err),
        }
    }
    drop(listen);
}

fn handle_client(mut stream: TcpStream) {
    // without packet size at beginning
    // TODO: Create packet wrapper of stream write and read which includes sorting endianess
    let handshake : [u8;16] = [ 0x0D, 0x00, 0x37, 0x00, 0x00, 0x00, 0x7A, 0x05, 0xBE, 0x0A, 0x81, 0x45, 0xFC, 0x52, 0x08, 0x00 ];
    match stream.peer_addr() {
        Ok(peer) => println!("Peer {} connect to server.", peer),
        Err(err) => println!("Error occured while trying to solve peer address. Error: {}", err),
    }
    match stream.write(&handshake) {
        Ok(bytes) => println!("Sent handshake; {} bytes.", bytes),
        Err(err) => println!("Error occured while sending handshake. Error: {}", err),
    }
    //println!("sent {} bytes", data);
    let mut recv = String::new();
    match stream.read_to_string(&mut recv) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }

    // send handshake
}
