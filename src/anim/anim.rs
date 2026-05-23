use std::io::Cursor;

use anyhow::{Context as _, Result, bail};

use crate::io::*;

/// The Klei animation file
#[derive(Debug)]
pub struct Anim {
  /// The header of the Klei animation file
  #[allow(unused)]
  header: AnimHeader,
}

impl Anim {
  /// Creates a new anim file from the given bytes
  pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
    let mut cursor = Cursor::new(bytes);

    let header = AnimHeader::from_cursor(&mut cursor).context("failed to read anim header")?;

    Ok(Self { header })
  }
}

// The header of a Klei animation file
#[derive(Debug, Default)]
struct AnimHeader {
  version: u32,
  num_elements: u32,
  num_frames: u32,
  num_events: u32,
  num_anims: u32,
}

impl AnimHeader {
  const MAGIC: [u8; 4] = *b"ANIM";
  const SUPPORTED_VERSIONS: &[u32] = &[5, 6];

  /// Creates a new anim header from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
    cursor.read_magic(&Self::MAGIC).context("Failed to read magic")?;

    let mut header = Self::default();

    header.version = cursor.read_u32_le().context("Failed to read version")?;
    header.num_elements = cursor.read_u32_le().context("Failed to read num_elements")?;
    header.num_frames = cursor.read_u32_le().context("Failed to read num_frames")?;
    header.num_events = cursor.read_u32_le().context("Failed to read num_events")?;
    header.num_anims = cursor.read_u32_le().context("Failed to read num_anims")?;

    header.validate().context("Invalid anim header")?;

    Ok(header)
  }

  fn validate(&self) -> Result<()> {
    if !Self::SUPPORTED_VERSIONS.contains(&self.version) {
      bail!(
        "Unsupported anim version: expected {:?}, got {}",
        Self::SUPPORTED_VERSIONS,
        self.version
      );
    }

    if self.num_events != 0 {
      bail!("Unsupported anim file: events are not supported");
    }

    Ok(())
  }
}
