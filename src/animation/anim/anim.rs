use std::io::Cursor;

use anyhow::{Context as _, Result};

use crate::io::*;

/// Animation data for a Klei animation file
#[derive(Debug, Default)]
pub struct Anim {
  /// Animation name
  pub name: String,

  /// Valid facing directions
  pub valid_facing: u8,

  /// Root symbol hash
  pub root_symbol_hash: u32,

  /// Frame rate of the animation
  pub frame_rate: f32,

  /// Animation frames
  pub frames: Vec<super::frame::Frame>,
}

impl Anim {
  /// Creates a new animation from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>, version: u32) -> Result<Self> {
    let mut anim = Self::default();

    anim.name = cursor.read_pascal_string_u32_le().context("failed to read animation name")?;
    anim.valid_facing = cursor.read_u8().context("failed to read animation valid facing")?;
    anim.root_symbol_hash = cursor.read_u32_le().context("failed to read animation root symbol hash")?;
    anim.frame_rate = cursor.read_f32_le().context("failed to read animation frame rate")?;

    let num_frames = cursor.read_u32_le().context("failed to read animation number of frames")?;
    for _ in 0..num_frames {
      let frame = super::frame::Frame::from_cursor(cursor, version).context("failed to read animation frame")?;
      anim.frames.push(frame);
    }

    Ok(anim)
  }
} 
