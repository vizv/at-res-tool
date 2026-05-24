use serde::Serialize;

// #[derive(Debug, Default, Serialize)]
// pub struct ScmlObject {
//   #[serde(rename = "spriter_data")]
//   root: SpriterData,
// }

#[derive(Debug, Serialize)]
#[serde(rename = "spriter_data")]
pub struct SpriterData {
  #[serde(rename = "@scml_version")]
  scml_version: String,
  #[serde(rename = "@generator")]
  generator: String,
  #[serde(rename = "@generator_version")]
  generator_version: String,
  #[serde(rename = "folder")]
  folders: Vec<Folder>,
  // #[serde(rename = "entity")]
  // entities: Vec<Entity>,
}

impl Default for SpriterData {
  fn default() -> Self {
    Self {
      scml_version: "1.0".to_string(),
      generator: "BrashMonkey Spriter".to_string(),
      generator_version: "b5".to_string(),
      folders: Vec::new(),
    }
  }
}

#[derive(Debug, Default, Serialize)]
pub struct Folder {
  #[serde(rename = "@id")]
  id: u32,
  #[serde(rename = "@name")]
  name: String,
  #[serde(rename = "file")]
  files: Vec<File>,
}

#[derive(Debug, Default, Serialize)]
pub struct File {
  #[serde(rename = "@id")]
  id: u32,
  #[serde(rename = "@name")]
  name: String,
  #[serde(rename = "@width")]
  width: u32,
  #[serde(rename = "@height")]
  height: u32,
  #[serde(rename = "@pivot_x")]
  pivot_x: f32,
  #[serde(rename = "@pivot_y")]
  pivot_y: f32,
}

// #[derive(Debug, Default, Serialize)]
// pub struct Entity {
//   #[serde(rename = "animation")]
//   animations: Vec<Animation>,
// }

// #[derive(Debug, Default, Serialize)]
// pub struct Animation {
//   #[serde(rename = "id")]
//   id: u32,
//   #[serde(rename = "name")]
//   name: String,
// }
