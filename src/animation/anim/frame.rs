use std::io::Cursor;

use anyhow::{Context as _, Result, bail};

use crate::io::*;

/// Frame data for a Klei animation file
#[derive(Debug, Default)]
pub struct Frame {
  /// Bounding box (x, y, w, h)
  pub bbox: (f32, f32, f32, f32),

  /// Frame elements
  pub elements: Vec<super::element::Element>,
}

impl Frame {
  /// Creates a new frame from the given cursor
  pub fn from_cursor(cursor: &mut Cursor<&[u8]>, version: u32) -> Result<Self> {
    let mut frame = Self::default();

    let x = cursor.read_f32_le().context("failed to read frame bbox.x")?;
    let y = cursor.read_f32_le().context("failed to read frame bbox.y")?;
    let w = cursor.read_f32_le().context("failed to read frame bbox.w")?;
    let h = cursor.read_f32_le().context("failed to read frame bbox.h")?;
    frame.bbox = (x, y, w, h);

    let num_events = cursor.read_u32_le().context("failed to read frame number of events")?;
    if num_events > 0 {
      bail!("frame events are not supported");
    }

    let num_elements = cursor.read_u32_le().context("failed to read frame number of elements")?;
    for _ in 0..num_elements {
      let element = super::element::Element::from_cursor(cursor, version).context("failed to read frame element")?;
      frame.elements.push(element);
    }

    Ok(frame)
  }
}
