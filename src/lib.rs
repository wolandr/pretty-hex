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

mod pretty_hex;
pub use pretty_hex::*;

#[cfg(test)]
mod test {
    use super::*;

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
        let bytes: Vec<u8> = (0..255).collect();
        println!("{:?}", bytes.hex_dump());
    }
}
