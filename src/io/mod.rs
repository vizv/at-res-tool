use std::io;

/// Helper methods for validating file magic.
pub trait MagicReadExt: io::Read {
  fn read_magic(&mut self, expected: &[u8]) -> io::Result<()> {
    let mut magic = vec![0u8; expected.len()];
    self.read_exact(&mut magic)?;
    if magic != expected {
      return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("Invalid magic: expected {:?}, got {:?}", expected, magic),
      ));
    }
    Ok(())
  }
}

impl<R: io::Read> MagicReadExt for R {}

/// Helper methods for reading unsigned byte.
pub trait ByteReadExt: io::Read {
  fn read_u8(&mut self) -> io::Result<u8> {
    let mut out = [0u8; 1];
    self.read_exact(&mut out)?;
    Ok(out[0])
  }
}

impl<R: io::Read> ByteReadExt for R {}

/// Helper methods for reading unsigned integers in little-endian form.
pub trait LittleEndianReadExt: io::Read {
  fn read_u16_le(&mut self) -> io::Result<u16> {
    let mut out = [0u8; 2];
    self.read_exact(&mut out)?;
    Ok(u16::from_le_bytes(out))
  }

  fn read_u32_le(&mut self) -> io::Result<u32> {
    let mut out = [0u8; 4];
    self.read_exact(&mut out)?;
    Ok(u32::from_le_bytes(out))
  }
}

impl<R: io::Read> LittleEndianReadExt for R {}

/// Helper methods for reading pascal strings (length-prefixed UTF-8 strings).
pub trait PascalStringReadExt: io::Read {
  fn read_pascal_string_u32_le(&mut self) -> io::Result<String> {
    let mut out = [0u8; 4];
    self.read_exact(&mut out)?;
    let len = u32::from_le_bytes(out) as usize;

    let mut out = vec![0u8; len];
    self.read_exact(&mut out)?;
    String::from_utf8(out).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
  }
}

impl<R: io::Read> PascalStringReadExt for R {}
