use std::{fs, io::Cursor, path::Path};

use anyhow::{Context as _, Result, bail};
use image::ImageBuffer;
use image_dds::ddsfile::Dds;

use crate::io::*;

/// The Klei texture file
#[derive(Debug)]
pub struct Ktex {
  /// The header of the Klei texture file
  #[allow(unused)]
  header: KtexHeader,
  /// The Klei texture DDS data
  dds: Dds,
}

impl Ktex {
  /// Creates a new ktex file from the given file path
  pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
    let data = fs::read(path)?;
    Self::from_bytes(&data)
  }

  pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
    let mut cursor = Cursor::new(bytes);

    let header = KtexHeader::from_bytes(&mut cursor).context("failed to read ktex header")?;
    let dds = Dds::read(&mut cursor).context("failed to read DDS data")?;

    Ok(Self { header, dds })
  }

  /// Gets the image as an ImageBuffer
  pub fn get_image(&self) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    image_dds::image_from_dds(&self.dds, 0).context("failed to convert DDS to image")
  }
}

/// The header of a Klei texture file
#[derive(Debug, Default)]
struct KtexHeader {
  version: u8,
  width: u16,
  height: u16,
}

impl KtexHeader {
  const MAGIC: [u8; 4] = *b"KTEX";
  const SUPPORTED_VERSION: u8 = 2;

  /// Creates a new ktex header from the given bytes
  pub fn from_bytes(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
    cursor.read_magic(&Self::MAGIC).context("Failed to read magic")?;

    let mut header = Self::default();

    header.version = cursor.read_u8().context("Failed to read version")?;
    header.width = cursor.read_u16_le().context("Failed to read width")?;
    header.height = cursor.read_u16_le().context("Failed to read height")?;

    header.validate().context("Invalid ktex header")?;

    Ok(header)
  }

  fn validate(&self) -> Result<()> {
    if self.version != Self::SUPPORTED_VERSION {
      bail!(
        "Unsupported ktex file: expected version {}, got {}",
        Self::SUPPORTED_VERSION,
        self.version
      );
    }

    if self.width % 2 != 0 || self.height % 2 != 0 {
      bail!(
        "Invalid ktex file: width and height must be even, got {}x{}",
        self.width,
        self.height
      );
    }

    Ok(())
  }
}
