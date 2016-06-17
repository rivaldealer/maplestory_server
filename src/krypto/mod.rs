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

pub fn get_length(header: [u8;4]) -> usize {
    let zero  = header[0] as usize;
    let one   = header[1] as usize;
    let two   = header[2] as usize;
    let three = header[3] as usize;
    return (zero + one * 0x100) ^ (two + three * 0x100);
}
// All of the encryption / decryption is reused from titan and mss-rs initial commit - credits to koolk & calc0000
pub fn decrypt(buffer: &mut Vec<u8>, size: usize, iv_recv: [u8;4]) {
    // do decrypt of ofb block cipher
    // needs to be named decrypt_ofb eventually
    let b_size = size;
    //let key_size : i32 = 32;

    // AES input/output
    //let mut cipher_text : [u8;16] = [0x00;16];
    let mut input       : [u8;16] = [0x00;16];
    let mut output      : [u8;16] = [0x00;16];
    let mut plain_text  : [u8;16] = [0x00;16];
    let mut iv          : [u8;16] = [0x00;16];

    for x in 0..16 {
        iv[x] = iv_recv[x%4];
    }

    // AES key
    let key : [u8;32] = [ 0x13, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06,
                          0x00, 0x00, 0x00, 0xB4, 0x00, 0x00, 0x00, 0x1B, 0x00,
                          0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00,
                          0x00, 0x52, 0x00, 0x00, 0x00 ];

    let mut first_round : bool = true;
    let mut j : usize = 0;
    while j < b_size / 16 + 1 {
        if first_round {
            aes_encrypt(&iv, &mut output, &key);
            first_round = false;
        } else {
            aes_encrypt(&input, &mut output, &key);
        }

        for x in 0..16 {
            plain_text[x] = output[x] ^ buffer[j * 16 + x];
        }
        // big thanks to calc0000 for these three loops
        if j == b_size/16 {
            for x in 0..b_size % 16 {
                buffer[(j * 16) + x] = plain_text[x];
            }
        } else {
            for x in 0..16 {
                buffer[(j * 16) + x] = plain_text[x];
            }
        }
        for x in 0..16 {
            input[x] = output[x];
        }
        j += 1;
    }





    //next_iv(ivRecv);
    //maple_decrypt(buffer, size);
}

fn aes_encrypt(iv: &[u8], output: &mut [u8], key: &[u8;32]) {
    let rounds              : u32      = 14;
    let mut expanded_key    : [u8;240] = [0x00;240];
    let mut block           : [u8;16]  = [0x00;16];

    // iterate over the columns
    for x in 0..4 {
        // iterate over the rows
        for y in 0..4 {
            block[x + (y * 4)] = iv[(x * 4) + y];
        }
    }
    expand_key(&mut expanded_key, key);
    aes_main(&mut block, &expanded_key, rounds);

    // unwrap the block again into the output
    for x in 0..4 {
        // iterate over the rows
        for y in 0..4 {
            output[(x * 4) + y] = block[x + (y * 4)];
        }
    }
}

fn aes_main() { // soonTM

}

fn expand_key(expanded_key: &mut [u8;240], key: &[u8;32]) {
    let expanded_key_size = 240;

    for x in 0..key.len() {
        expanded_key[x] = key[x];
    }

    let mut current_size = key.len();
    let mut rcon_iteration = 1;

    while current_size < expanded_key_size {
        let mut t: [u8;4] = [0x00;4];

        // assign the previous 4 bytes to the temp value t
        for x in 0..4 {
            t[x] = expanded_key[(current_size - 4) + x];
        }

        // every 16, 24, and 32 bytes we apply the core schedule to t
        // and increment rcon_iteration afterwards
        if current_size % key.len() == 0 {
            //core(&mut t, rcon_iteration);
            rcon_iteration += 1;
        }

        // for 246-bit keys, we add an extra sbox to the calculation
        if key.len() == 32 && (current_size % key.len()) == 16 {
            for x in 0..4 {
                //t[x] = get_sbox_value(t[x]);
            }
        }

        // we xor t with the four-byte block 16, 24, 32 bytes before the new
        // expanded key. This becomes the next four bytes in the expanded key
        for x in 0..4 {
            expanded_key[current_size] = expanded_key[current_size - key.len()] ^ t[x];
            current_size += 1;
        }
    }
}
