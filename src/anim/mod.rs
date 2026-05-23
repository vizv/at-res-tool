use std::{fs::File, io::Read};

use anyhow::{Context, Result, bail};
use zip::ZipArchive;

mod anim;

pub fn dump(path: impl AsRef<std::path::Path>) -> Result<()> {
  let file = File::open(&path).context("failed to open anim file")?;
  let mut archive = ZipArchive::new(file).context("failed to read anim zip archive")?;

  let mut anim_bin: Vec<u8> = Vec::new();
  let mut build_bin: Vec<u8> = Vec::new();
  let mut atlases: Vec<Vec<u8>> = Vec::new();

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).context("failed to read file from anim zip archive")?;
    let name = file.name();
    match name {
      "anim.bin" => {
        file.read_to_end(&mut anim_bin).context("failed to read anim.bin from anim file")?;
      }
      "build.bin" => {
        file.read_to_end(&mut build_bin).context("failed to read build.bin from anim file")?;
      }
      _ => {
        if name.starts_with("atlas-") && name.ends_with(".tex") {
          let id: usize = name["atlas-".len()..name.len() - ".tex".len()]
            .parse()
            .context("failed to parse atlas id from anim file name")?;
          if id >= atlases.len() {
            atlases.resize(id + 1, Vec::new());
          }
          file.read_to_end(&mut atlases[id]).context(format!("failed to read atlas {} from anim file", id))?;

          continue;
        }

        bail!("unexpected file in anim zip archive: {}", name);
      }
    }
  }

  log::debug!("anim.bin size: {} bytes", anim_bin.len());
  log::debug!("build.bin size: {} bytes", build_bin.len());
  for (i, atlas) in atlases.iter().enumerate() {
    log::debug!("atlas-{}.tex size: {} bytes", i, atlas.len());
  }

  Ok(())
}
