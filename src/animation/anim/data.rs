#[derive(Debug, Default)]
pub struct Animations {
  pub anims: Vec<Animation>,
}

#[derive(Debug, Default)]
pub struct Animation {
  pub name: String,
  pub frames: Vec<AnimationFrame>,
}

#[derive(Debug, Default)]
pub struct AnimationFrame {
  pub duration: u32,
  pub image: String,
}
