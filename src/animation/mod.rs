use std::{collections::BTreeMap, fs, io::Read};

use anyhow::{Context as _, Result, bail};
use image::{ImageBuffer, Rgba};
use quick_xml::se::Serializer;
use serde::Serialize;
use zip::ZipArchive;

mod shared;

mod anim;
mod bild;

mod scml;

pub fn dump(path: impl AsRef<std::path::Path>) -> Result<()> {
  let file = fs::File::open(&path).context("failed to open anim file")?;
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

  let mut scml_root = scml::SpriterData::default();
  if !build_bin.is_empty() {
    let build = bild::parse(&build_bin).context("failed to parse build.bin from anim file")?;
    let build_name = build.name.as_str();
    let build_dir = path.as_ref().with_file_name(build_name);
    fs::create_dir_all(&build_dir).context("failed to create build directory")?;

    let atlas_images: BTreeMap<usize, ImageBuffer<Rgba<u8>, Vec<u8>>> = atlases
      .iter()
      .enumerate()
      .map(|(i, atlas)| {
        let image = crate::texture::parse(atlas).context(format!("failed to parse atlas {} from anim file", i))?;
        Ok((i, image))
      })
      .collect::<Result<BTreeMap<_, _>>>()?;

    // export sprites
    let mut folder_id = 0;
    for symbol in build.symbols {
      let symbol_name = symbol.name.as_str();
      let symbol_dir = build_dir.join(symbol_name);
      fs::create_dir_all(&symbol_dir).context(format!("failed to create directory for symbol {}", symbol_name))?;
      let mut folder_node = scml::Folder::default();
      folder_node.id = folder_id;
      folder_id += 1;
      folder_node.name = symbol_name.to_owned();

      for frame in symbol.frames {
        let frame_num = frame.num;
        let frame_filename = format!("{}-{}.png", symbol_name, frame_num);
        let output_path = symbol_dir.join(&frame_filename);
        let atlas_image = atlas_images.get(&frame.atlas_index).ok_or_else(|| {
          anyhow::anyhow!(
            "failed to find atlas image for frame {} (atlas index {})",
            frame.num,
            frame.atlas_index
          )
        })?;

        let sprite = image::imageops::crop_imm(
          atlas_image,
          (frame.uv_box.x * atlas_image.width() as f32).floor() as u32,
          (frame.uv_box.y * atlas_image.height() as f32).floor() as u32,
          (frame.uv_box.w * atlas_image.width() as f32).ceil() as u32,
          (frame.uv_box.h * atlas_image.height() as f32).ceil() as u32,
        )
        .to_image();

        let mut file_node = scml::File::default();
        file_node.id = frame.num;
        file_node.name = format!("{}/{}", symbol_name, frame_filename);
        file_node.width = sprite.width();
        file_node.height = sprite.height();

        // borrowed from ktools:src/krane/scml.cpp
        let bbox = &frame.bounding_box;
        file_node.pivot_x = 0.5 - bbox.x / bbox.w.round();
        file_node.pivot_y = 0.5 + bbox.y / bbox.h.round();

        folder_node.files.push(file_node);

        sprite.save(&output_path).context(format!(
          "failed to save sprite for symbol {} frame {}",
          symbol.name, frame.num
        ))?;
      }

      scml_root.folders.push(folder_node);
    }

    if !anim_bin.is_empty() {
      let scml_path = build_dir.join(build_name).with_extension("scml");
      let mut scml_output = String::new();
      let mut serializer = Serializer::new(&mut scml_output);
      serializer.indent(' ', 4);
      scml_root.serialize(serializer).context("failed to serialize scml.xml")?;
      let scml_output = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}\n", scml_output);
      fs::write(&scml_path, scml_output).context("failed to write scml.xml file")?;
    }
  }

  Ok(())
}
