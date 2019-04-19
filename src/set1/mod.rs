use std::collections::HashMap;
use std::collections::LinkedList;
use std::f64;
use std::iter::FromIterator;
use std::string::FromUtf8Error;

pub fn hex_to_base64(hex: &str) -> String {
    let b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let mut buf = [0; 3];
    let mut result = String::new();
    let mut i = 0;
    let hexb = hex.as_bytes();
    while i < hexb.len() {
        for j in 0..3 {
            for k in 0..2 {
                let num = hex_as_decimal(hexb[i + j * 2 + k] as char).unwrap();
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

pub fn byte_hamming_distance(byte1: u8, byte2: u8) -> u8 {
    (byte1 ^ byte2).count_ones() as u8
}

// hamming distance in bits between these two byte slices (must be equal sized)
pub fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> usize {
    let mut hamming_distance: usize = 0;
    assert_eq!(bytes1.len(), bytes2.len());
    for i in 0..bytes1.len() {
        hamming_distance += byte_hamming_distance(bytes1[i], bytes2[i]) as usize;
    }

    hamming_distance
}

pub fn normalized_hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> f64 {
    hamming_distance(bytes1, bytes2) as f64 / bytes1.len() as f64
}

pub fn base64_decode(string: &str) -> Vec<u8> {
    fn quad2triplet(buf: [u8; 4]) -> [u8; 3] {
        [
            (buf[0] << 2) + (buf[1] >> 4),
            (buf[1] << 4) + (buf[2] >> 2),
            (buf[2] << 6) + buf[3],
        ]
    }

    let b64 = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let mut buf: [u8; 4] = [0; 4];
    let mut padding_count = 3;
    let string_len = string.as_bytes().len();
    const MAX_PADDING_COUNT: usize = 3;

    for i in string_len - MAX_PADDING_COUNT..string_len {
        if string.as_bytes()[i] as char == '=' {
            break;
        } else {
            padding_count -= 1;
        }
    }

    let real_string_len = string_len - padding_count;
    let mut result: Vec<u8> = vec![0; (real_string_len as f64 * 3.0 / 4.0) as usize];
    for (quad, out_triplet) in string[0..real_string_len]
        .as_bytes()
        .chunks(4)
        .zip(result.as_mut_slice().chunks_mut(3))
    {
        for i in 0..buf.len() {
            buf[i] = match quad.get(i) {
                Some(x) => match b64.find(*x as char) {
                    Some(ind) => ind as u8,
                    None => 0,
                },
                None => 0,
            }
        }
        // TODO: remove this variable when NLL is live
        let output_triplet_len = out_triplet.len();
        out_triplet.copy_from_slice(&quad2triplet(buf)[0..output_triplet_len]);
    }
    result
}

pub fn base64_encode(bytes: &[u8]) -> String {
    let b64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".as_bytes();
    let mut buf: [u8; 3] = [0, 0, 0];
    let mut result: Vec<u8> = vec![0; (bytes.len() as f64 * 4.0 / 3.0).ceil() as usize];
    for (v, vo) in bytes.chunks(3).zip(result.as_mut_slice().chunks_mut(4)) {
        fn triplet2quad(buf: [u8; 3]) -> [u8; 4] {
            [
                buf[0] >> 2,
                ((buf[0] & 0b0000_0011) << 4) + (buf[1] >> 4),
                ((buf[1] & 0b0000_1111) << 2) + (buf[2] >> 6),
                buf[2] & 0b0011_1111,
            ]
        }

        for i in 0..buf.len() {
            buf[i] = match v.get(i) {
                Some(x) => *x,
                None => 0,
            };
        }
        let volen = vo.len();
        vo.copy_from_slice(&triplet2quad(buf)[0..volen]);
    }
    let padding_length = (4 - result.len() % 4) % 4;
    for _ in 0..padding_length {
        result.push(64);
    }
    return String::from_utf8(result.iter().map(|x| b64[*x as usize]).collect()).unwrap();
}

// convert c to 12, can fail nicely
pub fn hex_as_decimal(ch: char) -> Result<u8, String> {
    match ch {
        '0'...'9' => Ok((ch as u8) - ('0' as u8)),
        'a'...'f' => Ok(10 + (ch as u8) - ('a' as u8)),
        'A'...'F' => Ok(10 + (ch as u8) - ('A' as u8)),
        _ => Err(format!("Invalid hexadecimal character: {}", ch)),
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
pub fn encode_hex(buf: &[u8]) -> String {
    let hex = "0123456789abcdef".as_bytes();
    let mut result = String::new();
    // if we have a byte that is 0x0a, ignore the 0. decoding makes sure we don't have multiple leading zeroes.
    match buf[0] >> 4 {
        0 => {}
        _ => result.push(hex[(buf[0] >> 4) as usize] as char),
    };
    result.push(hex[(buf[0] & 0b0000_1111) as usize] as char);
    for i in 1..buf.len() {
        // starts at 1 (instead of 0, cause first iteration was done manually)
        result.push(hex[(buf[i] >> 4) as usize] as char);
        result.push(hex[(buf[i] & 0b0000_1111) as usize] as char);
    }
    result
}

pub fn string_from_hex(buf: &str) -> Result<String, FromUtf8Error> {
    String::from_utf8(decode_hex(buf))
}

// xor two equal-length buffers
pub fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    if buf1.len() != buf2.len() {
        panic!("Buffers are not the same length");
    }
    let mut v = Vec::new();
    for i in 0..buf1.len() {
        v.push(buf1[i] ^ buf2[i]);
    }
    v
}

// xor every byte in the buffer with the single byte.
pub fn single_byte_xor(buf: &Vec<u8>, byte: u8) -> Vec<u8> {
    buf.iter().map(|c| c ^ byte).collect()
}

pub fn char_frequency(buf: &[u8]) -> HashMap<char, f64> {
    let mut result = HashMap::new();
    for i in 0..buf.len() {
        *result.entry(buf[i] as char).or_insert(0.0) += 1.0;
    }
    for (_, val) in result.iter_mut() {
        // converting all values from count to frequency (in one go instead of per character per update)
        *val /= buf.len() as f64;
    }
    result
}

pub fn decrypt_single_byte_xor_english(ciphertext: &[u8]) -> (Option<String>, u8, f64) {
    let (mut min_chi_squared, mut min_key, mut decryption) =
        (similarity_to_english(ciphertext), 0, ciphertext.to_owned());

    'outer: for i in 1..255 {
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
        Err(_) => None,
    };
    (plaintext_option, min_key, min_chi_squared)
}

pub fn similarity_to_english(buf: &[u8]) -> f64 {
    let frequency_table = vec![
        (9, 0.000057),
        (10, 0.020827),
        (13, 0.020827),
        (32, 0.171662),
        (33, 0.000072),
        (34, 0.002442),
        (35, 0.000179),
        (36, 0.000561),
        (37, 0.000160),
        (38, 0.000226),
        (39, 0.002447),
        (40, 0.002178),
        (41, 0.002233),
        (42, 0.000628),
        (43, 0.000215),
        (44, 0.007384),
        (45, 0.013734),
        (46, 0.015124),
        (47, 0.001549),
        (48, 0.005516),
        (49, 0.004594),
        (50, 0.003322),
        (51, 0.001847),
        (52, 0.001348),
        (53, 0.001663),
        (54, 0.001153),
        (55, 0.001030),
        (56, 0.001054),
        (57, 0.001024),
        (58, 0.004354),
        (59, 0.001214),
        (60, 0.001225),
        (61, 0.000227),
        (62, 0.001242),
        (63, 0.001474),
        (64, 0.000073),
        (65, 0.003132),
        (66, 0.002163),
        (67, 0.003906),
        (68, 0.003151),
        (69, 0.002673),
        (70, 0.001416),
        (71, 0.001876),
        (72, 0.002321),
        (73, 0.003211),
        (74, 0.001726),
        (75, 0.000687),
        (76, 0.001884),
        (77, 0.003529),
        (78, 0.002085),
        (79, 0.001842),
        (80, 0.002614),
        (81, 0.000316),
        (82, 0.002519),
        (83, 0.004003),
        (84, 0.003322),
        (85, 0.000814),
        (86, 0.000892),
        (87, 0.002527),
        (88, 0.000343),
        (89, 0.000304),
        (90, 0.000076),
        (91, 0.000086),
        (92, 0.000016),
        (93, 0.000088),
        (94, 0.000003),
        (95, 0.001159),
        (96, 0.000009),
        (97, 0.051880),
        (98, 0.010195),
        (99, 0.021129),
        (100, 0.025071),
        (101, 0.085771),
        (102, 0.013725),
        (103, 0.015597),
        (104, 0.027444),
        (105, 0.049019),
        (106, 0.000867),
        (107, 0.006753),
        (108, 0.031750),
        (109, 0.016437),
        (110, 0.049701),
        (111, 0.057701),
        (112, 0.015482),
        (113, 0.000747),
        (114, 0.042586),
        (115, 0.043686),
        (116, 0.063700),
        (117, 0.020999),
        (118, 0.008462),
        (119, 0.013034),
        (120, 0.001950),
        (121, 0.011330),
        (122, 0.000596),
        (123, 0.000026),
        (124, 0.000007),
        (125, 0.000026),
        (126, 0.000003),
        (131, 0.0),
        (149, 0.006410),
        (183, 0.000010),
        (223, 0.0),
        (226, 0.0),
        (229, 0.0),
        (230, 0.0),
        (237, 0.0),
    ];

    let mut eng_freq = HashMap::new();
    for &(c, f) in &frequency_table {
        eng_freq.insert(c, f);
    }
    let buf_freq = char_frequency(buf);
    // Pearson's chi-squared test:
    let mut chi_squared: f64 = 0.0;
    for (&c, &f) in buf_freq.iter() {
        let expected_f = match eng_freq.get(&(c as u8)) {
            Some(&freq) => freq,
            None => 0.0,
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

pub fn transpose_matrix(matrix: Vec<Vec<u8>>, columns: usize) -> Vec<Vec<u8>> {
    let mut new_matrix: Vec<Vec<u8>> = (0..columns).map(|_| Vec::with_capacity(columns)).collect();

    let rows = matrix.len();
    for i in 0..columns {
        for j in 0..rows {
            if matrix.len() > j && matrix[j].len() > i {
                new_matrix[i].push(matrix[j][i]);
            } else {
                continue;
            }
        }
    }
    new_matrix
}

// break repeating key xor
pub fn break_vigenere(ciphertext: &[u8]) -> String {
    const MIN_KEYSIZE: usize = 2;
    const MAX_KEYSIZE: usize = 40;
    const KEYSIZE_SAMPLES: usize = 50;

    let mut chosen_keysize = MIN_KEYSIZE;
    let mut min_hamming_distance = f64::MAX;

    for keysize in MIN_KEYSIZE..MAX_KEYSIZE {
        let ciphertext_keysize_samples: Vec<_> = ciphertext
            .chunks(keysize)
            .take(KEYSIZE_SAMPLES)
            .enumerate()
            .collect();
        let even_samples = ciphertext_keysize_samples
            .iter()
            .filter(|(index, _chunk)| index % 2 == 0);
        let odd_samples = ciphertext_keysize_samples
            .iter()
            .filter(|(index, _chunk)| index % 2 == 1);
        let sample_pairs = even_samples.zip(odd_samples);
        let average_hamming_distance_of_keysize = sample_pairs
            .map(|((_lenx, x), (_leny, y))| normalized_hamming_distance(x, y))
            .sum::<f64>()
            / KEYSIZE_SAMPLES as f64;
        /*println!(
            "average hamming distance of keysize {} is {:?}",
            keysize, average_hamming_distance_of_keysize
        );*/
        if average_hamming_distance_of_keysize < min_hamming_distance {
            min_hamming_distance = average_hamming_distance_of_keysize;
            chosen_keysize = keysize;
        }

        //println!("pairs: {:#?}", sample_pairs);
        //let average_hamming_distance = ciphertext_keysize_samples;
    }
    /*println!(
        "chosen keysize: {} with average hamming distance: {}",
        chosen_keysize, min_hamming_distance
    );
    for chunk in ciphertext.chunks(chosen_keysize) {
        println!("{:?}", chunk);
    }
    println!("Ciphertext length: {} bytes", ciphertext.len());
    */
    let transposed = transpose_matrix(
        ciphertext
            .chunks(chosen_keysize)
            .map(|chunk| chunk.to_vec())
            .collect(),
        chosen_keysize,
    );
    let transposed_rows = ciphertext.chunks(chosen_keysize).len();
    let _transposed_columns = chosen_keysize;

    //println!("{:?}", transposed);
    let decrypted_transposed: Vec<Vec<u8>> = transposed
        .iter()
        .map(|ciphertext| decrypt_single_byte_xor_english(ciphertext).0)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().as_bytes().into())
        .collect();

    let decrypted = transpose_matrix(decrypted_transposed, transposed_rows);

    let string_parts: Vec<String> = decrypted
        .iter()
        .map(|vec| String::from_utf8((*vec).clone()).unwrap())
        .collect();

    let solution = string_parts.concat();

    solution
}

pub mod aes {
    use nalgebra::Matrix4;
    type Block = Matrix4<u8>;
    const S_BOX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];
    const INVERSE_S_BOX: [u8; 256] = [
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7,
        0xfb, 0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde,
        0xe9, 0xcb, 0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42,
        0xfa, 0xc3, 0x4e, 0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49,
        0x6d, 0x8b, 0xd1, 0x25, 0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c,
        0xcc, 0x5d, 0x65, 0xb6, 0x92, 0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15,
        0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84, 0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7,
        0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06, 0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02,
        0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, 0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc,
        0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73, 0x96, 0xac, 0x74, 0x22, 0xe7, 0xad,
        0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e, 0x47, 0xf1, 0x1a, 0x71, 0x1d,
        0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, 0xfc, 0x56, 0x3e, 0x4b,
        0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4, 0x1f, 0xdd, 0xa8,
        0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f, 0x60, 0x51,
        0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, 0xa0,
        0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c,
        0x7d,
    ];

    pub fn round_constants(round_count: u8) -> Vec<u32> {
        (0..round_count).map(round_constant_i).collect()
    }

    fn round_constant_i(i: u8) -> u32 {
        fn round_constant_i_top_byte(i: u8) -> u8 {
            match i {
                0 => 1,
                _ if round_constant_i_top_byte(i - 1) < 0x80 => {
                    round_constant_i_top_byte(i - 1).wrapping_mul(2)
                }
                _ => ((round_constant_i_top_byte(i - 1) as u16).wrapping_mul(2) ^ 0x11b) as u8,
            }
        }
        (round_constant_i_top_byte(i) as u32) << 24
    }

    fn key_expansion(key: &[u8; 16], round_count: u8) -> Vec<Vec<u8>> {
        const N: usize = 4;
        let round_constants = round_constants(round_count);
        let mut round_keys: Vec<Vec<u8>> = Vec::new();
        let mut round_keys_u32: Vec<u32> = Vec::with_capacity((N as u8 * round_count) as usize);
        let mut key_in_32bit_words: Vec<u32> = Vec::with_capacity(4);
        for i in (0..key.len()).step_by(4) {
            key_in_32bit_words.push(u32::from_be_bytes(*arrayref::array_ref!(
                key[i..i + 4],
                0,
                4
            )));
        }
        for i in 0..round_keys_u32.len() {
            round_keys_u32.push(match i {
                i if i < N => key_in_32bit_words[i],
                i if i >= N && (i % N == 0) => {
                    round_keys_u32[i - N]
                        ^ sub_word(round_keys_u32[i - 1]).rotate_left(1)
                        ^ round_constants[i / N]
                }
                i if i >= N && N > 6 && (i % N == 4) => {
                    round_keys_u32[i - N] ^ sub_word(round_keys_u32[i - 1])
                }
                _ => round_keys_u32[i - N] ^ round_keys_u32[i - 1],
            });
        }
        for i in 0..round_keys_u32.len() {}
        round_keys
    }
    fn s_box(byte: u8) -> u8 {
        S_BOX[byte as usize]
    }

    fn sub_word(word: u32) -> u32 {
        let subbed_bytes_vec = word
            .to_be_bytes()
            .iter()
            .map(|x| s_box(*x))
            .collect::<Vec<u8>>();
        let mut subbed_bytes = Vec::with_capacity(4);
        for i in 0..4 {
            subbed_bytes[i] = subbed_bytes_vec[i];
        }

        u32::from_be_bytes(*arrayref::array_ref!(subbed_bytes, 0, 4))
    }

    fn add_round_key(block: Block, round_key: Block) -> Block {
        block.map(|byte| byte ^ round_key_byte)
    }

    fn sub_bytes(block: Block) -> Block {
        block.map(s_box)
    }

    fn shift_row(block: &mut Block, row_number: usize, shift_amount: usize) -> &mut Block {
        let mut row = block.row_mut(row_number);
        for i in 0..row.len() {
            row[i] = row[(i + shift_amount) % row.len()];
        }
        block
    }
    fn shift_rows(mut block: &mut Block) -> &mut Block {
        for i in 1..3 {
            block = shift_row(block, i, i);
        }
        block
    }

    fn mix_columns(mut block: &mut Block) -> &mut Block {
        block
    }
    pub fn aes_128_ecb(ciphertext: &[u8], key: &[u8; 16]) -> Vec<u8> {
        return vec![];
    }

}
