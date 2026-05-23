#[derive(Debug, Default)]
pub struct Animations {
  /// Animations
  pub anims: Vec<Animation>,
}

#[derive(Debug, Default)]
pub struct Animation {
  /// Name
  pub name: String,

  /// Valid facing
  pub valid_facing: u8,

  /// Root symbol
  pub root_symbol: String,

  /// Frame rate
  pub frame_rate: f32,

  /// Frames
  pub frames: Vec<AnimationFrame>,
}

#[derive(Debug, Default)]
pub struct AnimationFrame {
  /// Bounding box
  pub bounding_box: super::super::shared::RectangleBox,

  /// Elements
  pub elements: Vec<AnimationElement>,
}

#[derive(Debug, Default)]
pub struct AnimationElement {
  /// Symbol
  pub symbol: String,

  /// Symbol frame
  pub symbol_frame: u32,

  /// Folder
  pub folder: String,

  /// Affine transform matrix
  pub affine_transform: super::super::shared::AffineTransform,

  /// Depth
  pub depth: f32,
}
