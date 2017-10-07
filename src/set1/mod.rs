use std::collections::LinkedList;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::string::FromUtf8Error;


pub fn hex_to_base64(hex: &str) -> String
{
    let b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let mut buf = [0; 3];
    let mut result = String::new();
    let mut i = 0;
    let hexb = hex.as_bytes();
    while i < hexb.len() {
        for j in 0 .. 3 {
            for k in 0 .. 2 {
                let num = hex_as_decimal(hexb[i + j*2 + k] as char).unwrap();
                buf[j] = (buf[j] << 4) | num;
            }
        }
        result.push(b64[(buf[0] >> 2) as usize] as char);
        result.push(b64[(((buf[0] & 0b0000_0011) << 4) + (buf[1] >> 4)) as usize] as char);
        result.push(b64[(((buf[1] & 0b0000_1111) << 2) + (buf[2] >> 6)) as usize] as char);
        result.push(b64[(buf[2] & 0b0011_1111) as usize] as char);
        buf = [0; 3];
        i += 6;
    }
    result
}

pub fn base64_decode(string: &str) -> Vec<u8> {
    let b64 = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=");
    let mut buf : [u8; 4] = [0; 4];
    let mut padding_begin_index = string.len() - 1;
    while string.as_bytes()[padding_begin_index] as char == '=' {
        padding_begin_index -= 1;
    }
    let mut result : Vec<u8> = vec![0; ((padding_begin_index + 1) as f64 * 3.0 / 4.0) as usize];
    for (v, vo) in string[0..padding_begin_index + 1].as_bytes().chunks(4).zip(result.as_mut_slice().chunks_mut(3)) 
    {
        fn quad2triplet(buf: [u8; 4]) -> [u8; 3] {
            [(buf[0] << 2) + (buf[1] >> 4), (buf[1] << 4) + (buf[2] >> 2), (buf[2] << 6) + buf[3]]
        }

        for i in 0..buf.len()
        {
            buf[i] = match v.get(i) {
                Some(x) => match b64.find(*x as char) {
                    Some(ind) => ind as u8,
                    None => 0
                },
                None => 0
            };
        }
        let volen = vo.len();
        vo.copy_from_slice(&quad2triplet(buf)[0..volen]);
    }
    result
}

pub fn base64_encode(bytes: &[u8]) -> String {
    let b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".as_bytes();
    let mut buf : [u8; 3] = [0, 0, 0];
    let mut result : Vec<u8> = vec![0; (bytes.len() as f64 * 4.0 / 3.0).ceil() as usize];
    for (v, vo) in bytes.chunks(3).zip(result.as_mut_slice().chunks_mut(4)) 
    {
        fn triplet2quad(buf: [u8; 3]) -> [u8; 4] {
            [buf[0] >> 2, ((buf[0] & 0b0000_0011) << 4) + (buf[1] >> 4), ((buf[1] & 0b0000_1111) << 2) + (buf[2] >> 6), buf[2] & 0b0011_1111]
        }

        for i in 0..buf.len()
        {
            buf[i] = match v.get(i) {
                Some(x) => *x,
                None => 0
            };
        }
        let volen = vo.len();
        vo.copy_from_slice(&triplet2quad(buf)[0..volen]);
    }
    let padding_length = (4 - result.len() % 4) % 4;
    for _ in 0 .. padding_length {
        result.push(64);
    }
    return String::from_utf8(result.iter().map(|x| b64[*x as usize]).collect()).unwrap();
}


// convert 0xc to 12, can fail nicely
pub fn hex_as_decimal(ch : char) -> Result<u8, String> {
    match ch {
        '0' ... '9' => Ok((ch as u8) - ('0' as u8)),
        'a' ... 'f' => Ok(10 + (ch as u8) - ('a' as u8)),
        'A' ... 'F' => Ok(10 + (ch as u8) - ('A' as u8)),
        _ => Err(format!("Invalid hexadecimal character: {}", ch))
    }
}

// convert hex like 'abc' to a byte vector: [0x0a, 0xbc]
pub fn decode_hex(buf: &str) -> Vec<u8> {
    let mut result = LinkedList::new();
    let mut num;
    let mut chars = buf.chars().rev();
    while let Some(c1) = chars.next() {
        num = hex_as_decimal(c1).unwrap();
        if let Some(c2) = chars.next() {
            num += hex_as_decimal(c2).unwrap() << 4
        }
        result.push_front(num);
    }
    Vec::from_iter(result.into_iter())
}

// convert a byte vector from decode_hex to a hexadecimal string
pub fn encode_hex(buf: &[u8]) -> String
{
    let hex = "0123456789abcdef".as_bytes();
    let mut result = String::new();
    // if we have a byte that is 0x0a, ignore the 0. decoding makes sure we don't have multiple leading zeroes.
    match buf[0] >> 4 {
        0 => {},
        _ => result.push(hex[(buf[0] >> 4) as usize] as char),
    };
    result.push(hex[(buf[0] & 0b0000_1111) as usize] as char);
    for i in 1 .. buf.len() { // starts at 1 (instead of 0, cause first iteration was done manually)
        result.push(hex[(buf[i] >> 4) as usize] as char);
        result.push(hex[(buf[i] & 0b0000_1111) as usize] as char);
    }
    result
}

pub fn string_from_hex(buf: &str) -> Result<String, FromUtf8Error> {
    String::from_utf8(decode_hex(buf))
}

// xor two equal-length buffers
pub fn fixed_xor(buf1 : &[u8], buf2 : &[u8]) -> Vec<u8>
{
    if buf1.len() != buf2.len() {
        panic!("Buffers are not the same length");
    }
    let mut v = Vec::new();
    for i in 0 .. buf1.len() {
        v.push(buf1[i] ^ buf2[i]);
    }
    v
}


// xor every byte in the buffer with the single byte.
pub fn single_byte_xor(buf: &Vec<u8>, byte: u8) -> Vec<u8>
{
    buf.iter().map(|c| c ^ byte).collect()
}

pub fn char_frequency(buf: &[u8]) -> HashMap<char, f64> {
    let mut result = HashMap::new();
    for i in 0 .. buf.len() {
        *result.entry(buf[i] as char).or_insert(0.0) += 1.0;
    }
    for (_, val) in result.iter_mut() {
        // converting all values from count to frequency (in one go instead of per character per update)
        *val /= buf.len() as f64;
    }
    result
}

pub fn decrypt_single_byte_xor_english(ciphertext: &[u8]) -> (Option<String>, u8, f64) {
    let (mut min_chi_squared, mut min_key, mut decryption) = (similarity_to_english(ciphertext), 0, ciphertext.to_owned());

    'outer: for i in 1 .. 255 {
        let attempted_decryption = single_byte_xor(&ciphertext.to_owned(), i);
        'inner: for ch in &attempted_decryption {
            if *ch > 127 {
                continue 'outer;
            }
        }
        let chi_squared = similarity_to_english(&attempted_decryption);
        if chi_squared < min_chi_squared {
            min_chi_squared = chi_squared;
            min_key = i;
            decryption = attempted_decryption;
        }
    }
    let plaintext_option = match String::from_utf8(decryption) {
        Ok(plaintext) => Some(plaintext),
        Err(_) => None
    };
    (plaintext_option, min_key, min_chi_squared)
}

pub fn similarity_to_english(buf: &[u8]) -> f64 {
    let frequency_table = vec![
        (9, 0.000057), (10, 0.020827), (13, 0.020827), (32, 0.171662), (33, 0.000072), (34, 0.002442), (35, 0.000179),
        (36, 0.000561), (37, 0.000160), (38, 0.000226), (39, 0.002447), (40, 0.002178),
        (41, 0.002233), (42, 0.000628), (43, 0.000215), (44, 0.007384), (45, 0.013734),
        (46, 0.015124), (47, 0.001549), (48, 0.005516), (49, 0.004594), (50, 0.003322),
        (51, 0.001847), (52, 0.001348), (53, 0.001663), (54, 0.001153), (55, 0.001030),
        (56, 0.001054), (57, 0.001024), (58, 0.004354), (59, 0.001214), (60, 0.001225),
        (61, 0.000227), (62, 0.001242), (63, 0.001474), (64, 0.000073), (65, 0.003132),
        (66, 0.002163), (67, 0.003906), (68, 0.003151), (69, 0.002673), (70, 0.001416),
        (71, 0.001876), (72, 0.002321), (73, 0.003211), (74, 0.001726), (75, 0.000687),
        (76, 0.001884), (77, 0.003529), (78, 0.002085), (79, 0.001842), (80, 0.002614),
        (81, 0.000316), (82, 0.002519), (83, 0.004003), (84, 0.003322), (85, 0.000814),
        (86, 0.000892), (87, 0.002527), (88, 0.000343), (89, 0.000304), (90, 0.000076),
        (91, 0.000086), (92, 0.000016), (93, 0.000088), (94, 0.000003), (95, 0.001159),
        (96, 0.000009), (97, 0.051880), (98, 0.010195), (99, 0.021129), (100, 0.025071),
        (101, 0.085771), (102, 0.013725), (103, 0.015597), (104, 0.027444), (105, 0.049019),
        (106, 0.000867), (107, 0.006753), (108, 0.031750), (109, 0.016437), (110, 0.049701),
        (111, 0.057701), (112, 0.015482), (113, 0.000747), (114, 0.042586), (115, 0.043686),
        (116, 0.063700), (117, 0.020999), (118, 0.008462), (119, 0.013034), (120, 0.001950),
        (121, 0.011330), (122, 0.000596), (123, 0.000026), (124, 0.000007), (125, 0.000026),
        (126, 0.000003), (131, 0.0), (149, 0.006410), (183, 0.000010), (223, 0.0),
        (226, 0.0), (229, 0.0), (230, 0.0), (237, 0.0), 
    ];

    let mut eng_freq = HashMap::new();
    for &(c, f) in &frequency_table {
        eng_freq.insert(c, f);
    }
    let buf_freq = char_frequency(buf);
    // Pearson's chi-squared test:
    let mut chi_squared : f64 = 0.0;
    for (&c, &f) in buf_freq.iter() {
        let expected_f = match eng_freq.get(&(c as u8)) {
            Some(&freq) => freq,
            None => 0.0
        };        
        chi_squared += (f - expected_f).powi(2) / expected_f;
    }
    chi_squared
}

pub fn repeating_key_xor(plaintext: &str, key: &str) -> String {
    let mut ciphertext = String::new();
    let mut key_index = 0;
    for c in plaintext.chars() {
        ciphertext.push((c as u8 ^ key.as_bytes()[key_index]) as char);
        key_index = (key_index + 1) % key.len();
    }
    ciphertext
}