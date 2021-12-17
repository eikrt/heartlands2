use serde::{Serialize, Deserialize};

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Biome {
    pub name: String,
    pub temperature: i32,
    pub tile_type: String,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tile_type: String
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk_point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
   pub points: Vec<Vec<Point>>,
}
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub name: String,
    pub chunks: Vec<Vec<Chunk>>,
    pub chunk_size: usize,
    pub sea_level: f32


}



