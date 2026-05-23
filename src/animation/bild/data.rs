#[derive(Debug, Default)]
pub struct Build {
  /// Name
  pub name: String,

  /// Symbols
  pub symbols: Vec<BuildSymbol>,
}

#[derive(Debug, Default)]
pub struct BuildSymbol {
  /// Name
  pub name: String,

  /// Frames
  pub frames: Vec<BuildFrame>,
}

#[derive(Debug, Default)]
pub struct BuildFrame {
  /// Number
  pub num: u32,

  /// Duration
  pub duration: u32,

  /// Bounding box
  pub bounding_box: super::super::shared::RectangleBox,

  /// Atlas index
  pub atlas_index: usize,

  /// Atlas UV box
  pub uv_box: super::super::shared::RectangleBox,
}
