use bignumber::UnsignedBigNumber;

#[test]
fn same_length_same_value() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[u128::MAX]);

    let result = a ^ b;
    let expected = UnsignedBigNumber::new(&[0]);
    assert_eq!(result, expected);
}

#[test]
fn same_length_not_same_value() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[0]);

    let result = a ^ b;
    let expected = UnsignedBigNumber::new(&[u128::MAX]);
    assert_eq!(result, expected);
}

#[test]
fn not_same_length() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[u128::MAX, u128::MAX]);

    let result = a ^ b;
    let expected = UnsignedBigNumber::new(&[u128::MAX, 0]);
    assert_eq!(result, expected);
}

#[test]
fn bitxor_ref() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[u128::MAX]);

    let result = a ^ &b;
    let expected = UnsignedBigNumber::new(&[0]);
    assert_eq!(result, expected);
}

#[test]
fn bitxor_ref_and_ref() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[u128::MAX]);

    let result = &a ^ &b;
    let expected = UnsignedBigNumber::new(&[0]);
    assert_eq!(result, expected);
}
