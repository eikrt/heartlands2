
pub struct Biome {
    pub name: String,
    pub temperature: i32,
    pub tile_type: String,
}



pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Chunk {
   pub points: Vec<Vec<Point>>,
}

pub struct World {
    pub name: String,
    pub chunks: Vec<Vec<Chunk>>,
    pub chunk_size: usize,
    pub sea_level: f32


}



