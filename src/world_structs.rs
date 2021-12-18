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
pub struct WorldRequest {
    pub x: i32,
    pub y: i32,
    pub req_type: String
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
   pub points: Vec<Vec<Point>>,
}
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldData {

    pub name: String,
    pub sea_level: f32,
    pub width: usize,
    pub height: usize,
    pub chunk_size: usize 
}
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub chunks: Vec<Vec<Chunk>>,
    pub world_data: WorldData,
}


