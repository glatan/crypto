#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::fmt::Write;

pub fn hex_to_bytes(s: &str) -> Vec<u8> {
    // 上位4ビット
    let s1: Vec<u8> = s
        .chars()
        .by_ref()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, c)| (c.to_digit(16).unwrap() as u8) << 4)
        .collect();
    // 下位4ビット
    let s2: Vec<u8> = s
        .chars()
        .by_ref()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, c)| c.to_digit(16).unwrap() as u8)
        .collect();
    if s1.len() != s2.len() {
        unreachable!();
    }

    {
        let mut bytes: Vec<u8> = Vec::new();
        for i in 0..s1.len() {
            bytes.push((s1[i] & 0b1111_0000) | (s2[i] & 0b0000_1111));
        }
        bytes
    }
}

pub fn bytes_to_lowerhex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut hex, byte| {
        let _ = write!(hex, "{byte:02x}");
        hex
    })
}

pub fn bytes_to_upperhex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut hex, byte| {
        let _ = write!(hex, "{byte:02X}");
        hex
    })
}
