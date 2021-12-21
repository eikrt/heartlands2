use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[serde(tag = "TileType")]
pub enum TileType {
    GRASS,
    COLD_LAND,
    WATER,
    ICE,
    PERMAFROST,
    COARSE_LAND,
    SAVANNAH_LAND,
    SAND,
    RED_SAND,
    MUD_HIVE_FLOOR,
    MUD_HIVE_WALL,
    STONE_HIVE_FLOOR,
    STONE_HIVE_WALL
}
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "EntityType")]
pub enum EntityType {
    OAK,
    APPLETREE,
    BIRCH,
    PINE,
    SPRUCE,
    CACTUS,
    WORKER_ANT,

}
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[serde(tag = "RequestType")]
pub enum RequestType {
    CHUNK,
    DATA,
}
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Biome {
    pub name: String,
    pub temperature: i32,
    pub tile_type: TileType,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tile_type: TileType
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldRequest {
    pub x: i32,
    pub y: i32,
    pub req_type: RequestType
}
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub entity_type: EntityType
}
impl Entity {
    fn mov(&mut self, dir: f32) {
        self.x += 1.0;
    }
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
    pub chunk_size: usize, 
    pub tile_size: i32 
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
           let rx = e.x / self.world_data.tile_size as f32;
           let ry = e.y / self.world_data.tile_size as f32;
           let l_u_corner_x = chunk.points[0][0].x;
           let l_u_corner_y = chunk.points[0][0].y;
           let r_b_corner_x = chunk.points[self.world_data.chunk_size  - 1 ][self.world_data.chunk_size - 1].x;
           let r_b_corner_y = chunk.points[self.world_data.chunk_size  - 1 ][self.world_data.chunk_size - 1].y;
           if rx >= l_u_corner_x && rx <= r_b_corner_x && ry >= l_u_corner_y && ry <= r_b_corner_y { 
               filtered_entities.push(e.clone());
           }
       }
       return filtered_entities; 

    }

    pub fn update_entities(&mut self, action_type: String) {
       for e in self.entities.iter_mut() {
                if e.entity_type == EntityType::BIRCH { 
                e.mov(0.0);

}
}
}}
