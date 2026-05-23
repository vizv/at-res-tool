use std::io::Cursor;

use anyhow::{Context as _, Result};

use crate::io::*;

/// Element data for a Klei animation file
#[derive(Debug, Default)]
pub struct Element {
  /// Symbol hash
  pub symbol_hash: u32,

  /// Symbol frame
  pub symbol_frame: u32,

  /// Folder hash
  pub folder_hash: u32,

  /// Transform matrix
  pub mat: super::matrix::Matrix
}

impl Element {
  /// Creates a new element from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>, version: u32) -> Result<Self> {
    let mut elem = Self::default();

    elem.symbol_hash = cursor.read_u32_le().context("failed to read element symbol hash")?;
    elem.symbol_frame = cursor.read_u32_le().context("failed to read element symbol frame")?;
    elem.folder_hash = cursor.read_u32_le().context("failed to read element folder hash")?;
    if version == 5 {
      // Skip unknown value for animation file version 5
      cursor.read_f32_le().context("failed to read element unknown value for animation file version 5")?;
    }
    if version == 6 {
      // Skip unknown hash for animation file version 6
      cursor.read_u32_le().context("failed to read element unknown hash for animation file version 6")?;
    }
    elem.mat = super::matrix::Matrix::from_cursor(cursor, version).context("failed to read element transform matrix")?;

    Ok(elem)
  }
}
