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

use std::net::TcpStream;
use std::io::prelude::*;
use rand::{thread_rng, Rng};
use krypto;

pub fn send_handshake(mut stream: &TcpStream) {
    // little endian format
    let packet_size : [u8;2] = [ 0x0D, 0x00 ];
    let version     : [u8;2] = [ 0x37, 0x00 ];
    let mut iv_recv : [u8;4] = [ 0x00, 0x00, 0x00, 0x00 ];
    let mut iv_send : [u8;4] = [ 0x00, 0x00, 0x00, 0x00 ];
    let locale      : u8 = 0x08;

    // generate random initialization vectors
    thread_rng().fill_bytes(&mut iv_recv);
    thread_rng().fill_bytes(&mut iv_send);

    // just making sure the bytes are truely random for each client
    //println!("send: {:?}.\nrecv: {:?}.", iv_send, iv_recv);

    // unfortunately stream writer can only write in an array of u8s...I think
    let handshake : [u8;15] = [ packet_size[0], packet_size[1], version[0],
                                version[1], 0x00, 0x00, iv_recv[0], iv_recv[1],
                                iv_recv[2], iv_recv[3], iv_send[0], iv_send[1],
                                iv_send[2], iv_send[3], locale ];
    match stream.write(&handshake) {
        Ok(bytes) => println!("Sent handshake; {} bytes.", bytes),
        Err(err) => {
            println!("Error occured while sending handshake. Error: {}", err);
            drop(&stream);
        },
    }

    //let mut packet : [u8;10000] = [0x00;10000];
    //let mut packet_vec: Vec<u8> = Vec::new();
    const HEADER_LEN : usize = 4;
    let mut buffer_bytes = 0;
    let mut header : [u8;HEADER_LEN] = [0x00;HEADER_LEN];

    loop {
        while buffer_bytes < 2 {
            // read header
            if buffer_bytes < HEADER_LEN {
                match stream.read_exact(&mut header[..]) {
                    Ok(_) => {
                        if header.len() <= 0 {
                            drop(&stream);
                        }
                        buffer_bytes += header.len();
                        //println!("headerlen {}", buffer_bytes);
                        //return;
                    },
                    Err(err) => {
                        println!("Error: {}", err);
                        drop(&stream);
                    },
                };
            }
            if buffer_bytes >= HEADER_LEN {
                let packet_size : usize = krypto::get_length(header);
                println!("packet_size: {}", packet_size);
                let mut buffer : Vec<u8> = vec![0x00; packet_size];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        // 1 CALL TO RULE THEM ALL Kappa
                        let buffer_len = buffer.len();
                        if buffer_len <= 0 {
                            println!("Error: buffer length is {}", buffer_len);
                            drop(&stream);
                            return;
                        }
                        buffer_bytes += bytes_read;
                        // print buffer contents
                        //for x in 0..buffer_len {
                        //    println!("{} ", buffer[x]);
                        //}
                        if buffer_bytes == packet_size + HEADER_LEN {
                            krypto::decrypt(&mut buffer, packet_size, &iv_recv);
                        }
                    },
                    Err(err) => {
                        println!("Error: {}", err);
                        drop(&stream);
                        return;
                    },
                }
            }
        }
    }
}
