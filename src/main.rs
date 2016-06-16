/*  This file is part of TitanRS.
    Copyright (C) 2016  rivaldealer <rivaldealer@cocaine.ninja>

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
use std::thread;

extern crate rand;

mod krypto;
mod maplepacket;

fn main() {
    match TcpListener::bind("127.0.0.1:8484") {
        Ok(listen) => {
            println!("Server listening on port 8484.");
            start(listen);
        },
        Err(err) => println!("Unable to bind to port 8484. Error: {}", err),
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
            Err(err) => {
                println!("Failed to accept connection {}", err);
                drop(&listen);
            },
        }
    }
    drop(listen);
}

fn handle_client(stream: TcpStream) {
    match stream.peer_addr() {
        Ok(peer) => println!("Peer {} connected to server.", peer),
        Err(err) => {
            println!("Error: {}", err);
            drop(&stream);
        },
    }
    maplepacket::send_handshake(&stream);
    drop(&stream);

    // send handshake
}
