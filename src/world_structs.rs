use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::Rng;
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
#[derive(Eq)]
#[derive(Hash)]
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
    SNAIL,

}
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[serde(tag = "RequestType")]
pub enum RequestType {
    CHUNK,
    DATA,
}

pub fn get_descriptions_for_tiles() -> HashMap<TileType, String> {
    return HashMap::from([(TileType::GRASS,
                                "Grass".to_string(),
                          ),
                          (TileType::ICE,
                                "Ice".to_string() 
                           )


    ]);
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
    pub speed: f32,
    pub dir: f32,
    pub stopped: bool,
    pub id: i32,
    pub entity_type: EntityType,
    pub faction: String, 
    pub faction_id: i32, 
}
impl Entity {
    pub fn mov(&mut self) {
        if !self.stopped {
            
            let mut rng = rand::thread_rng();
            self.x += self.dir.cos() * self.speed;
            self.y += self.dir.sin() * self.speed;
            self.dir = rng.gen_range(0.0..3.14*2.0);
        }
    }
    pub fn stop(&mut self) {
        self.stopped = true;
    }
}
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldResponse {
    pub chunk: Chunk,
    pub entities: Vec<Entity>,
    pub valid: bool
}
#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
   pub points: Vec<Vec<Point>>,
   pub name: String,
   pub id: i32
}
impl Chunk {
    pub fn assign_name(&mut self, s: String) {
        self.name = s.clone();
    }
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

    pub fn update_entities(&mut self) {
       for e in self.entities.iter_mut() {
            e.mov();
        }
    }
}
