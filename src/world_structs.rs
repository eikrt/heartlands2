use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::Rng;
const TARGET_SIZE: f32 = 8.0;
const VICINITY_SIZE: f32 = 32.0;
const INTERACTION_SIZE: f32 = 8.0;
#[derive(PartialEq)]
#[derive(Clone,Serialize, Deserialize, Debug)]
pub enum ItemType {
    NOTHING, 
    WOODEN_SPEAR,
    FRUIT,
    MEAT,
    SHOVEL,
    CROWN,
}
#[derive(PartialEq)]
#[derive(Clone,Serialize, Deserialize, Debug)]
pub enum ActionType {
    IDLE,
    FETCH_FOOD,
    STORAGE_FOOD,
    FISH,
    HUNT,
    TRADE,
    BREED,
    DEFEND,
    CONQUER,
    EXPLORE,

}
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
    QUEEN_ANT,
    DRONE_ANT,
    SOLDIER_ANT,
    MECHANT,
    SNAIL,
    FOOD_STORAGE

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
    pub target_x: f32,
    pub target_y: f32,
    pub stopped: bool,
    pub id: i32,
    pub entity_type: EntityType,
    pub faction: String, 
    pub faction_id: i32,
    pub current_action: ActionType,
    pub wielding_item: ItemType,
    pub backpack_item: ItemType,
    pub wearable_item: ItemType,
    
}
impl Entity {
    pub fn idle_mov(&mut self) {
        if !self.stopped {
            
            let mut rng = rand::thread_rng();
            self.x += self.dir.cos() * self.speed;
            self.y += self.dir.sin() * self.speed;
            self.dir = rng.gen_range(0.0..3.14*2.0);
        }
    }
    pub fn mov(&mut self) {
        if !self.stopped {
            self.dir = (self.y - self.target_y).atan2(self.x - self.target_x); 
            self.x += self.dir.cos() * self.speed;
            self.y -= self.dir.sin() * self.speed;
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
    pub fn get_entities_in_range(&self, x: f32, y: f32, width: f32, height: f32) -> Vec<Entity> {
       let mut filtered_entities = Vec::new();
       for e in self.entities.iter() {
           let rx = e.x / self.world_data.tile_size as f32;
           let ry = e.y / self.world_data.tile_size as f32;
           let l_u_corner_x = x;
           let l_u_corner_y = y;
           let r_b_corner_x = x + width;
           let r_b_corner_y = y + height;
           if rx >= l_u_corner_x && rx <= r_b_corner_x && ry >= l_u_corner_y && ry <= r_b_corner_y { 
               filtered_entities.push(e.clone());
           }
       }
       return filtered_entities; 

    }

    pub fn update_entities(&mut self) {
         
        let mut rng = rand::thread_rng();
        let entities_clone = self.entities.clone();
        for e in self.entities.iter_mut() {
            if e.entity_type == EntityType::WORKER_ANT {
                if e.current_action == ActionType::IDLE && e.backpack_item == ItemType::FRUIT {

                for v in entities_clone.iter() {
                        let dist_from_entity = ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                        if e.backpack_item == ItemType::FRUIT || e.backpack_item == ItemType::MEAT {
                            if dist_from_entity < INTERACTION_SIZE {
                                e.backpack_item = ItemType::NOTHING;
                                e.current_action = ActionType::IDLE;
                                e.target_x = 0.0;
                                e.target_y = 0.0;
                                break;
                            }
                    }
                }
            }
                else if e.current_action == ActionType::IDLE && e.target_x == 0.0 && e.target_y == 0.0 {
                    e.current_action = ActionType::EXPLORE;
                    e.target_x = e.x + rng.gen_range(-256.0..256.0);
                    e.target_y = e.y + rng.gen_range(-256.0..256.0);
                }
            if e.current_action == ActionType::EXPLORE {
                if e.x > e.target_x - TARGET_SIZE && e.y > e.target_y - TARGET_SIZE && e.x < e.target_x + TARGET_SIZE  && e.y < e.target_y + TARGET_SIZE {
                    e.current_action = ActionType::IDLE;
                    e.target_x = 0.0;
                    e.target_y = 0.0;
                }
                
                for v in entities_clone.iter() {

                    let dist_from_entity = ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                    if dist_from_entity < VICINITY_SIZE {
                        if v.entity_type == EntityType::APPLETREE {
                            e.current_action = ActionType::FETCH_FOOD;
                            e.target_x = v.x;
                            e.target_y = v.y;

                        }
                        if dist_from_entity < INTERACTION_SIZE {
                            if v.entity_type == EntityType::APPLETREE {
                                e.backpack_item = ItemType::FRUIT;
                                e.current_action = ActionType::IDLE;
                                e.target_x = 0.0;
                                e.target_y = 0.0;
                                break;
                            }
                        }
                    }
                }
            }
            if e.current_action == ActionType::IDLE {
                e.idle_mov();
            }
            else if e.current_action == ActionType::FETCH_FOOD {
                e.mov();
            }
            else if e.current_action == ActionType::EXPLORE {
                e.mov();
            }
    }
}
}
}
