#![no_std]
#![cfg_attr(feature = "alloc", feature(alloc))]

//! A Rust library prividing pretty hex dump.
//! 
//! A `simple_*` way renders one-line hex dump, and a `pretty_*` wayrenders 
//! columned multy-line hex dump with addresing and ASCII representation.
//! 
//! ## Example of `simple_hex*`
//! ```
//! use pretty_hex::*;
//! 
//! let v = vec![222, 173, 190, 239, 202, 254, 32, 24];
//! assert_eq!(simple_hex(&v), format!("{}", v.hex_dump()));
//! 
//! println!("{}", v.hex_dump());
//! ```
//! Output: 
//! 
//! ```text
//! de ad be ef  ca fe 20 18
//! ```
//! ## Example of `pretty_hex*
//! ```
//! use pretty_hex::*;
//! 
//! let v: &[u8] = &random::<[u8;30]>();
//! assert_eq!(pretty_hex(&v), format!("{:?}", v.hex_dump()));
//! 
//! println!("{:?}", v.hex_dump());
//! ```
//! Output: 
//! 
//! ```text
//! Length: 30 (0x1e) bytes
//! 0000:   6b 4e 1a c3  af 03 d2 1e  7e 73 ba c8  bd 84 0f 83   kN......~s......
//! 0010:   89 d5 cf 90  23 67 4b 48  db b1 bc 35  bf ee         ....#gKH...5..
//! ```

extern crate alloc;

mod pretty_hex;
pub use pretty_hex::*;

#[cfg(test)]
mod test {
    use super::*;
    use alloc::{format, vec, vec::Vec};

    #[test]
    fn test_simple() {
        let bytes: Vec<u8> = (0..16).collect();
        let simple = "00 01 02 03  04 05 06 07  08 09 0a 0b  0c 0d 0e 0f";
        assert_eq!(simple, simple_hex(&bytes));
        assert_eq!(simple, format!("{}", bytes.hex_dump()));
        let s = "string";
        assert_eq!("73 74 72 69  6e 67", simple_hex(&s));
        assert!(simple_hex(&vec![]).is_empty());
    }

    #[test]
    fn test_pretty() {
        let expected_dump = r##"Length: 255 (0xff) bytes
0000:   00 01 02 03  04 05 06 07  08 09 0a 0b  0c 0d 0e 0f   ................
0010:   10 11 12 13  14 15 16 17  18 19 1a 1b  1c 1d 1e 1f   ................
0020:   20 21 22 23  24 25 26 27  28 29 2a 2b  2c 2d 2e 2f    !"#$%&'()*+,-./
0030:   30 31 32 33  34 35 36 37  38 39 3a 3b  3c 3d 3e 3f   0123456789:;<=>?
0040:   40 41 42 43  44 45 46 47  48 49 4a 4b  4c 4d 4e 4f   @ABCDEFGHIJKLMNO
0050:   50 51 52 53  54 55 56 57  58 59 5a 5b  5c 5d 5e 5f   PQRSTUVWXYZ[\]^_
0060:   60 61 62 63  64 65 66 67  68 69 6a 6b  6c 6d 6e 6f   `abcdefghijklmno
0070:   70 71 72 73  74 75 76 77  78 79 7a 7b  7c 7d 7e 7f   pqrstuvwxyz{|}~.
0080:   80 81 82 83  84 85 86 87  88 89 8a 8b  8c 8d 8e 8f   ................
0090:   90 91 92 93  94 95 96 97  98 99 9a 9b  9c 9d 9e 9f   ................
00a0:   a0 a1 a2 a3  a4 a5 a6 a7  a8 a9 aa ab  ac ad ae af   ................
00b0:   b0 b1 b2 b3  b4 b5 b6 b7  b8 b9 ba bb  bc bd be bf   ................
00c0:   c0 c1 c2 c3  c4 c5 c6 c7  c8 c9 ca cb  cc cd ce cf   ................
00d0:   d0 d1 d2 d3  d4 d5 d6 d7  d8 d9 da db  dc dd de df   ................
00e0:   e0 e1 e2 e3  e4 e5 e6 e7  e8 e9 ea eb  ec ed ee ef   ................
00f0:   f0 f1 f2 f3  f4 f5 f6 f7  f8 f9 fa fb  fc fd fe      ..............."##;

        let bytes: Vec<u8> = (0..255).collect();
        assert_eq!(expected_dump, format!("{:?}", bytes.hex_dump()));
    }
}
