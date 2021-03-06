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
    use crate::set1;
    use arrayref;
    use std::fs::read_to_string;

    #[test]
    fn byte_hamming_distance() {
        assert_eq!(set1::byte_hamming_distance(0x0, 0x0), 0);
        assert_eq!(set1::byte_hamming_distance(0x1, 0x0), 1);
        assert_eq!(set1::byte_hamming_distance(0x2, 0x0), 1);
        assert_eq!(set1::byte_hamming_distance(0x3, 0x0), 2);
        assert_eq!(set1::byte_hamming_distance(0x3, 0x1), 1);
        assert_eq!(set1::byte_hamming_distance(0xff, 0xfe), 1);
        assert_eq!(set1::byte_hamming_distance(0xff, 0x0), 8);
    }

    #[test]
    fn hamming_distance() {
        assert_eq!(
            set1::hamming_distance(b"this is a tes!", b"wokka wokka!!!"),
            33
        );
        assert_eq!(
            set1::hamming_distance(b"this is b test", b"wokka wokka!!!"),
            37
        );
        assert_eq!(
            set1::hamming_distance(b"this is a test", b"wokka wokka!!!"),
            37
        );
    }

    #[test]
    fn base64_encode() {
        assert_eq!(
            String::from("YW55IGNhcm5hbCBwbGVhcw=="),
            set1::base64_encode("any carnal pleas".as_bytes())
        );
        assert_eq!(
            String::from("YW55IGNhcm5hbCBwbGVhc3U="),
            set1::base64_encode("any carnal pleasu".as_bytes())
        );
        assert_eq!(
            String::from("YW55IGNhcm5hbCBwbGVhc3Vy"),
            set1::base64_encode("any carnal pleasur".as_bytes())
        );
    }
    #[test]
    fn base64_decode() {
        assert_eq!(
            "any carnal pleas".as_bytes().to_vec(),
            set1::base64_decode("YW55IGNhcm5hbCBwbGVhcw==")
        );
        assert_eq!(
            "any carnal pleasu".as_bytes().to_vec(),
            set1::base64_decode("YW55IGNhcm5hbCBwbGVhc3U=")
        );
        assert_eq!(
            "any carnal pleasur".as_bytes().to_vec(),
            set1::base64_decode("YW55IGNhcm5hbCBwbGVhc3Vy")
        );

        let contents_s1c6 = read_to_string("resources/s1c6_no_newlines.txt").unwrap();
        // decoded externally:
        let contents_s1c6_de = read_to_string("resources/s1c6_decoded_externally.bin").unwrap();

        assert_eq!(
            contents_s1c6_de.as_bytes(),
            set1::base64_decode(&contents_s1c6).as_slice()
        )
    }
    #[test]
    fn set1_challenge1() {
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            set1::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
        )
    }
    #[test]
    fn set1_challenge2() {
        assert_eq!(
            set1::fixed_xor(
                &set1::decode_hex("1c0111001f010100061a024b53535009181c"),
                &set1::decode_hex("686974207468652062756c6c277320657965")
            ),
            set1::decode_hex("746865206b696420646f6e277420706c6179")
        )
    }
    #[test]
    fn set1_challenge3() {
        let decoded_string = set1::decode_hex(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        );
        let (plaintext, min_key, min_chi_squared) =
            set1::decrypt_single_byte_xor_english(&decoded_string);
        println!(
            "Solution: '{:?}', with key: {}, Chi-squared index of similarity (lower is better): {}",
            plaintext, min_key, min_chi_squared
        );

        assert_eq!("Cooking MC's like a pound of bacon", plaintext.unwrap());
    }

    #[test]
    fn set1_challenge4() {
        let contents = read_to_string("resources/s1c4.txt").unwrap();
        let mut lines = contents.lines().map(set1::decode_hex);

        let (mut plaintext, mut min_key, mut min_chi_squared) =
            set1::decrypt_single_byte_xor_english(&lines.next().unwrap());
        for line in lines {
            let (attempted_decryption, temp_key, temp_chi_squared) =
                set1::decrypt_single_byte_xor_english(&line);
            if temp_chi_squared < min_chi_squared {
                plaintext = attempted_decryption;
                min_key = temp_key;
                min_chi_squared = temp_chi_squared;
            }
        }

        println!(
            "Solution: {:?}, with key: {}, Chi-squared index of similarity (lower is better): {}",
            plaintext, min_key, min_chi_squared
        );

        assert_eq!(
            Some(String::from("Now that the party is jumping\n")),
            plaintext
        )
    }
    #[test]
    fn set1_challenge5() {
        let plaintext = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let key = "ICE";
        let ciphertext = set1::repeating_key_xor(plaintext, key);

        assert_eq!(
            set1::encode_hex(ciphertext.as_bytes()),
            "b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    }
    #[test]
    fn set1_challenge6() {
        let contents_s1c6 = read_to_string("resources/s1c6_no_newlines.txt").unwrap();
        let ciphertext = set1::base64_decode(&contents_s1c6);
        let solution = set1::break_vigenere(&ciphertext);
        let solution_from_file = read_to_string("resources/s1c6_solution.txt").unwrap();

        assert_eq!(solution, solution_from_file);
    }
    #[test]
    fn transpose_matrix() {
        let matrix = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
        ];

        assert_eq!(
            set1::transpose_matrix(matrix.clone(), 4),
            vec![
                vec![0, 4, 8, 12],
                vec![1, 5, 9, 13],
                vec![2, 6, 10, 14],
                vec![3, 7, 11, 15]
            ]
        );

        assert_eq!(
            matrix.clone(),
            set1::transpose_matrix(
                set1::transpose_matrix(matrix.clone(), 4),
                4
            )
        );
    }

    #[test]
    fn set1_challenge7() {
        let contents_s1c7 = read_to_string("resources/s1c7_no_newlines.txt").unwrap();
        let key : &[u8; 16] = arrayref::array_ref!("YELLOW SUBMARINE".as_bytes(), 0, 16);
        let ciphertext = set1::base64_decode(&contents_s1c7);
        let plaintext = set1::aes::aes_128_ecb(&ciphertext, key);
        println!("ciphertext: {:?}", ciphertext);
        println!("plaintext: {:?}", plaintext);
    }

    #[test]
    fn aes_round_constants() {
        assert_eq!(set1::aes::round_constants(10), [0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000, 0x80000000, 0x1B000000, 0x36000000]);
    }

}
