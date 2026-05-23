use std::io::Cursor;

use anyhow::{Context as _, Result};

use crate::io::*;

/// Frame data for a Klei build file
#[derive(Debug, Default)]
pub struct Frame {
  /// Frame number
  num: u32,

  /// Frame duration
  duration: u32,

  /// Bounding box (x, y, w, h)
  bbox: (f32, f32, f32, f32),

  /// Vertex buffer start index
  vb_start_index: u32,

  /// Number of vertices in the vertex buffer
  num_verts: u32,
}

impl Frame {
  /// Creates a new frame from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
    let mut frame = Self::default();

    frame.num = cursor.read_u32_le().context("failed to read frame number")?;
    frame.duration = cursor.read_u32_le().context("failed to read frame duration")?;
    let x = cursor.read_f32_le().context("failed to read frame bbox.x")?;
    let y = cursor.read_f32_le().context("failed to read frame bbox.y")?;
    let w = cursor.read_f32_le().context("failed to read frame bbox.w")?;
    let h = cursor.read_f32_le().context("failed to read frame bbox.h")?;
    frame.bbox = (x, y, w, h);
    frame.vb_start_index = cursor.read_u32_le().context("failed to read frame vertex buffer start index")?;
    frame.num_verts = cursor.read_u32_le().context("failed to read frame number of vertices")?;

    Ok(frame)
  }
}
