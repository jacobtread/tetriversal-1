pub mod logging;

#[derive(Clone, Debug)]
pub enum Tile {
    Red,
    Green,
    Blue,
    Empty,
}

impl Tile {
    pub fn get_color(&self) -> [f32; 4] {
        match self {
            Tile::Red => {
                [1.0, 0.0, 0.0, 1.0]
            }
            Tile::Green => {
                [0.0, 1.0, 0.0, 1.0]
            }
            Tile::Blue => {
                [0.0, 0.0, 1.0, 1.0]
            }
            Tile::Empty => {
                [0.0, 0.0, 0.0, 0.0]
            }
        }
    }
}