use std::{collections::BTreeMap, io::Cursor};

use anyhow::{Context as _, Result, bail};

use crate::io::*;

/// The Klei build file
#[derive(Debug)]
pub struct Bild {
  /// The header of the Klei build file
  #[allow(unused)]
  header: BildHeader,

  /// Build name
  pub name: String,

  /// Materials
  pub materials: Vec<String>,

  /// Symbols
  pub symbols: Vec<super::symbol::Symbol>,

  /// Vertices (x, y, z, u, v, w)
  pub vertices: Vec<(f32, f32, f32, f32, f32, f32)>,

  /// Hashed Strings
  pub hashed_strings: BTreeMap<u32, String>,
}

impl Bild {
  /// Creates a new build file from the given bytes
  pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
    let mut cursor = Cursor::new(bytes);

    let header = BildHeader::from_cursor(&mut cursor).context("failed to read build header")?;
    let name = cursor.read_pascal_string_u32_le().context("failed to read build name")?;

    let materials_len = cursor.read_u32_le().context("failed to read materials length")?;
    let materials = (0..materials_len)
      .map(|_| cursor.read_pascal_string_u32_le().context("failed to read material name"))
      .collect::<Result<Vec<_>>>()?;

    let symbols = (0..header.num_symbols)
      .map(|_| super::symbol::Symbol::from_cursor(&mut cursor).context("failed to read symbol"))
      .collect::<Result<Vec<_>>>()?;

    let vertices_len = cursor.read_u32_le().context("failed to read vertices length")?;
    let vertices = (0..vertices_len)
      .map(|_| {
        let x = cursor.read_f32_le().context("failed to read vertex x")?;
        let y = cursor.read_f32_le().context("failed to read vertex y")?;
        let z = cursor.read_f32_le().context("failed to read vertex z")?;
        let u = cursor.read_f32_le().context("failed to read vertex u")?;
        let v = cursor.read_f32_le().context("failed to read vertex v")?;
        let w = cursor.read_f32_le().context("failed to read vertex w")?;
        Ok((x, y, z, u, v, w))
      })
      .collect::<Result<Vec<_>>>()?;

    let hashed_strings_len = cursor.read_u32_le().context("failed to read hashed strings length")?;
    let hashed_strings = (0..hashed_strings_len)
      .map(|_| {
        let hash = cursor.read_u32_le().context("failed to read hashed string hash")?;
        let string = cursor.read_pascal_string_u32_le().context("failed to read hashed string value")?;
        Ok((hash, string))
      })
      .collect::<Result<BTreeMap<_, _>>>()?;

    Ok(Self {
      header,
      name,
      materials,
      symbols,
      vertices,
      hashed_strings,
    })
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

  /// Creates a new build header from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
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
