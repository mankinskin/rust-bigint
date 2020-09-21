//! Serialize any bigint representation. These functions can be used by serde

use crate::traits::Converter;
use crate::BigInt;
use serde::{de, ser};
use std::fmt;

pub fn serialize<S: ser::Serializer>(x: &BigInt, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&x.to_str_radix(16))
}

pub fn deserialize<'de, D: de::Deserializer<'de>>(deserializer: D) -> Result<BigInt, D::Error> {
    struct BigIntVisitor;

    impl<'de> de::Visitor<'de> for BigIntVisitor {
        type Value = BigInt;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("bigint")
        }

        fn visit_str<E: de::Error>(self, s: &str) -> Result<BigInt, E> {
            let v: String = str::parse(s).map_err(de::Error::custom)?;
            let i = BigInt::from_hex(&v).map_err(de::Error::custom)?;
            Ok(i)
        }
    }

    deserializer.deserialize_str(BigIntVisitor)
}