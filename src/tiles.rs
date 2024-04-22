// Describes a single renderable tile
pub struct Tile {
  character: char,
  color: String,
}

impl Tile {
  pub fn new() -> Self {
    Tile {
      character: ' ',
      color: "".to_string(),
    }
  }
}