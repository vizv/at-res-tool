use std::io::Cursor;

use anyhow::{Context as _, Result};

use crate::io::*;

/// Matrix data for a Klei animation file
#[derive(Debug, Default)]
pub struct Matrix {
  /// Affine transform matrix (a, b, c, d, tx, ty)
  pub affine: (f32, f32, f32, f32, f32, f32),

  /// Depth value for rendering order
  pub depth: f32,
}

impl Matrix {
  /// Creates a new matrix from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>, version: u32) -> Result<Self> {
    let mut mat = Self::default();

    // Skip the first affine transform matrix and depth value
    cursor.read_f32_le().context("failed to read matrix unused_affine.a")?;
    cursor.read_f32_le().context("failed to read matrix unused_affine.b")?;
    cursor.read_f32_le().context("failed to read matrix unused_affine.c")?;
    cursor.read_f32_le().context("failed to read matrix unused_affine.d")?;
    cursor.read_f32_le().context("failed to read matrix unused_affine.tx")?;
    cursor.read_f32_le().context("failed to read matrix unused_affine.ty")?;
    cursor.read_f32_le().context("failed to read matrix unused_affine.z")?;
    if version == 6 {
      // Skip unknown value for animation file version 6
      cursor.read_f32_le().context("failed to read matrix unknown value for animation file version 6")?;
    }

    // Read the actual affine transform matrix and depth value
    let a = cursor.read_f32_le().context("failed to read matrix affine.a")?;
    let b = cursor.read_f32_le().context("failed to read matrix affine.b")?;
    let c = cursor.read_f32_le().context("failed to read matrix affine.c")?;
    let d = cursor.read_f32_le().context("failed to read matrix affine.d")?;
    let tx = cursor.read_f32_le().context("failed to read matrix affine.tx")?;
    let ty = cursor.read_f32_le().context("failed to read matrix affine.ty")?;
    mat.affine = (a, b, c, d, tx, ty);
    mat.depth = cursor.read_f32_le().context("failed to read matrix depth")?;

    Ok(mat)
  }
}
