use std::io::Cursor;

use anyhow::{Context as _, Result, bail};

use crate::io::*;

mod material;

/// The Klei build file
#[derive(Debug)]
pub struct Bild {
  /// The header of the Klei build file
  #[allow(unused)]
  header: BildHeader,

  /// Build name
  name: String,
}

impl Bild {
  /// Creates a new build file from the given bytes
  pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
    let mut cursor = Cursor::new(bytes);

    let header = BildHeader::from_bytes(&mut cursor).context("failed to read build header")?;
    let name = cursor.read_pascal_string_u32_le().context("failed to read build name")?;

    Ok(Self { header, name })
  }
}

// The header of a Klei build file
#[derive(Debug, Default)]
struct BildHeader {
  version: u32,
  num_symbols: u32,
  num_frames: u32,
}

impl BildHeader {
  const MAGIC: [u8; 4] = *b"BILD";
  const SUPPORTED_VERSION: u32 = 6;

  /// Creates a new build header from the given bytes
  pub fn from_bytes(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
    cursor.read_magic(&Self::MAGIC).context("Failed to read magic")?;

    let mut header = Self::default();

    header.version = cursor.read_u32_le().context("Failed to read version")?;
    header.num_symbols = cursor.read_u32_le().context("Failed to read num_symbols")?;
    header.num_frames = cursor.read_u32_le().context("Failed to read num_frames")?;

    header.validate().context("Invalid build header")?;

    Ok(header)
  }

  /// Validates the build header
  fn validate(&self) -> Result<()> {
    if self.version != Self::SUPPORTED_VERSION {
      bail!(
        "Unsupported build version: expected {}, got {}",
        Self::SUPPORTED_VERSION,
        self.version
      );
    }

    Ok(())
  }
}
