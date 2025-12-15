#[macro_export]
macro_rules! impl_short_msg_kat_from_file {
    // $path should be a string literal path to the KAT file (relative to the test file that invokes the macro)
    ($file:expr, $hasher:expr) => {
        use byte::hex_to_bytes;

        #[test]
        fn short_msg_kat() {
            // include the KAT text at compile time
            let txt: &str = include_str!($file);

            let mut lines = txt.lines();
            let mut entries: Vec<(String, String)> = Vec::new();

            while let Some(line) = lines.next() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if line.starts_with("Len") {
                    // advance to find Msg and MD lines for this Len block
                    let mut len: usize = if let Some(idx) = line.find('=') {
                        line[idx + 1..].trim().parse().unwrap()
                    } else {
                        panic!("Invalid Len line");
                    };
                    let mut msg_value = "";
                    let mut md_value = "";
                    loop {
                        match lines.next() {
                            Some(line) => {
                                let line = line.trim();
                                if line.is_empty() || line.starts_with('#') {
                                    continue;
                                }
                                if line.starts_with("Msg") {
                                    if let Some(idx) = line.find('=') {
                                        msg_value = line[idx + 1..].trim();
                                    }
                                    continue;
                                } else if line.starts_with("MD") {
                                    if let Some(idx) = line.find('=') {
                                        md_value = line[idx + 1..].trim();
                                    }
                                    assert_eq!(len, msg_value.len() * 4, "{msg_value}");
                                    // we got MD, end of block
                                    break;
                                } else {
                                    // ignore unexpected lines
                                    continue;
                                }
                            }
                            None => break,
                        }
                    }
                    entries.push((msg_value.to_string(), md_value.to_string()));
                }
            }

            for (message, expected) in entries.into_iter() {
                println!("Bytes: {:?}, Hex: {}", hex_to_bytes(&message), message);
                let digest = $hasher.hash_to_lowerhex(&hex_to_bytes(&message));
                assert_eq!(
                    digest, expected,
                    "mismatched digest for message hex '{}'",
                    message
                );
            }
        }
    };
}

#[macro_export]
macro_rules! impl_short_msg_kat_hmac_from_file {
    // $path should be a string literal path to the KAT file (relative to the test file that invokes the macro)
    ($file:expr, $hasher:expr) => {
        use byte::hex_to_bytes;

        #[test]
        fn short_msg_kat_hmac() {
            // include the KAT text at compile time
            let txt: &str = include_str!($file);

            let mut lines = txt.lines();
            let mut entries: Vec<(String, String, String)> = Vec::new();

            while let Some(line) = lines.next() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if line.starts_with("MsgLen") {
                    // advance to find Msg and MD lines for this Len block
                    let mut len: usize = if let Some(idx) = line.find('=') {
                        line[idx + 1..].trim().parse().unwrap()
                    } else {
                        panic!("Invalid MsgLen line");
                    };
                    let mut msg_value = "";
                    let mut key_value = "";
                    let mut md_value = "";
                    loop {
                        match lines.next() {
                            Some(line) => {
                                let line = line.trim();
                                if line.is_empty() || line.starts_with('#') {
                                    continue;
                                }
                                if line.starts_with("Msg") {
                                    if let Some(idx) = line.find('=') {
                                        msg_value = line[idx + 1..].trim();
                                    }
                                    continue;
                                } else if line.starts_with("Key") {
                                    if let Some(idx) = line.find('=') {
                                        key_value = line[idx + 1..].trim();
                                    }
                                    assert_eq!(64 * 8, key_value.len() * 4, "{key_value}");
                                    continue;
                                } else if line.starts_with("MD") {
                                    if let Some(idx) = line.find('=') {
                                        md_value = line[idx + 1..].trim();
                                    }
                                    assert_eq!(len, msg_value.len() * 4, "{msg_value}");
                                    // we got MD, end of block
                                    break;
                                } else {
                                    // ignore unexpected lines
                                    continue;
                                }
                            }
                            None => break,
                        }
                    }
                    entries.push((
                        msg_value.to_string(),
                        key_value.to_string(),
                        md_value.to_string(),
                    ));
                }
            }

            for (message, key, expected) in entries.into_iter() {
                println!("Bytes: {:?}, Hex: {}", hex_to_bytes(&message), message);
                let digest =
                    $hasher.hmac_hash_to_lower_hex(&hex_to_bytes(&message), &hex_to_bytes(&key));
                assert_eq!(
                    digest, expected,
                    "mismatched digest for message hex '{}'",
                    message
                );
            }
        }
    };
}
