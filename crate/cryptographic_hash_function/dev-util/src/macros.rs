#[macro_export]
macro_rules! impl_test {
    ($self:ident, $test_name:ident, $test_cases:ident, $hasher:expr) => {
        mod $test_name {
            use super::$self;
            use super::$test_cases;
            use cryptographic_hash_function::CryptographicHashFunction;
            #[test]
            fn lower_hex() {
                for (m, e) in $test_cases.iter() {
                    assert_eq!($hasher.hash_to_lowerhex(m), *e);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_hmac_test {
    ($self:ident, $test_cases:ident, $hasher:expr) => {
        #[test]
        fn lower_hex() {
            for (key, message, expected_digest) in $test_cases.iter() {
                let digest = bytes_to_lowerhex($hasher.hmac_hash_to_bytes(message, key).as_ref());
                assert_eq!(&digest, *expected_digest);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_benchmark {
    ($module:ident, $T:ident) => {
        #[allow(non_snake_case)]
        mod $T {
            extern crate test;
            use cryptographic_hash_function::CryptographicHashFunction;
            use test::Bencher;
            use $module::$T;
            #[bench]
            #[allow(non_snake_case)]
            fn B064(b: &mut Bencher) {
                b.iter(|| $T::default().hash_to_bytes(&[0; 64]));
                b.bytes = 64;
            }
            #[bench]
            #[allow(non_snake_case)]
            fn KB256(b: &mut Bencher) {
                b.iter(|| $T::default().hash_to_bytes(&[0; 1024 * 256]));
                b.bytes = 1024 * 256;
            }
            #[bench]
            #[allow(non_snake_case)]
            fn MB004(b: &mut Bencher) {
                b.iter(|| $T::default().hash_to_bytes(&[0; 1024 * 1024 * 4]));
                b.bytes = 1024 * 1024 * 4;
            }
        }
    };
}
