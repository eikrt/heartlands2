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
pub struct Entity {
    pub x: f32,
    pub y: f32
}
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldResponse {
    pub chunk: Chunk,
    pub entities: Vec<Entity>
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
    pub entities: Vec<Entity>,
    pub world_data: WorldData,
}

impl World {
    pub fn get_entities_for_chunk(&self, chunk: Chunk) -> Vec<Entity> {
       let mut filtered_entities = Vec::new();

       for e in self.entities.iter() {
           if e.x > chunk.points[0][0].x && e.x < chunk.points[self.world_data.chunk_size][self.world_data.chunk_size].x && e.y > chunk.points[0][0].x && e.y < chunk.points[self.world_data.chunk_size][self.world_data.chunk_size].y {
               filtered_entities.push(e.clone());
           }
       }

       return filtered_entities; 

    }
}
