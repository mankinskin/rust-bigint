# rust-bigint

Provides traits for common functionality across several Rust BigInt implementations

## Example interaction: 

```rust
// import BigInt from this library. use a feature flag to select the BigInt you need
// also, pull one or more traits from this library into scope
use rust_bigint::BigInt;
use rust_bigint::traits::Converter;
 
let number = BigInt::from(42);
// now use one of the methods exposed by the converter trait
let hex_str = number.to_hex();
```

See the traits for more examples.