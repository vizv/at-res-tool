use std::io::Cursor;

use anyhow::{Context as _, Result};

use crate::io::*;

/// Symbol data for a Klei build file
#[derive(Debug)]
pub struct Symbol {
  /// Symbol hash
  hash: u32,

  /// Frames in the symbol
  frames: Vec<super::frame::Frame>,
}

impl Symbol {
  /// Creates a new symbol from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
    let hash = cursor.read_u32_le().context("failed to read symbol hash")?;

    let frames_len = cursor.read_u32_le().context("failed to read symbol frames length")?;

    let frames = (0..frames_len)
      .map(|_| super::frame::Frame::from_cursor(cursor).context("failed to read symbol frame"))
      .collect::<Result<Vec<_>>>()?;

    Ok(Self { hash, frames })
  }
}
