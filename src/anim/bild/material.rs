use std::io::Cursor;

use anyhow::{Context as _, Result};

use crate::io::*;

/// Material data for a Klei build file
#[derive(Debug)]
pub struct Material {
  /// Texture name
  texture_name: String,
}

impl Material {
  // Creates a new material from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
    let texture_name = cursor.read_pascal_string_u32_le().context("failed to read material texture name")?;

    Ok(Self { texture_name })
  }
}

impl ToString for Material {
  fn to_string(&self) -> String {
    self.texture_name.clone()
  }
}
