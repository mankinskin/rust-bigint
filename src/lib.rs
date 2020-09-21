//! Provides traits for common functionality across several Rust BigInt implementations
//!
//! ## Example interaction:
//! ```
//! // import BigInt from this library. use a feature flag to select the BigInt you need
//! // also, pull one or more traits from this library into scope
//! use rust_bigint::BigInt;
//! use rust_bigint::traits::Converter;
//! 
//! let number = BigInt::from(42);
//! // now use one of the methods exposed by the converter trait
//! let hex_str = number.to_hex();
//! ```
//!
//! See the traits for more examples.

pub mod serialize;
pub mod traits;

#[cfg(feature = "rust_gmp")]
mod big_gmp;
/// Expose selected BigInt type (defaults to GMP)
#[cfg(feature = "rust_gmp")]
pub type BigInt = gmp::mpz::Mpz;
/// Expose error type associated with selected BigInt type (defaults to GMP)
#[cfg(feature = "rust_gmp")]
pub type HexError = gmp::mpz::ParseMpzError;

#[cfg(feature = "num_bigint")]
extern crate num_bigint;
#[cfg(feature = "num_bigint")]
extern crate num_integer;
#[cfg(feature = "num_bigint")]
extern crate num_traits;
#[cfg(feature = "num_bigint")]
pub type BigInt = num_bigint::BigInt;
#[cfg(feature = "num_bigint")]
pub type HexError = num_bigint::ParseBigIntError;
#[cfg(feature = "num_bigint")]
mod big_num;
#[cfg(feature = "num_bigint")]
mod big_num_gcd;
