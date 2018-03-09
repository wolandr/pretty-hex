# pretty-hex
[![crates.io](https://img.shields.io/crates/v/pretty-hex.svg)](https://crates.io/crates/pretty-hex) [![docs.rs](https://docs.rs/pretty-hex/badge.svg)](https://docs.rs/pretty-hex)

A Rust library prividing pretty hex dump.

A `simple_*` way renders one-line hex dump, and a `pretty_*` wayrenders 
columned multy-line hex dump with addresing and ASCII representation.

[View `pretty-hex` documentation on docs.rs.](https://docs.rs/pretty-hex)

## Example of `simple_hex`

```rust
use pretty_hex::*;

let v = vec![222, 173, 190, 239, 202, 254, 32, 24];
assert_eq!(simple_hex(&v), format!("{}", v.hex_dump()));

println!("{}", v.hex_dump());
```
Output: 

```text
de ad be ef  ca fe 20 18
```

## Example of `pretty_hex`

```rust
use pretty_hex::*;

let v: &[u8] = &random::<[u8;30]>();
assert_eq!(pretty_hex(&v), format!("{:?}", v.hex_dump()));

println!("{:?}", v.hex_dump());
```

Output: 

```text
Length: 30 (0x1e) bytes
0000:   6b 4e 1a c3  af 03 d2 1e  7e 73 ba c8  bd 84 0f 83   kN......~s......
0010:   89 d5 cf 90  23 67 4b 48  db b1 bc 35  bf ee         ....#gKH...5..
```

---

Inspired by [haskell's pretty-hex](https://hackage.haskell.org/package/pretty-hex-1.0).
