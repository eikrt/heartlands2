use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
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
impl Default for Point {
    fn default() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            tile_type: TileType::Grass,
        }
    }
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
    pub fn to_json(&self) -> String {
        return "".to_string();
        //serde_json::to_str(entities.iter().skip(1).map(|(_, e)| e.to_json().fold(first.to_json() |acc,s|)).unwrap();
    }
}
impl Default for Entity {
    fn default() -> Entity {
        Entity {
            x: 0.0,
            y: 0.0,
            id: 0,
            speed: 0.0,
            dir: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            stopped: true,
            entity_type: EntityType::Oak,
            category_type: CategoryType::Tree,
            faction: "Neutral".to_string(),
            faction_id: 0,
            current_action: ActionType::Idle,
            wielding_item: ItemType::Nothing,
            wearable_item: ItemType::Nothing,
            backpack_amount: 0,
            backpack_item: ItemType::Nothing,
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorldResponse {
    pub chunk: Chunk,
    pub world_data: WorldData,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub points: Vec<Vec<Point>>,
    pub entities: HashMap<i32, Entity>,
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
    pub is_default: bool,
}
impl Default for WorldData {
    fn default() -> WorldData {
        WorldData {
            name: "Default name".to_string(),
            sea_level: 0.0,
            width: 1,
            height: 1,
            chunk_size: 1,
            tile_size: 1,
            is_default: true,
        }
    }
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct World {
    pub chunks: Vec<Vec<Chunk>>,
    pub world_data: WorldData,
    pub v_x: i32, // slice dimensions for formatting
    pub v_y: i32,
    pub v_w: i32,
    pub v_h: i32,
}
impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut view_x =
            self.v_x / self.world_data.tile_size / self.world_data.chunk_size as i32 - self.v_w;
        let mut view_y =
            self.v_y / self.world_data.tile_size / self.world_data.chunk_size as i32 - self.v_h;
        let mut view_width = view_x + self.v_w * 2;
        let mut view_height = view_y + self.v_h * 2;
        if view_x < 0 {
            view_x = 0;
        }
        if view_y < 0 {
            view_y = 0;
        }
        if view_x > self.chunks.len() as i32 - 2 {
            view_x = self.chunks.len() as i32 - 2;
        }
        if view_y > self.chunks.len() as i32 - 2 {
            view_y = self.chunks.len() as i32 - 2;
        }

        if view_width < 0 {
            view_width = 1;
        }
        if view_height < 0 {
            view_height = 1;
        }
        if view_width > self.chunks.len() as i32 - 1 {
            view_width = self.chunks.len() as i32 - 1;
        }
        if view_height > self.chunks.len() as i32 - 1 {
            view_height = self.chunks.len() as i32 - 1;
        }
        /*let mut selected_chunks: Vec<Vec<Chunk>> = Vec::from(Vec::new());
        for i in view_x as usize..view_height as usize as usize {
            selected_chunks.push(Vec::new());
            for j in view_y as usize..view_height as usize {
                println!("{}", i - view_x as usize);
                selected_chunks[i - view_x as usize].push(self.chunks[i][j].clone());
            }
        }*/

        let mut selected_chunks = self.chunks.clone();
        let mut selected_chunks2 = self.chunks.clone();
        for i in 0..selected_chunks.len() {
            for j in 0..selected_chunks.len() {
                selected_chunks2[i].retain(|x| {
                    x.x > view_x && x.x < view_width && x.y > view_y && x.y < view_height
                });
            }
        }

        write!(
            f,
            "{{\"chunks\": {}, \"world_data\": {}, \"v_x\": {}, \"v_y\": {}, \"v_w\": {}, \"v_h\": {}}}",
            serde_json::to_string(&selected_chunks2).unwrap(),
            serde_json::to_string(&self.world_data).unwrap(),
            serde_json::to_string(&view_x).unwrap(),
            serde_json::to_string(&view_y).unwrap(),
            serde_json::to_string(&view_width).unwrap(),
            serde_json::to_string(&view_height).unwrap(),

        )
        //write!(f, "{}", serde_json::to_string(self).unwrap())
        //write!(f, "\"x\":{}", 5)
    }
}
impl World {
    pub fn get(&self, x: i32, y: i32) -> String {
        let mut view_x =
            x / self.world_data.tile_size / self.world_data.chunk_size as i32 - self.v_w;
        let mut view_y =
            y / self.world_data.tile_size / self.world_data.chunk_size as i32 - self.v_h;
        let mut view_width = view_x + self.v_w * 2;
        let mut view_height = view_y + self.v_h * 2;
        if view_x < 0 {
            view_x = 0;
        }
        if view_y < 0 {
            view_y = 0;
        }
        if view_x > self.chunks.len() as i32 - 2 {
            view_x = self.chunks.len() as i32 - 2;
        }
        if view_y > self.chunks.len() as i32 - 2 {
            view_y = self.chunks.len() as i32 - 2;
        }

        if view_width < 0 {
            view_width = 1;
        }
        if view_height < 0 {
            view_height = 1;
        }
        if view_width > self.chunks.len() as i32 - 1 {
            view_width = self.chunks.len() as i32 - 1;
        }
        if view_height > self.chunks.len() as i32 - 1 {
            view_height = self.chunks.len() as i32 - 1;
        }
        /*let mut selected_chunks: Vec<Vec<Chunk>> = Vec::from(Vec::new());
        for i in view_x as usize..view_height as usize as usize {
            selected_chunks.push(Vec::new());
            for j in view_y as usize..view_height as usize {
                println!("{}", i - view_x as usize);
                selected_chunks[i - view_x as usize].push(self.chunks[i][j].clone());
            }
        }*/

        let mut selected_chunks = self.chunks.clone();
        let mut selected_chunks2 = self.chunks.clone();
        for i in 0..selected_chunks.len() {
            for j in 0..selected_chunks.len() {
                selected_chunks2[i].retain(|x| {
                    x.x > view_x && x.x < view_width && x.y > view_y && x.y < view_height
                });
            }
        }

        format!(
            "{{\"chunks\": {}, \"world_data\": {}, \"v_x\": {}, \"v_y\": {}, \"v_w\": {}, \"v_h\": {}}}",
            serde_json::to_string(&selected_chunks2).unwrap(),
            serde_json::to_string(&self.world_data).unwrap(),
            serde_json::to_string(&view_x).unwrap(),
            serde_json::to_string(&view_y).unwrap(),
            serde_json::to_string(&view_width).unwrap(),
            serde_json::to_string(&view_height).unwrap(),

        )
        //write!(f, "{}", serde_json::to_string(self).unwrap())
        //write!(f, "\"x\":{}", 5)
    }
    pub fn update_entities(&mut self) {
        let mut rng = rand::thread_rng();
        /*for i in 0..self.world_data.width {
            for j in 0..self.world_data.height {
                for (k, v) in self.chunks[i][j].entities.iter_mut() {
                    v.idle_mov();
                }
            }
        }*/
        for i in 0..self.world_data.width {
            for j in 0..self.world_data.height {
                let mut chunks_entities: HashMap<i32, Entity> = HashMap::new();
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
                        let ents: Vec<Entity> =
                            self.chunks[k][h].entities.values().cloned().collect();
                        for e in ents {
                            chunks_entities.insert(e.id, e);
                        }
                    }
                }
                for (key, e) in self.chunks[i][j].entities.iter_mut() {
                    if e.category_type == CategoryType::Ant {
                        if e.current_action == ActionType::Idle {
                            e.idle_mov();
                        } else {
                            e.mov();
                        }
                    }
                    if e.entity_type == EntityType::FoodStorage {
                        for (key2, v) in chunks_entities.iter_mut() {
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
                            for (key2, v) in chunks_entities.iter_mut() {
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

                            for (key2, v) in chunks_entities.iter_mut() {
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
                            for (key, v) in chunks_entities.iter_mut() {
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
