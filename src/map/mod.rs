#[derive(Debug)]
pub struct Map {
  width: i32,
  height: i32,
  layout: Vec<Vec<Tile>>
}

impl Map {
  pub fn new() -> Self {
    let width = 40;
    let height = 40;

    let mut map = Map {
      width,
      height,
      layout: Vec::new()
    };

    map.generate_map();
    map
  }
}

impl Map {
  fn generate_map(&mut self) {
    self.layout = Vec::new();
    for y_coordinate in 0..self.height {
      let mut x_vector = Vec::new();
      for x_coordinate in 0..self.width {
        let tile: Tile = if x_coordinate == y_coordinate {
            Tile::Floor
          } else {
            Tile::Wall
          };
        x_vector.push(tile);
      }
      self.layout.push(x_vector);
    }
  }

  pub fn render(&self) {
    let mut render = String::from("");
    let mut y_iter = self.layout.iter();

    'y_loop: loop {
      let y_value = y_iter.next();
      if let None = y_value { break 'y_loop; }

      let mut x_iter = y_value.unwrap().iter();
      'x_loop: loop {
        let x_value = x_iter.next();
        if let None = x_value { break 'x_loop; }


        render.push(x_value.unwrap().render());
      }
      render.push('\n');
    }

    
    println!("{}", render);
  }
}


#[derive(Debug)]
enum Tile {
  Null, // A non-space used for void or other unknowns
  Floor, // Walkable floor
  Wall, // A solid wall
}

impl Tile {
  fn render(&self) -> char {
    match self {
      Self::Floor => ' ',
      Self::Wall => '█',
      _ => '░'
    }
  }
}
