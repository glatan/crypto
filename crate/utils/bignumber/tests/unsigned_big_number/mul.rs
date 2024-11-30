use bignumber::UnsignedBigNumber;

#[test]
fn same_length_no_overflow() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[0]);

    let result = a * b;
    let expected = UnsignedBigNumber::new(&[0]);
    assert_eq!(result, expected);
}

#[test]
fn same_length_overflow() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[u128::MAX]);

    let result = a * b;
    let expected = UnsignedBigNumber::new(&[u128::MAX - 1, 1]);
    assert_eq!(result, expected);
}

#[test]
fn not_same_length_no_overflow() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[0, u128::MAX]);

    let result = a * b;
    let expected = UnsignedBigNumber::new(&[u128::MAX - 1, 1]);
    assert_eq!(result, expected);
}

#[test]
fn mul_ref() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[0]);

    let result = a * &b;
    let expected = UnsignedBigNumber::new(&[0]);
    assert_eq!(result, expected);
}

#[test]
fn mul_ref_and_ref() {
    let a = UnsignedBigNumber::new(&[u128::MAX]);
    let b = UnsignedBigNumber::new(&[0]);

    let result = &a * &b;
    let expected = UnsignedBigNumber::new(&[0]);
    assert_eq!(result, expected);
}
