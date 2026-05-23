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
  pub bounding_box: RectangleBox,

  /// Atlas index
  pub atlas_index: usize,

  /// Atlas UV box
  pub uv_box: RectangleBox,
}

#[derive(Debug, Default)]
pub struct RectangleBox {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}
