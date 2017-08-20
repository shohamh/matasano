pub mod set1;
pub mod set2;
pub mod set3;
pub mod set4;
pub mod set5;
pub mod set6;
pub mod set7;
pub mod set8;


#[cfg(test)]
mod tests {
    use set1;
    #[test]
    fn set1_challenge1() {
        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
                    set1::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"))
    }
    #[test]
    fn set1_challenge2() {
        assert_eq!(
            set1::fixed_xor(set1::decode_hex("1c0111001f010100061a024b53535009181c").as_slice(), set1::decode_hex("686974207468652062756c6c277320657965").as_slice()),
            set1::decode_hex("746865206b696420646f6e277420706c6179")
        )
    }
    #[test]
    fn set1_challenge3() {
        let decoded_string = set1::decode_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let string = String::from_utf8(decoded_string.to_owned()).unwrap();
        let (mut min_chi_squared, mut min_key, mut decryption) = (set1::similarity_to_english(&string), 0, string);

        'outer: for i in 1 .. 255 {
            let attempted_decryption = String::from_utf8(set1::single_byte_xor(&decoded_string, i)).unwrap_or(String::from("00000000000000"));
            'inner: for ch in attempted_decryption.as_bytes() {
                if (*ch as char).is_control() || *ch > 127 {
                    continue 'outer;
                }
            }
            let chi_squared = set1::similarity_to_english(&attempted_decryption);
            if chi_squared < min_chi_squared {
                min_chi_squared = chi_squared;
                min_key = i;
                decryption = attempted_decryption;
            }
        }
        println!("Solution: '{}', with key: {}, Chi-squared index of similarity (lower is better): {}", decryption, min_key, min_chi_squared);
        
        assert_eq!("Cooking MC's like a pound of bacon", decryption);
    }
    #[test]
    fn set1_challenge4() {
        let mut f = ::std::fs::File::open("resources/s1c4.txt").expect("File not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Couldn't read to string");

    }

}
