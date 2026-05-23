use anyhow::{Context, Result, bail};

/// The Klei build file
#[derive(Debug)]
pub struct Bild {
  /// The header of the Klei build file
  #[allow(unused)]
  header: BildHeader,
}

impl Bild {
  /// Creates a new build file from the given bytes
  pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
    let header = BildHeader::from_bytes(bytes).context("failed to read build header")?;
    Ok(Self { header })
  }
}

// The header of a Klei build file
#[derive(Debug)]
struct BildHeader {
  magic: [u8; 4],
  version: u32,
  num_symbols: u32,
  num_frames: u32,
}

impl BildHeader {
  const MAGIC: [u8; 4] = *b"BILD";
  const SUPPORTED_VERSION: u32 = 6;

  /// Creates a new build header from the given bytes
  pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
    if bytes.len() < std::mem::size_of::<Self>() {
      bail!("Not enough bytes for build header ({} bytes)", bytes.len());
    }

    let mut header = Self {
      magic: [0; 4],
      version: 0,
      num_symbols: 0,
      num_frames: 0,
    };

    let magic = &bytes[0..4];
    if magic != Self::MAGIC {
      bail!(
        "Invalid build file: expected file magic {:?}, got {:?}",
        Self::MAGIC,
        magic
      );
    }
    header.magic.copy_from_slice(magic);

    let version = u32::from_le_bytes(bytes[4..8].try_into().context("Failed to read version")?);
    if version != Self::SUPPORTED_VERSION {
      bail!(
        "Unsupported build version: expected {}, got {}",
        Self::SUPPORTED_VERSION,
        version
      );
    }
    header.version = version;

    header.num_symbols = u32::from_le_bytes(bytes[8..12].try_into().context("Failed to read num_symbols")?);
    header.num_frames = u32::from_le_bytes(bytes[12..16].try_into().context("Failed to read num_frames")?);

    Ok(header)
  }
}
