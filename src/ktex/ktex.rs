use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use image::ImageBuffer;
use image_dds::ddsfile::Dds;

/// The Klei texture file
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
    let header = KtexHeader::from_bytes(&data).context("failed to read ktex header")?;
    let data = &data[std::mem::size_of::<KtexHeader>()..];
    let dds = Dds::read(data).context("failed to read DDS data")?;
    Ok(Self { header, dds })
  }

  /// Gets the image as an ImageBuffer
  pub fn get_image(&self) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
    image_dds::image_from_dds(&self.dds, 0).context("failed to convert DDS to image")
  }
}

/// The header of a Klei texture file
#[derive(Debug)]
#[repr(packed)]
struct KtexHeader {
  magic: [u8; 4],
  version: u8,
  width: u16,
  height: u16,
}

impl KtexHeader {
  const MAGIC: [u8; 4] = *b"KTEX";
  const SUPPORTED_VERSION: u8 = 2;

  pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
    if bytes.len() < std::mem::size_of::<Self>() {
      bail!("Not enough bytes for ktex header ({} bytes)", bytes.len());
    }

    let mut header = Self {
      magic: [0; 4],
      version: 0,
      width: 0,
      height: 0,
    };

    let magic = &bytes[0..4];
    if magic != Self::MAGIC {
      bail!("Invalid ktex file: expected file magic {:?}, got {:?}", Self::MAGIC, magic);
    }
    header.magic.copy_from_slice(magic);

    let version = bytes[4];
    if version != Self::SUPPORTED_VERSION {
      bail!("Unsupported ktex file: expected version {}, got {}", Self::SUPPORTED_VERSION, version);
    }
    header.version = version;

    let width = u16::from_le_bytes(bytes[5..7].try_into().context("Failed to read width")?);
    let height = u16::from_le_bytes(bytes[7..9].try_into().context("Failed to read height")?);
    if width % 2 != 0 || height % 2 != 0 {
      bail!("Invalid ktex file: width and height must be even, got {}x{}", width, height);
    }
    header.width = u16::from_le_bytes(bytes[5..7].try_into().unwrap());
    header.height = u16::from_le_bytes(bytes[7..9].try_into().unwrap());

    Ok(header)
  }
}
