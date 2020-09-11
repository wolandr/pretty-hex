#[cfg(feature = "alloc")]
use alloc::string::String;
use core::{default::Default, fmt};

/// Returns a one-line hexdump of `source` grouped in default format without header
/// and ASCII column.
#[cfg(feature = "alloc")]
pub fn simple_hex<T: AsRef<[u8]>>(source: &T) -> String {
    let mut writer = String::new();
    hex_write(&mut writer, source, HexConfig::simple()).unwrap_or(());
    writer
}

/// Dump `source` as hex octets in default format without header and ASCII column to the `writer`.
pub fn simple_hex_write<T, W>(writer: &mut W, source: &T) -> fmt::Result
where
    T: AsRef<[u8]>,
    W: fmt::Write,
{
    hex_write(writer, source, HexConfig::simple())
}

/// Return a multi-line hexdump in default format complete with addressing, hex digits,
/// and ASCII representation.
#[cfg(feature = "alloc")]
pub fn pretty_hex<T: AsRef<[u8]>>(source: &T) -> String {
    let mut writer = String::new();
    hex_write(&mut writer, source, HexConfig::default()).unwrap_or(());
    writer
}

/// Write multi-line hexdump in default format complete with addressing, hex digits,
/// and ASCII representation to the writer.
pub fn pretty_hex_write<T, W>(writer: &mut W, source: &T) -> fmt::Result
where
    T: AsRef<[u8]>,
    W: fmt::Write,
{
    hex_write(writer, source, HexConfig::default())
}

/// Return a hexdump of `source` in specified format.
#[cfg(feature = "alloc")]
pub fn config_hex<T: AsRef<[u8]>>(source: &T, cfg: HexConfig) -> String {
    let mut writer = String::new();
    hex_write(&mut writer, source, cfg).unwrap_or(());
    writer
}

/// Configuration parameters for hexdump.
#[derive(Clone, Copy, Debug)]
pub struct HexConfig {
    /// Write first line header with data length.
    pub title: bool,
    /// Append ASCII representation column.
    pub ascii: bool,
    /// Source bytes per row. 0 for single row without address prefix.
    pub width: usize,
    /// Chunks count per group. 0 for single group (column).
    pub group: usize,
    /// Source bytes per chunk (word). 0 for single word.
    pub chunk: usize,
}

/// Default configuration with `title`, `ascii`, 16 source bytes `width` grouped to 4 separate
/// hex bytes. Using in `pretty_hex`, `pretty_hex_write` and `fmt::Debug` implementation.
impl Default for HexConfig {
    fn default() -> HexConfig {
        HexConfig {
            title: true,
            ascii: true,
            width: 16,
            group: 4,
            chunk: 1,
        }
    }
}

impl HexConfig {
    /// Returns configuration for `simple_hex`, `simple_hex_write` and `fmt::Display` implementation.
    pub fn simple() -> Self {
        HexConfig::default().to_simple()
    }

    fn delimiter(&self, i: usize) -> &'static str {
        if i > 0 && self.chunk > 0 && i % self.chunk == 0 {
            if self.group > 0 && i % (self.group * self.chunk) == 0 {
                "  "
            } else {
                " "
            }
        } else {
            ""
        }
    }

    fn to_simple(self) -> Self {
        HexConfig {
            title: false,
            ascii: false,
            width: 0,
            ..self
        }
    }
}

const NON_ASCII: char = '.';

/// Write hex dump in specified format.
pub fn hex_write<T, W>(writer: &mut W, source: &T, cfg: HexConfig) -> fmt::Result
where
    T: AsRef<[u8]>,
    W: fmt::Write,
{
    if cfg.title {
        writeln!(writer, "Length: {0} (0x{0:x}) bytes", source.as_ref().len())?;
    }

    if source.as_ref().is_empty() {
        return Ok(());
    }

    let lines = source.as_ref().chunks(if cfg.width > 0 {
        cfg.width
    } else {
        source.as_ref().len()
    });
    let lines_len = lines.len();
    for (i, row) in lines.enumerate() {
        if cfg.width > 0 {
            write!(writer, "{:04x}:   ", i * cfg.width)?;
        }
        for (i, x) in row.as_ref().iter().enumerate() {
            write!(writer, "{}{:02x}", cfg.delimiter(i), x)?;
        }
        if cfg.ascii {
            for j in row.len()..cfg.width {
                write!(writer, "{}  ", cfg.delimiter(j))?;
            }
            write!(writer, "   ")?;
            for x in row {
                if x.is_ascii() && !x.is_ascii_control() {
                    writer.write_char((*x).into())?;
                } else {
                    writer.write_char(NON_ASCII)?;
                }
            }
        }
        if i + 1 < lines_len {
            writeln!(writer)?;
        }
    }
    Ok(())
}

/// Reference wrapper for use in arguments formatting.
pub struct Hex<'a, T: 'a>(&'a T, HexConfig);

impl<'a, T: 'a + AsRef<[u8]>> fmt::Display for Hex<'a, T> {
    /// Formats the value by `simple_hex_write` using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hex_write(f, self.0, self.1.to_simple())
    }
}

impl<'a, T: 'a + AsRef<[u8]>> fmt::Debug for Hex<'a, T> {
    /// Formats the value by `pretty_hex_write` using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hex_write(f, self.0, self.1)
    }
}

/// Allows generates hex dumps to a formatter.
pub trait PrettyHex: Sized {
    /// Wrap self reference for use in `std::fmt::Display` and `std::fmt::Debug`
    /// formatting as hex dumps.
    fn hex_dump(&self) -> Hex<Self>;

    /// Wrap self reference for use in `std::fmt::Display` and `std::fmt::Debug`
    /// formatting as hex dumps in specified format.
    fn hex_conf(&self, cfg: HexConfig) -> Hex<Self>;
}

impl<T> PrettyHex for T
where
    T: AsRef<[u8]>,
{
    fn hex_dump(&self) -> Hex<Self> {
        Hex(self, HexConfig::default())
    }
    fn hex_conf(&self, cfg: HexConfig) -> Hex<Self> {
        Hex(self, cfg)
    }
}
