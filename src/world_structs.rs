use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
const TARGET_SIZE: f32 = 8.0;
const VICINITY_SIZE: f32 = 96.0;
const INTERACTION_SIZE: f32 = 8.0;
const CHUNKRANGE: usize = 2;
const BACKPACKSIZE: u8 = 64;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "CategoryType")]
pub enum CategoryType {
    Ant,
    Tree,
    Vegetation,
    Animal,
    Furniture,
}
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum ItemType {
    Nothing,
    WoodenSpear,
    WoodenShovel,
    Fruit,
    Meat,
    Shovel,
    Crown,
}
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum ActionType {
    Idle,
    FetchFood,
    ReturnFood,
    StorageFood,
    Fish,
    Hunt,
    Trade,
    Breed,
    Defend,
    Conquer,
    Explore,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "TileType")]
pub enum TileType {
    Grass,
    ColdLand,
    Water,
    Ice,
    PermaFrost,
    CoarseLand,
    SavannahLand,
    Sand,
    RedSand,
    MudHiveFloor,
    MudHiveWall,
    StoneHiveFloor,
    StoneHiveWall,
}
#[derive(PartialEq, Clone, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(tag = "EntityType")]
pub enum EntityType {
    Oak,
    AppleTree,
    Birch,
    Pine,
    Spruce,
    Cactus,
    WorkerAnt,
    QueenAnt,
    DroneAnt,
    SoldierAnt,
    Mechant,
    Snail,
    FoodStorage,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "RequestType")]
pub enum RequestType {
    Chunk,
    Data,
}

pub fn get_descriptions_for_tiles() -> HashMap<TileType, String> {
    return HashMap::from([
        (TileType::Grass, "Grass".to_string()),
        (TileType::Ice, "Ice".to_string()),
    ]);
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Biome {
    pub name: String,
    pub temperature: i32,
    pub tile_type: TileType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tile_type: TileType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorldRequest {
    pub x: i32,
    pub y: i32,
    pub req_type: RequestType,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
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
    pub category_type: CategoryType,
    pub faction: String,
    pub faction_id: i32,
    pub current_action: ActionType,
    pub wielding_item: ItemType,
    pub backpack_item: ItemType,
    pub wearable_item: ItemType,
    pub backpack_amount: u8,
}
impl Entity {
    pub fn idle_mov(&mut self) {
        if !self.stopped {
            let mut rng = rand::thread_rng();
            self.x += self.dir.cos() * self.speed;
            self.y += self.dir.sin() * self.speed;
            self.dir = rng.gen_range(0.0..3.14 * 2.0);
        }
    }
    pub fn mov(&mut self) {
        if !self.stopped {
            self.dir = (self.y - self.target_y).atan2(self.x - self.target_x);
            self.x -= self.dir.cos() * self.speed;
            self.y -= self.dir.sin() * self.speed;
        }
    }
    pub fn stop(&mut self) {
        self.stopped = true;
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorldResponse {
    pub chunk: Chunk,
    pub valid: bool,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Chunk {
    pub points: Vec<Vec<Point>>,
    pub entities: Vec<Entity>,
    pub name: String,
    pub id: i32,
}
impl Chunk {
    pub fn assign_name(&mut self, s: String) {
        self.name = s.clone();
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorldData {
    pub name: String,
    pub sea_level: f32,
    pub width: usize,
    pub height: usize,
    pub chunk_size: usize,
    pub tile_size: i32,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct World {
    pub chunks: Vec<Vec<Chunk>>,
    pub world_data: WorldData,
}

impl World {
    pub fn update_entities(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..self.world_data.width {
            for j in 0..self.world_data.height {
                let mut chunks_entities: Vec<Entity> = vec![];
                let mut chunk_range_min_k = (i as i32 - CHUNKRANGE as i32) as i32;
                if chunk_range_min_k < 0 {
                    chunk_range_min_k = 0;
                }
                let mut chunk_range_max_k = (i as i32 + CHUNKRANGE as i32) as i32;
                if chunk_range_max_k > (self.world_data.width - 1) as i32 {
                    chunk_range_max_k = (self.world_data.width - 1) as i32;
                }
                let mut chunk_range_min_h = (j as i32 - CHUNKRANGE as i32);
                if chunk_range_min_h < 0 {
                    chunk_range_min_h = 0;
                }
                let mut chunk_range_max_h = (j as i32 + CHUNKRANGE as i32);
                if chunk_range_max_h > (self.world_data.height - 1) as i32 {
                    chunk_range_max_h = (self.world_data.height - 1) as i32;
                }

                for k in chunk_range_min_k as usize..chunk_range_max_k as usize {
                    for h in chunk_range_min_h as usize..chunk_range_max_h as usize {
                        for e in self.chunks[k][h].entities.clone() {
                            chunks_entities.push(e);
                        }
                    }
                }
                for e in self.chunks[i][j].entities.iter_mut() {
                    if e.category_type == CategoryType::Ant {
                        if e.current_action == ActionType::Idle {
                            e.idle_mov();
                        } else {
                            e.mov();
                        }
                    }
                    if e.entity_type == EntityType::FoodStorage {
                        for v in chunks_entities.iter() {
                            let dist_from_entity =
                                ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                            if v.backpack_item == ItemType::Fruit
                                || v.backpack_item == ItemType::Meat
                            {
                                if dist_from_entity < INTERACTION_SIZE {
                                    if e.backpack_amount + 1 < BACKPACKSIZE {
                                        e.backpack_item = ItemType::Fruit;
                                        e.backpack_amount += 1;
                                    }
                                }
                            }
                        }
                    } else if e.entity_type == EntityType::WorkerAnt {
                        if e.current_action == ActionType::ReturnFood {
                            for v in chunks_entities.iter() {
                                let dist_from_entity =
                                    ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                                if e.backpack_item == ItemType::Fruit
                                    || e.backpack_item == ItemType::Meat
                                {
                                    if v.entity_type == EntityType::FoodStorage
                                        && v.faction == e.faction
                                    {
                                        e.target_x = v.x;
                                        e.target_y = v.y;
                                        if dist_from_entity < INTERACTION_SIZE {
                                            e.backpack_item = ItemType::Nothing;
                                            e.current_action = ActionType::Idle;
                                            e.target_x = 0.0;
                                            e.target_y = 0.0;
                                        }
                                    }
                                }
                            }
                        } else if e.current_action == ActionType::Idle {
                            if e.target_x == 0.0 && e.target_y == 0.0 {
                                e.current_action = ActionType::Explore;
                                e.target_x = e.x + rng.gen_range(-128.0..128.0);
                                e.target_y = e.y + rng.gen_range(-128.0..128.0);
                            }
                        } else if e.current_action == ActionType::Explore {
                            if e.x > e.target_x - TARGET_SIZE
                                && e.y > e.target_y - TARGET_SIZE
                                && e.x < e.target_x + TARGET_SIZE
                                && e.y < e.target_y + TARGET_SIZE
                            {
                                e.current_action = ActionType::Idle;
                                e.target_x = 0.0;
                                e.target_y = 0.0;
                            }

                            for v in chunks_entities.iter() {
                                let dist_from_entity =
                                    ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                                if dist_from_entity < VICINITY_SIZE {
                                    if v.entity_type == EntityType::AppleTree
                                        || v.entity_type == EntityType::Cactus
                                    {
                                        e.current_action = ActionType::FetchFood;
                                        e.target_x = v.x;
                                        e.target_y = v.y;
                                    }
                                }
                            }
                        } else if e.current_action == ActionType::FetchFood {
                            for v in chunks_entities.iter() {
                                let dist_from_entity =
                                    ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                                if dist_from_entity < INTERACTION_SIZE {
                                    if v.entity_type == EntityType::AppleTree
                                        || v.entity_type == EntityType::Cactus
                                    {
                                        e.backpack_item = ItemType::Fruit;
                                        e.current_action = ActionType::ReturnFood;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
