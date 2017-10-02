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
    use std::io::Read;
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
        let (plaintext, min_key, min_chi_squared) = set1::decrypt_single_byte_xor_english(&string);
        println!("Solution: '{}', with key: {}, Chi-squared index of similarity (lower is better): {}", plaintext, min_key, min_chi_squared);
        
        assert_eq!("Cooking MC's like a pound of bacon", plaintext);
    }

    #[test]
    fn set1_challenge4() {
        let mut f = ::std::fs::File::open("resources/s1c4.txt").expect("File not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Couldn't read to string");
        let lines : Vec<&str> = contents.lines().collect();
        let (mut plaintext, mut min_key, mut min_chi_squared) = set1::decrypt_single_byte_xor_english(&lines[0]);
        
        //while i < lines.len() {
        for i in 1..lines.len() {    
            let (attempted_decryption, temp_key, temp_chi_squared) = set1::decrypt_single_byte_xor_english(&lines[i]);
            println!("Solution: '{}', with key: {}, Chi-squared index of similarity (lower is better): {}", attempted_decryption, temp_key, temp_chi_squared);
            if temp_chi_squared < min_chi_squared {
                plaintext = attempted_decryption;
                min_key = temp_key;
                min_chi_squared = temp_chi_squared;
            }
        }

        println!("Solution: '{}', with key: {}, Chi-squared index of similarity (lower is better): {}", plaintext, min_key, min_chi_squared);

    }

}
