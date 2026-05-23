use std::{fs, path::Path};

use anyhow::{Context, Result, bail};

/// The Klei animation file
pub struct Anim {
  /// The header of the Klei animation file
  #[allow(unused)]
  header: AnimHeader,
}

// The header of a Klei animation file
#[derive(Debug)]
#[repr(packed)]
struct AnimHeader {
  magic: [u8; 4],
  version: u8,
}
