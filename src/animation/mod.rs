use std::{
  collections::BTreeMap, fmt, fs::{self, File}, io::Read
};

use anyhow::{Context as _, Result, bail};
use image::{ImageBuffer, Rgba};
use quick_xml::se::{self, Serializer};
use serde::Serialize;
use zip::ZipArchive;

mod shared;

mod anim;
mod bild;

mod scml;

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
    let build_xml_path = path.as_ref().with_extension("build.xml");
    // TODO: export build to XML
    log::debug!("build.bin: {:#?}", build);

    let atlas_images: BTreeMap<usize, ImageBuffer<Rgba<u8>, Vec<u8>>> = atlases
      .iter()
      .enumerate()
      .map(|(i, atlas)| {
        let image = crate::texture::parse(atlas).context(format!("failed to parse atlas {} from anim file", i))?;
        Ok((i, image))
      })
      .collect::<Result<BTreeMap<_, _>>>()?;

    // export sprites
    for symbol in build.symbols {
      let symbol_name = symbol.name.as_str();
      for frame in symbol.frames {
        let frame_num = frame.num;
        let output_path = path.as_ref().with_extension(format!("{}.{}.png", symbol_name, frame_num));
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

        sprite.save(&output_path).context(format!(
          "failed to save sprite for symbol {} frame {}",
          symbol.name, frame.num
        ))?;
      }
    }

    if !anim_bin.is_empty() {
      let animations = anim::parse(&anim_bin).context("failed to parse anim.bin from anim file")?;
      let anim_xml_path = path.as_ref().with_extension("anim.xml");
      // TODO: export animations to XML
      log::debug!("animations: {:#?}", animations);
    }

    let scml_root = scml::SpriterData::default();
    let scml_xml_path = path.as_ref().with_extension("scml.xml");
    let mut scml_output = String::new();
    let mut serializer = Serializer::new(&mut scml_output);
    serializer.indent(' ', 4);
    scml_root.serialize(serializer).context("failed to serialize scml.xml")?;
    // let scml_file = File::create(&scml_xml_path).context("failed to create scml.xml file")?;
    // let scml_writer = std::io::BufWriter::new(scml_file);
    // let scml_writer = quick_xml::writer::Writer::new(scml_writer);
    // let scml_output = r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string()
    //   + &se::to_string(&scml_object).context("failed to serialize scml.xml")?;
    // scml_output.insert_str(0, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    // scml_output.push('\n');
    let scml_output = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}\n", scml_output);
    fs::write(&scml_xml_path, scml_output).context("failed to write scml.xml file")?;
  }

  Ok(())
}
