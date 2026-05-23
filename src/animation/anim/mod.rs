use anyhow::{Context as _, Result};

mod anim_file;

mod anim;
mod element;
mod frame;
mod matrix;

mod data;

pub fn parse(bytes: &[u8]) -> Result<data::Animations> {
  let anim_file = AnimFile::from_bytes(bytes).context("failed to parse anim file")?;
  let mut animations = data::Animations::default();

  for anim in anim_file.anims {
    let mut animation = data::Animation::default();
    animation.name = anim.name.to_owned();
    for frame in anim.frames {
      let mut animation_frame = data::AnimationFrame::default();
      animation_frame.bounding_box = super::shared::RectangleBox {
        x: frame.bbox.0,
        y: frame.bbox.1,
        w: frame.bbox.2,
        h: frame.bbox.3,
      };
      for element in frame.elements {
        let mut animation_element = data::AnimationElement::default();
        let symbol = anim_file
          .hashed_strings
          .get(&element.symbol_hash)
          .ok_or_else(|| {
            anyhow::anyhow!(
              "failed to find symbol name for element with symbol hash {:08x}",
              element.symbol_hash
            )
          })?
          .to_owned();
        animation_element.symbol = symbol;
        animation_element.symbol_frame = element.symbol_frame;
        let folder = anim_file
          .hashed_strings
          .get(&element.folder_hash)
          .ok_or_else(|| {
            anyhow::anyhow!(
              "failed to find folder name for element with folder hash {:08x}",
              element.folder_hash
            )
          })?
          .to_owned();
        animation_element.folder = folder;
        let affine_transform = super::shared::AffineTransform {
          a: element.mat.affine.0,
          b: element.mat.affine.1,
          c: element.mat.affine.2,
          d: element.mat.affine.3,
          tx: element.mat.affine.4,
          ty: element.mat.affine.5,
        };
        animation_element.affine_transform = affine_transform;
        animation_element.depth = element.mat.depth;
        animation_frame.elements.push(animation_element);
      }
      animation.frames.push(animation_frame);
    }
    animations.anims.push(animation);
  }

  Ok(animations)
}

pub use anim_file::AnimFile; // XXX
