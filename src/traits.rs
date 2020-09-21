//! Traits to define common functionality across BigInt implementations. This is based on MIT/Apache-licensed
/// work by [KZen](https://github.com/KZen-networks/curv/blob/master/src/arithmetic/traits.rs)

use super::BigInt;
use super::HexError;
use std::marker::Sized;

pub trait ZeroizeBN {
    fn zeroize_bn(&mut self);
}

/// Convert BigInt to and from various formats
pub trait Converter {
    /// Convert BigInt to `Vec<u8>`
    fn to_vec(n: &Self) -> Vec<u8>;
    /// Convert BigInt to a hex string
    fn to_hex(&self) -> String;
    /// Create a BigInt from a hex String
    fn from_hex(n: &str) -> Result<BigInt, HexError>;
    /// Create a BigInt from bytes
    fn from_bytes(bytes: &[u8]) -> Self;
    /// Convert a BigInt to bytes
    fn to_bytes(&self) -> Vec<u8>;
}

/// Mod operations for BigInt
pub trait Modulo {
    fn mod_pow(base: &Self, exponent: &Self, modulus: &Self) -> Self;
    fn mod_mul(a: &Self, b: &Self, modulus: &Self) -> Self;
    fn mod_sub(a: &Self, b: &Self, modulus: &Self) -> Self;
    fn mod_add(a: &Self, b: &Self, modulus: &Self) -> Self;
    fn mod_inv(a: &Self, modulus: &Self) -> Self;
}

/// Sampling trait (e.g., for rejection sampling)
pub trait Samplable {
    fn sample_below(upper: &Self) -> Self;
    fn sample_range(lower: &Self, upper: &Self) -> Self;
    fn strict_sample_range(lower: &Self, upper: &Self) -> Self;
    fn sample(bitsize: usize) -> Self;
    fn strict_sample(bit_size: usize) -> Self;
}

/// Numerical tests for a BigInt
pub trait NumberTests {
    /// Is a BigInt equal to zero
    fn is_zero(_: &Self) -> bool;
    /// Is a BigInt even
    fn is_even(_: &Self) -> bool;
    /// Is a BitInt negative
    fn is_negative(_: &Self) -> bool;
    /// How many bits in a BigInt
    fn bits(_: &Self) -> usize;
}

/// Trait for the extended eucledian algorithm
pub trait EGCD
where
    Self: Sized,
{
    /// The extended eucledian algorithm
    fn egcd(a: &Self, b: &Self) -> (Self, Self, Self);
}

/// Allow bit manipulation across BigInts
pub trait BitManipulation {
    /// Set a given bit
    fn set_bit(self: &mut Self, bit: usize, bit_val: bool);
    /// Test value of given bit
    fn test_bit(self: &Self, bit: usize) -> bool;
}

/// Trait to allow other BigInt conversion (e.g., from u64)
pub trait ConvertFrom<T> {
    fn _from(_: &T) -> Self;
}
