#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::{
    fmt::{Debug, Display},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Sub},
};

#[derive(PartialOrd, Clone)]
pub struct UnsignedBigNumber {
    digits: Vec<u128>,
}

impl UnsignedBigNumber {
    pub fn new(digits: &[u128]) -> Self {
        let skip_len = digits.iter().take_while(|digit| **digit == 0).count();
        let mut inner = digits[skip_len..].to_vec();
        inner.reverse();
        if inner.is_empty() {
            inner.push(0);
        }
        Self { digits: inner }
    }
    pub fn push(&mut self, digit: u128) {
        let len = self.digits.len();
        let mut needs_to_allocate = true;
        for i in 0..self.digits.len() {
            if self.digits[len - i] == 0 {
                self.digits[len - i] = digit;
                needs_to_allocate = false;
            }
        }
        if needs_to_allocate {
            self.digits.push(digit);
        }
    }
    fn upper_digit_is_zero(&self, index: usize) -> bool {
        if index > self.digits.len() {
            panic!("digit length is {} but {} given", self.digits.len(), index);
        }
        self.digits[index..].iter().all(|digit| digit == &0)
    }
    fn get_or_zero(&self, index: usize) -> u128 {
        *self.digits.get(index).unwrap_or(&0)
    }
}

impl Display for UnsignedBigNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "digits: 0x")?;
        for i in (0..self.digits.len()).rev() {
            write!(f, "{:032X}", self.digits[i])?;
            if i != 0 {
                write!(f, "_")?;
            }
        }
        Ok(())
    }
}

impl Debug for UnsignedBigNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl PartialEq for UnsignedBigNumber {
    fn eq(&self, other: &Self) -> bool {
        let different_len = self.digits.len() != other.digits.len();
        if different_len {
            if self.digits.len() > other.digits.len() {
                let len = other.digits.len();
                let upper_digit_is_not_zero = !self.upper_digit_is_zero(len);
                if upper_digit_is_not_zero {
                    return false;
                }
            } else {
                let len = self.digits.len();
                let upper_digit_is_not_zero = !other.upper_digit_is_zero(len);
                if upper_digit_is_not_zero {
                    return false;
                }
            };
        }
        let mut same = true;
        let len = if self.digits.len() > other.digits.len() {
            other.digits.len()
        } else {
            self.digits.len()
        };
        for i in 0..len {
            same = self.digits[i] == other.digits[i];
        }
        same
    }
}

impl BitAnd for UnsignedBigNumber {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let len = if self.digits.len() > rhs.digits.len() {
            self.digits.len()
        } else {
            rhs.digits.len()
        };
        let mut result = Self {
            digits: vec![0; len],
        };

        for i in 0..len {
            if i >= self.digits.len() || i >= rhs.digits.len() {
                result.digits[i] = 0;
            } else {
                result.digits[i] = self.digits[i].bitand(rhs.digits[i]);
            }
        }

        result
    }
}

impl BitOr for UnsignedBigNumber {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let len = if self.digits.len() > rhs.digits.len() {
            self.digits.len()
        } else {
            rhs.digits.len()
        };
        let mut result = Self {
            digits: vec![0; len],
        };

        for i in 0..len {
            if i >= self.digits.len() {
                result.digits[i] = rhs.digits[i];
            } else if i >= rhs.digits.len() {
                result.digits[i] = self.digits[i];
            } else {
                result.digits[i] = self.digits[i].bitor(rhs.digits[i]);
            }
        }

        result
    }
}

impl BitXor for UnsignedBigNumber {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let len = if self.digits.len() > rhs.digits.len() {
            self.digits.len()
        } else {
            rhs.digits.len()
        };
        let mut result = Self {
            digits: vec![0; len],
        };

        for i in 0..len {
            result.digits[i] = self.get_or_zero(i).bitxor(rhs.get_or_zero(i));
        }

        result
    }
}

impl Add<Self> for UnsignedBigNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let len = if self.digits.len() > rhs.digits.len() {
            self.digits.len()
        } else {
            rhs.digits.len()
        };
        let mut result = Self {
            digits: vec![0; len],
        };

        let mut carry = false;
        for i in 0..len {
            let (mut digit, mut overflow) = self.get_or_zero(i).overflowing_add(rhs.get_or_zero(i));
            if carry {
                if overflow {
                    (digit, _) = digit.overflowing_add(1);
                } else {
                    (digit, overflow) = digit.overflowing_add(1);
                }
            }
            carry = overflow;

            result.digits[i] = digit;
        }
        if carry {
            result.digits.push(1);
        }

        result
    }
}

impl Sub<Self> for UnsignedBigNumber {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        unimplemented!("");

        if rhs > self {
            panic!("{} is greeter than {}", rhs, self);
        }

        let len = if self.digits.len() > rhs.digits.len() {
            self.digits.len()
        } else {
            rhs.digits.len()
        };
        let mut result = Self {
            digits: vec![0; len],
        };
        result
    }
}

impl Mul<Self> for UnsignedBigNumber {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let len = self.digits.len() + rhs.digits.len();
        let mut result = Self {
            digits: vec![0; len],
        };

        let mut carry = 0;
        for i in 0..len {
            if i >= self.digits.len() || i >= rhs.digits.len() {
                result.digits[i] = carry;
                break;
            }

            let a1 = (self.digits[i] >> u64::BITS) & u64::MAX as u128;
            let a0 = self.digits[i] & u64::MAX as u128;
            let b1 = (rhs.digits[i] >> u64::BITS) & u64::MAX as u128;
            let b0 = rhs.digits[i] & u64::MAX as u128;

            let z2 = a1 * b1;
            let (z1, overflow_z1) = (a1 * b0).overflowing_add(b1 * a0);
            let z0 = a0 * b0;

            /*
             *   z2 | z0
             * +   z1
             */
            let mut current_digit;
            let overflow_z0_plus_z1_half;
            (current_digit, overflow_z0_plus_z1_half) =
                z0.overflowing_add((z1 << u64::BITS) & (u128::MAX - u64::MAX as u128));
            let overflow_on_add_carry;
            (current_digit, overflow_on_add_carry) = current_digit.overflowing_add(carry);

            carry = z2;
            if overflow_z1 {
                carry += 1 << u64::BITS;
            }
            if overflow_z0_plus_z1_half {
                carry += 1;
            }
            if overflow_on_add_carry {
                carry += 1;
            }
            carry += (z1 >> u64::BITS) & u64::MAX as u128;

            result.digits[i] = current_digit;
        }

        result
    }
}

impl Div<Self> for UnsignedBigNumber {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        unimplemented!("");

        let len = if self.digits.len() > rhs.digits.len() {
            self.digits.len()
        } else {
            rhs.digits.len()
        };
        let mut result = Self {
            digits: vec![0; len],
        };
        result
    }
}

impl Rem<Self> for UnsignedBigNumber {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        unimplemented!("");

        let len = if self.digits.len() > rhs.digits.len() {
            self.digits.len()
        } else {
            rhs.digits.len()
        };
        let mut result = Self {
            digits: vec![0; len],
        };
        result
    }
}

macro_rules! impl_ops_by_ref {
    ($ops_trait: ident, $ops_fn: ident) => {
        // T ops &T
        impl $ops_trait<&Self> for UnsignedBigNumber {
            type Output = <UnsignedBigNumber as $ops_trait>::Output;
            fn $ops_fn(self, rhs: &Self) -> Self::Output {
                $ops_trait::$ops_fn(self, rhs.clone())
            }
        }

        // &T ops &T
        impl $ops_trait<Self> for &UnsignedBigNumber {
            type Output = <UnsignedBigNumber as $ops_trait>::Output;
            fn $ops_fn(self, rhs: Self) -> Self::Output {
                $ops_trait::$ops_fn(self.clone(), rhs.clone())
            }
        }
    };
}

impl_ops_by_ref!(Add, add);
impl_ops_by_ref!(Sub, sub);
impl_ops_by_ref!(Mul, mul);
impl_ops_by_ref!(Div, div);
impl_ops_by_ref!(Rem, rem);

impl_ops_by_ref!(BitAnd, bitand);
impl_ops_by_ref!(BitOr, bitor);
impl_ops_by_ref!(BitXor, bitxor);

#[cfg(test)]
mod tests {
    use crate::UnsignedBigNumber;

    #[test]
    fn new_with_empty_digits() {
        let a = UnsignedBigNumber::new(&[]);
        assert_eq!(a.digits.len(), 1);
        assert_eq!(a.digits, [0].to_vec());
    }

    #[test]
    fn new_with_start_by_non_zero_digits() {
        let a = UnsignedBigNumber::new(&[u128::MAX, 0, 0]);
        assert_eq!(a.digits.len(), 3);
        assert_eq!(a.digits, [0, 0, u128::MAX].to_vec());
    }

    #[test]
    fn new_with_start_by_zero_digits() {
        let a = UnsignedBigNumber::new(&[0, 0, u128::MAX]);
        assert_eq!(a.digits.len(), 1);
        assert_eq!(a.digits, [u128::MAX].to_vec());
    }
}
