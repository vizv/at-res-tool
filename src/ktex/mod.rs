use anyhow::Result;

mod ktex;

pub fn dump(path: impl AsRef<std::path::Path>) -> Result<()> {
  let ktex = ktex::Ktex::new(path)?;

  Ok(())
}
