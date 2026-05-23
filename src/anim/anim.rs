use anyhow::{Context, Result, bail};

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
    let header = AnimHeader::from_bytes(bytes).context("failed to read anim header")?;
    Ok(Self { header })
  }
}

// The header of a Klei animation file
#[derive(Debug, Default)]
struct AnimHeader {
  magic: [u8; 4],
  version: u32,
  num_elements: u32,
  num_frames: u32,
  num_events: u32,
  num_anims: u32,
}

impl AnimHeader {
  const MAGIC: [u8; 4] = *b"ANIM";
  const SUPPORTED_VERSIONS: &[u32] = &[5, 6];

  /// Creates a new anim header from the given bytes
  pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
    if bytes.len() < std::mem::size_of::<Self>() {
      bail!("Not enough bytes for anim header ({} bytes)", bytes.len());
    }

    let mut header = Self::default();

    let magic = &bytes[0..4];
    if magic != Self::MAGIC {
      bail!(
        "Invalid anim file: expected file magic {:?}, got {:?}",
        Self::MAGIC,
        magic
      );
    }
    header.magic.copy_from_slice(magic);

    let version = u32::from_le_bytes(bytes[4..8].try_into().context("Failed to read version")?);
    if !Self::SUPPORTED_VERSIONS.contains(&version) {
      bail!(
        "Unsupported anim version: expected {:?}, got {}",
        Self::SUPPORTED_VERSIONS,
        version
      );
    }
    header.version = version;

    header.num_elements = u32::from_le_bytes(bytes[8..12].try_into().context("Failed to read num_elements")?);
    header.num_frames = u32::from_le_bytes(bytes[12..16].try_into().context("Failed to read num_frames")?);
    header.num_events = u32::from_le_bytes(bytes[16..20].try_into().context("Failed to read num_events")?);
    header.num_anims = u32::from_le_bytes(bytes[20..24].try_into().context("Failed to read num_anims")?);

    if header.num_events != 0 {
      bail!("Unsupported anim file: events are not supported");
    }

    Ok(header)
  }
}
