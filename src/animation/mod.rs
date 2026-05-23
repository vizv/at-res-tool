use std::{collections::BTreeMap, fs::File, io::Read};

use anyhow::{Context as _, Result, bail};
use image::{ImageBuffer, Rgba};
use zip::ZipArchive;

mod shared;

mod anim;
mod bild;

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

  if !build_bin.is_empty() {
    let build = bild::parse(&build_bin).context("failed to parse build.bin from anim file")?;
    // TODO: export build to XML
    // log::debug!("build.bin: {:#?}", build);

    let atlas_images: BTreeMap<usize, ImageBuffer<Rgba<u8>, Vec<u8>>> = atlases
      .iter()
      .enumerate()
      .map(|(i, atlas)| {
        let image = crate::texture::parse(atlas).context(format!("failed to parse atlas {} from anim file", i))?;
        Ok((i, image))
      })
      .collect::<Result<BTreeMap<_, _>>>()?;
    log::debug!("atlas images count: {}", atlas_images.len());

    if !anim_bin.is_empty() {
      let animations = anim::parse(&anim_bin).context("failed to parse anim.bin from anim file")?;
      // TODO: export animations to XML
      // log::debug!("animations: {:#?}", animations);
    }
  }

  Ok(())
}
