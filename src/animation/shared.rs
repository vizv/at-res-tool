#[derive(Debug, Default)]
pub struct RectangleBox {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
}

#[derive(Debug, Default)]
pub struct AffineTransform {
  pub a: f32,
  pub b: f32,
  pub c: f32,
  pub d: f32,
  pub tx: f32,
  pub ty: f32,
}
