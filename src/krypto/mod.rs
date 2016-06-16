

pub fn get_length(header: [u8;4]) -> usize {
    let zero  = header[0] as usize;
    let one   = header[1] as usize;
    let two   = header[2] as usize;
    let three = header[3] as usize;
    return (zero + one * 0x100) ^ (two + three * 0x100);
}

pub fn decrypt(buffer: &mut Vec<u8>, size: usize, iv_recv: &[u8;4]) {
    // do decrypt of ofb block cipher
    let b_size = size;
    let key_size : i32 = 32;

    // AES input/output
    //let mut cipher_text : [u8;16] = [0x00;16];
    let mut input       : [u8;16] = [0x00;16];
    let output          : [u8;16] = [0x00;16];
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
            aes_encrypt(iv, output, key, key_size);
            first_round = false;
        } else {
            aes_encrypt(input, output, key, key_size);
        }

        for x in 0..16 {
            plain_text[x] = output[x] ^ buffer[j * 16 + x];
        }

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

fn aes_encrypt(iv: [u8;16], output: [u8;16], key: [u8;32], key_size: i32) {
    //
}
