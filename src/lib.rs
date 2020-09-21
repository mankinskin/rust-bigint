pub mod serialize;
pub mod traits;

#[cfg(feature = "rust_gmp")]
pub mod big_gmp;
#[cfg(feature = "rust_gmp")]
pub type BigInt = gmp::mpz::Mpz;
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
pub mod big_num;
#[cfg(feature = "num_bigint")]
mod big_num_gcd;

#[cfg(feature = "rug")]
pub mod big_rug;
#[cfg(feature = "rug")]
pub type BigInt = rug::Integer;
#[cfg(feature = "rug")]
pub type HexError = rug::integer::ParseIntegerError;