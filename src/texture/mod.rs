use anyhow::{Context as _, Result};
use image::{ImageBuffer, Rgba};

mod ktex;

pub fn dump(path: impl AsRef<std::path::Path>) -> Result<()> {
  let ktex = ktex::Ktex::from_path(&path).context("failed to load ktex file")?;
  let image = ktex.get_image().context("failed to get image from ktex file")?;
  let output_path = path.as_ref().with_extension("png");
  image.save(&output_path).context("failed to save image")?;

  Ok(())
}

pub fn parse(bytes: &[u8]) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
  let ktex = ktex::Ktex::from_bytes(bytes).context("failed to load ktex file")?;
  let image = ktex.get_image().context("failed to get image from ktex file")?;
  Ok(image)
}
