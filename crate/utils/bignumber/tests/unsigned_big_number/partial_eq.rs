use bignumber::UnsignedBigNumber;

#[test]
fn compare_same_length_same_value() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[u128::MAX]);
    assert_eq!(a, b);
}

#[test]
fn compare_same_length_not_same_value() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[0]);
    assert_ne!(a, b);
}

#[test]
fn compare_not_same_length() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[u128::MAX, u128::MAX]);
    assert_ne!(a, b);
}
