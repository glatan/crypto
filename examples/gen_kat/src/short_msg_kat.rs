use crypto::{cryptographic_hash_function::CryptographicHashFunction, mac::Hmac};

use crate::{AlgorithmName, bytes_to_lowerhex};

pub fn short_msg_kat<T: Clone + AlgorithmName + CryptographicHashFunction>(hasher: &mut T) {
    println!("# Short Message KAT");
    println!("# Algorithm Name: {}", T::ALGORITHM_NAME);
    println!();

    for len in 0usize..=255usize {
        let mut message: Vec<u8> = vec![0u8; len];
        getrandom::fill(&mut message).unwrap();

        let bits = message.len() * 8;
        let msg_hex = bytes_to_lowerhex(&message);
        let digest = hasher.clone().hash_to_lowerhex(&message);

        println!("Len = {}", bits);
        if message.is_empty() {
            println!("Msg =");
        } else {
            println!("Msg = {}", msg_hex);
        }
        println!("MD = {}", digest);
        if len != 255 {
            println!();
        }
    }
}

pub fn short_msg_kat_hmac<T: Clone + AlgorithmName + CryptographicHashFunction + Hmac>(
    hasher: &mut T,
) {
    println!("# Short Message KAT HMAC");
    println!("# Algorithm Name: {}", T::ALGORITHM_NAME);
    println!();

    for len in 0usize..=255usize {
        let mut key = vec![0u8; 64];
        getrandom::fill(&mut key).unwrap();
        let mut message: Vec<u8> = vec![0u8; len];
        getrandom::fill(&mut message).unwrap();

        let bits = message.len() * 8;
        let key_hex = bytes_to_lowerhex(&key);
        let msg_hex = bytes_to_lowerhex(&message);
        let digest = hasher.clone().hmac_hash_to_lower_hex(&message, &key);

        println!("MsgLen = {}", bits);
        if message.is_empty() {
            println!("Msg =");
        } else {
            println!("Msg = {}", msg_hex);
        }
        println!("Key = {}", key_hex);
        println!("MD = {}", digest);
        if len != 255 {
            println!();
        }
    }
}
