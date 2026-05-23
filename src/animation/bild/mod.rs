use anyhow::{Result, bail};

mod bild_file;
mod frame;
mod symbol;

mod data;

pub fn parse(bytes: &[u8]) -> Result<data::Build> {
  let bild_file = bild_file::BildFile::from_bytes(bytes)?;
  let mut build = data::Build::default();

  build.name = bild_file.name;
  for symbol in bild_file.symbols {
    let mut build_symbol = data::BuildSymbol::default();
    let name = bild_file
      .hashed_strings
      .get(&symbol.hash)
      .ok_or_else(|| anyhow::anyhow!("failed to find symbol name"))?
      .to_owned();
    build_symbol.name = name;
    for frame in symbol.frames {
      let mut build_frame = data::BuildFrame::default();
      build_frame.num = frame.num;
      build_frame.duration = frame.duration;
      build_frame.bounding_box = data::RectangleBox {
        x: frame.bbox.0,
        y: frame.bbox.1,
        w: frame.bbox.2,
        h: frame.bbox.3,
      };

      let mut u1 = f32::MAX;
      let mut v1 = f32::MAX;
      let mut u2 = f32::MIN;
      let mut v2 = f32::MIN;
      for vertex_index in frame.vb_start_index..(frame.vb_start_index + frame.num_verts) {
        let &(_x, _y, z, u, v, w) = bild_file
          .vertices
          .get(vertex_index as usize)
          .ok_or_else(|| anyhow::anyhow!("failed to find vertex for frame {} (index {})", frame.num, vertex_index))?;
        u1 = u1.min(u);
        v1 = v1.min(v);
        u2 = u2.max(u);
        v2 = v2.max(v);

        if z != 0.0 {
          bail!("unexpected non-zero vertex z value in frame {}: {}", frame.num, z);
        }

        if w % 1.0 != 0.0 {
          bail!("unexpected non-integer vertex w value in frame {}: {}", frame.num, w);
        }

        build_frame.atlas_index = w as usize;
        if build_frame.atlas_index >= bild_file.materials.len() {
          bail!(
            "unexpected vertex material index in frame {}: {} ({} materials)",
            frame.num,
            build_frame.atlas_index,
            bild_file.materials.len()
          );
        }
      }
      build_frame.uv_box = data::RectangleBox {
        x: u1,
        y: v1,
        w: u2 - u1,
        h: v2 - v1,
      };
      build_symbol.frames.push(build_frame);
    }
    build.symbols.push(build_symbol);
  }

  Ok(build)
}
