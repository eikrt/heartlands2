use crate::client_structs::ClientPacket;
use crate::client_structs::Player;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
const TARGET_SIZE: f32 = 8.0;
const PROP_SIZE: i32 = 16;
const VICINITY_SIZE: f32 = 96.0;
const INTERACTION_SIZE: f32 = 8.0;
const CHUNKRANGE: usize = 2;
const REPRODUCE_CHANCE: usize = 256;
const BACKPACKSIZE: u8 = 64;
const INTERACTION_COOLDOWN: u128 = 10;
pub const HATCH_TIME: u128 = 10000;
pub const LETHAL_RANGE: f32 = 16.0;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "PropType")]
pub enum PropType {
    Raft,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "ColliderType")]
pub enum ColliderType {
    Meteoroid,
    SoulTrap,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "ReligionType")]
pub enum ReligionType {
    Plasma,
    Moon,
    Technology,
    Giants,
    Element,
    Spiral,
    Infinity,
    Sacrifice,
    Nothing,
}
impl Distribution<ReligionType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ReligionType {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=7) {
            0 => ReligionType::Plasma,
            1 => ReligionType::Moon,
            2 => ReligionType::Technology,
            3 => ReligionType::Giants,
            4 => ReligionType::Element,
            5 => ReligionType::Spiral,
            6 => ReligionType::Infinity,
            7 => ReligionType::Sacrifice,
            _ => ReligionType::Nothing,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(tag = "TaskType")]
pub enum TaskType {
    Reproduce,
    Nothing,
    FindFood,
    Hunt,
    Trade,
    Defend,
    Terrorize,
}
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
    FetchFoodForEating,
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
    CultistAnt,
    Plasmant,
    Mechant,
    Snail,
    FoodStorage,
    AntEgg,
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
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub speed: f32,
    pub dir: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub stopped: bool,
    pub id: i32,
    pub entity_type: EntityType,
    pub religion_type: ReligionType,
    pub category_type: CategoryType,
    pub faction: String,
    pub faction_id: i32,
    pub current_action: ActionType,
    pub task_type: TaskType,
    pub wielding_item: ItemType,
    pub backpack_item: ItemType,
    pub wearable_item: ItemType,
    pub backpack_amount: u8,
    pub time: u128,
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
    pub fn pick_fruit(&mut self) {
        self.backpack_item = ItemType::Fruit;
        if self.backpack_amount < BACKPACKSIZE {
            self.backpack_amount += 1;
        }
    }
    pub fn drop_item(&mut self) {
        if self.backpack_amount > 0 {
            self.backpack_amount -= 1;
        }
    }
    pub fn drop_items(&mut self) {
        self.backpack_item = ItemType::Nothing;
        self.backpack_amount = 0;
    }
    pub fn idle(&mut self) {
        self.current_action = ActionType::Idle;
        self.target_x = 0.0;
        self.target_y = 0.0;
    }
    pub fn wander(&mut self) {
        let mut rng = rand::thread_rng();
        self.target_x = self.x + rng.gen_range(-128.0..128.0);
        self.target_y = self.y + rng.gen_range(-128.0..128.0);
    }
    pub fn tick(&mut self) {
        self.time += 10;
    }
}
impl Default for Entity {
    fn default() -> Entity {
        Entity {
            x: 0.0,
            y: 0.0,
            hp: 1,
            id: 0,
            speed: 0.0,
            dir: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            stopped: true,
            entity_type: EntityType::Oak,
            category_type: CategoryType::Tree,
            religion_type: ReligionType::Nothing,
            faction: "Neutral".to_string(),
            faction_id: 0,
            current_action: ActionType::Idle,
            wielding_item: ItemType::Nothing,
            wearable_item: ItemType::Nothing,
            task_type: TaskType::Nothing,
            backpack_amount: 0,
            backpack_item: ItemType::Nothing,
            time: 0,
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
    pub religion: ReligionType,
    pub id: i32,
}
impl Chunk {
    pub fn assign_name(&mut self, s: String) {
        self.name = s.clone();
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Prop {
    pub x: f32,
    pub y: f32,
    pub prop_type: PropType,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Collider {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub life_y: f32,
    pub speed: f32,
    pub dir: f32,
    pub collider_type: ColliderType,
    pub lethal: bool,
    pub owner_id: i32,
}
impl Collider {
    pub fn mov(&mut self) {
        self.x += self.dir.cos() * self.speed;
        self.y += self.dir.sin() * self.speed;
    }
    pub fn tick(&mut self) {
        if self.y > self.life_y {
            self.hp = -1;
        }
        if self.y > self.life_y - LETHAL_RANGE {
            self.lethal = true;
        }
    }
    pub fn collide(&mut self, entity: &mut Entity) -> String {
        let size = 16.0;
        if self.x > entity.x
            && self.x < entity.x + size
            && self.y > entity.y
            && self.y < entity.y + size
        {
            if self.lethal {
                if self.collider_type == ColliderType::Meteoroid {
                    entity.hp = -1;
                    return "killed".to_string();
                }
            }
            if self.collider_type == ColliderType::SoulTrap {
                entity.hp = -1;
                self.hp = -1;
                return "soul gathered".to_string();
            }
        }
        return "success".to_string();
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
    pub players: Vec<Player>,
    pub colliders: Vec<Collider>,
    pub props: Vec<Prop>,
    pub v_x: i32, // slice dimensions for formatting
    pub v_y: i32,
    pub v_w: i32,
    pub v_h: i32,
}
impl World {
    pub fn get(&self, x: i32, y: i32) -> String {
        let mut view_x =
            x / self.world_data.tile_size / self.world_data.chunk_size as i32 - self.v_w + 1;
        let mut view_y =
            y / self.world_data.tile_size / self.world_data.chunk_size as i32 - self.v_h + 1;
        let mut view_width = view_x + self.v_w * 2;
        let mut view_height = view_y + self.v_h * 2;
        if view_x < 0 {
            view_x = 0;
        }
        if view_y < 0 {
            view_y = 0;
        }
        if view_x > self.chunks.len() as i32 - 0 {
            view_x = self.chunks.len() as i32 - 0;
        }
        if view_y > self.chunks.len() as i32 - 0 {
            view_y = self.chunks.len() as i32 - 0;
        }

        if view_width < 0 {
            view_width = 1;
        }
        if view_height < 0 {
            view_height = 1;
        }
        if view_width > self.chunks.len() as i32 - 0 {
            view_width = self.chunks.len() as i32 - 0;
        }
        if view_height > self.chunks.len() as i32 - 0 {
            view_height = self.chunks.len() as i32 - 0;
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
            "{{\"chunks\": {}, \"world_data\": {}, \"players\": {}, \"colliders\": {}, \"props\": {}, \"v_x\": {}, \"v_y\": {}, \"v_w\": {}, \"v_h\": {}}}",
            serde_json::to_string(&selected_chunks2).unwrap(),
            serde_json::to_string(&self.world_data).unwrap(),
            serde_json::to_string(&self.players).unwrap(),
            serde_json::to_string(&self.colliders).unwrap(),
            serde_json::to_string(&self.props).unwrap(),
            serde_json::to_string(&view_x).unwrap(),
            serde_json::to_string(&view_y).unwrap(),
            serde_json::to_string(&view_width).unwrap(),
            serde_json::to_string(&view_height).unwrap(),

        )
        //write!(f, "{}", serde_json::to_string(self).unwrap())
        //write!(f, "\"x\":{}", 5)
    }

    pub fn update_political_and_religion_situation(&mut self) {
        // political
        let mut biggest_value_data = (0, 0, "Neutral".to_string());
        let ant_number_to_change_ownership = 1;
        for row in self.chunks.clone().iter().enumerate() {
            for c in self.chunks[row.0].clone().iter().enumerate() {
                let mut entity_types: HashMap<String, i32> = HashMap::new();
                if (c.1.entities.len() as i32) < ant_number_to_change_ownership {
                    self.chunks[row.0][c.0].name = "Neutral".to_string();
                }
                for e in c.1.entities.values() {
                    if !entity_types.contains_key(&e.faction) {
                        if e.entity_type == EntityType::DroneAnt {
                            entity_types.insert(e.faction.clone(), 0);
                        }
                    } else {
                        if e.entity_type == EntityType::DroneAnt {
                            *entity_types.get_mut(&e.faction).unwrap() += 1;
                        }
                    }
                    let mut biggest_value = ("Neutral".to_string(), 0);
                    for (key, value) in &entity_types {
                        if value > &biggest_value.1 {
                            biggest_value = (key.to_string(), *value);
                        }
                    }
                    biggest_value_data = (row.0, c.0, biggest_value.0);
                    if biggest_value.1 <= ant_number_to_change_ownership {
                        biggest_value_data = (row.0, c.0, "Neutral".to_string());
                    }
                    self.chunks[biggest_value_data.0][biggest_value_data.1].name =
                        biggest_value_data.2;
                }
            }
        }

        // religion
        let mut biggest_value_data = (0, 0, ReligionType::Nothing);
        let ant_number_to_change_religion = 1;
        for row in self.chunks.clone().iter().enumerate() {
            for c in self.chunks[row.0].clone().iter().enumerate() {
                let mut entity_types: HashMap<ReligionType, i32> = HashMap::new();
                if (c.1.entities.len() as i32) < ant_number_to_change_ownership {
                    self.chunks[row.0][c.0].religion = ReligionType::Nothing;
                }
                for e in c.1.entities.values() {
                    if !entity_types.contains_key(&e.religion_type) {
                        if e.category_type == CategoryType::Ant {
                            entity_types.insert(e.religion_type.clone(), 0);
                        }
                    } else {
                        if e.category_type == CategoryType::Ant {
                            *entity_types.get_mut(&e.religion_type).unwrap() += 1;
                        }
                    }
                    let mut biggest_value = (ReligionType::Nothing, 0);
                    for (key, value) in &entity_types {
                        if value > &biggest_value.1 {
                            biggest_value = (key.clone(), *value);
                        }
                    }
                    biggest_value_data = (row.0, c.0, biggest_value.0);
                    if biggest_value.1 <= ant_number_to_change_ownership {
                        biggest_value_data = (row.0, c.0, ReligionType::Nothing);
                    }
                    self.chunks[biggest_value_data.0][biggest_value_data.1].religion =
                        biggest_value_data.2;
                }
            }
        }
    }
    pub fn update_entities(&mut self) {
        for collider in self.colliders.iter_mut() {
            collider.mov();
            collider.tick();
            for row in self.chunks.iter_mut() {
                for chunk in row.iter_mut() {
                    for (key, val) in chunk.entities.iter_mut() {
                        collider.collide(val);
                    }
                }
            }
        }
        self.colliders.retain(|x| x.hp > 0);
        let mut add_entities = HashMap::new();
        for i in 0..self.world_data.width {
            for j in 0..self.world_data.height {
                let mut id = 0;
                let mut rng = rand::thread_rng();
                for entity in self.chunks[i][j].entities.clone().values() {
                    if entity.x
                        < self.chunks[i][j].points[0][0].x * self.world_data.tile_size as f32
                    {
                        id = entity.id;
                        if i > 0 {
                            self.chunks[i - 1][j].entities.insert(id, entity.clone());
                        }
                    } else if entity.y
                        < self.chunks[i][j].points[0][0].y * self.world_data.tile_size as f32
                    {
                        id = entity.id;
                        if j > 0 {
                            self.chunks[i][j - 1].entities.insert(id, entity.clone());
                        }
                    } else if entity.x
                        > self.chunks[i][j].points[self.world_data.chunk_size - 1]
                            [self.world_data.chunk_size - 1]
                            .x
                            * self.world_data.tile_size as f32
                    {
                        id = entity.id;
                        if i < self.world_data.width - 1 {
                            self.chunks[i + 1][j].entities.insert(id, entity.clone());
                        }
                    } else if entity.y
                        > self.chunks[i][j].points[self.world_data.chunk_size - 1]
                            [self.world_data.chunk_size - 1]
                            .y
                            * self.world_data.tile_size as f32
                    {
                        id = entity.id;
                        if j < self.world_data.height - 1 {
                            self.chunks[i][j + 1].entities.insert(id, entity.clone());
                        }
                    }
                    if self.chunks[i][j].entities.len() > 0 {
                        self.chunks[i][j].entities.remove(&id);
                    }
                }
            }
        }

        let mut rng = rand::thread_rng();
        for i in 0..self.world_data.width {
            for j in 0..self.world_data.height {
                let chunk_clone = self.chunks[i][j].clone();
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
                    e.tick();
                    if e.hp < 0 {
                        continue;
                    }
                    if e.entity_type == EntityType::AntEgg && e.time > HATCH_TIME {
                        let id = rng.gen_range(0..999999);
                        e.hp = -1;
                        add_entities.insert(
                            chunk_clone.id,
                            Entity {
                                id: id,
                                hp: 100,
                                x: e.x as f32,
                                y: e.y as f32,
                                stopped: false,
                                speed: 1.5,
                                dir: 0.0,
                                target_x: 0.0,
                                target_y: 0.0,
                                entity_type: EntityType::WorkerAnt,
                                category_type: CategoryType::Ant,
                                religion_type: ReligionType::Nothing,
                                faction: chunk_clone.name.clone().to_string(),
                                faction_id: chunk_clone.id,
                                current_action: ActionType::Idle,
                                task_type: TaskType::Nothing,
                                wielding_item: ItemType::WoodenShovel,
                                backpack_item: ItemType::Nothing,
                                wearable_item: ItemType::Nothing,
                                backpack_amount: 0,
                                time: 0,
                            },
                        );
                    }
                    if e.entity_type == EntityType::WorkerAnt && e.task_type == TaskType::Nothing {
                        let random_task = rng.gen_range(0..1);
                        if random_task == 0 {
                            e.task_type = TaskType::FindFood;
                        } else if random_task == 1 {
                            e.task_type = TaskType::FindFood;
                        }
                    }
                    if e.entity_type == EntityType::DroneAnt && e.task_type == TaskType::Nothing {
                        let random_task = rng.gen_range(0..1);
                        if random_task == 0 {
                            e.task_type = TaskType::Reproduce;
                        } else if random_task == 1 {
                            e.task_type = TaskType::Reproduce;
                        }
                    }
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
                            if v.entity_type == EntityType::WorkerAnt
                                && (v.backpack_item == ItemType::Fruit
                                    || v.backpack_item == ItemType::Meat)
                                && v.faction == e.faction
                            {
                                if dist_from_entity < INTERACTION_SIZE
                                    && e.time % INTERACTION_COOLDOWN == 0
                                {
                                    e.pick_fruit();
                                }
                            } else if v.entity_type == EntityType::DroneAnt
                                && (v.backpack_item == ItemType::Nothing)
                            {
                                if dist_from_entity < INTERACTION_SIZE
                                    && e.time % INTERACTION_COOLDOWN == 0
                                {
                                    e.drop_item();
                                }
                            }
                        }
                    } else if e.entity_type == EntityType::WorkerAnt {
                        if e.current_action == ActionType::ReturnFood {
                            let mut found_storage = false;
                            for (key2, v) in chunks_entities.iter_mut() {
                                let dist_from_entity =
                                    ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                                if e.backpack_item == ItemType::Fruit
                                    || e.backpack_item == ItemType::Meat
                                {
                                    if v.entity_type == EntityType::FoodStorage
                                        && v.faction == e.faction
                                    {
                                        found_storage = true;
                                        e.target_x = v.x;
                                        e.target_y = v.y;
                                        if dist_from_entity < INTERACTION_SIZE {
                                            e.idle();
                                        }
                                    }
                                }
                            }
                            if !found_storage {
                                e.drop_items();
                                e.idle();
                                found_storage = true;
                            }
                        } else if e.current_action == ActionType::Idle {
                            if e.target_x == 0.0 && e.target_y == 0.0 {
                                e.current_action = ActionType::Explore;
                                e.wander();
                            }
                        } else if e.current_action == ActionType::Explore {
                            if e.time % 100 == 0 {
                                e.drop_items();
                            }
                            if e.x > e.target_x - TARGET_SIZE
                                && e.y > e.target_y - TARGET_SIZE
                                && e.x < e.target_x + TARGET_SIZE
                                && e.y < e.target_y + TARGET_SIZE
                            {
                                e.idle();
                            }
                            for (key2, v) in chunks_entities.iter_mut() {
                                let dist_from_entity =
                                    ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                                if e.task_type == TaskType::FindFood
                                    && dist_from_entity < VICINITY_SIZE
                                {
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
                    } else if e.entity_type == EntityType::DroneAnt {
                        if e.task_type == TaskType::Reproduce {
                            for (key, v) in chunks_entities.iter_mut() {
                                let dist_from_entity =
                                    ((e.x - v.x).powf(2.0) + (e.y - v.y).powf(2.0) as f32).sqrt();
                                if e.current_action == ActionType::Idle
                                    && v.faction == e.faction
                                    && v.backpack_amount > 0
                                {
                                    e.current_action = ActionType::FetchFoodForEating;
                                    e.target_x = v.x;
                                    e.target_y = v.y;
                                } else if e.current_action == ActionType::FetchFoodForEating {
                                    if v.entity_type == EntityType::FoodStorage
                                        && v.faction == e.faction
                                    {
                                        e.target_x = v.x;
                                        e.target_y = v.y;
                                        if dist_from_entity < INTERACTION_SIZE {
                                            e.pick_fruit();
                                            e.current_action = ActionType::Breed;
                                        }
                                    }
                                } else if e.current_action == ActionType::Breed
                                    && v.faction == e.faction
                                    && v.entity_type == EntityType::QueenAnt
                                {
                                    e.target_x = v.x;
                                    e.target_y = v.y;
                                    if dist_from_entity < INTERACTION_SIZE {
                                        e.drop_items();
                                        e.idle();
                                        let id = rng.gen_range(0..999999);
                                        if rng.gen_range(0..REPRODUCE_CHANCE) == 0 {
                                            add_entities.insert(
                                                chunk_clone.id,
                                                Entity {
                                                    id: id,
                                                    hp: 1,
                                                    x: e.x + rng.gen_range(-32.0..32.0),
                                                    y: e.y + rng.gen_range(-32.0..32.0),
                                                    dir: 0.0,
                                                    target_x: 0.0,
                                                    target_y: 0.0,
                                                    speed: 0.0,
                                                    stopped: true,
                                                    entity_type: EntityType::AntEgg,
                                                    category_type: CategoryType::Vegetation,
                                                    religion_type: ReligionType::Nothing,
                                                    faction: chunk_clone.name.clone().to_string(),
                                                    faction_id: chunk_clone.id,
                                                    current_action: ActionType::Idle,
                                                    task_type: TaskType::Nothing,
                                                    backpack_item: ItemType::Nothing,
                                                    wearable_item: ItemType::Nothing,
                                                    wielding_item: ItemType::Nothing,
                                                    backpack_amount: 0,
                                                    time: 0,
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for (key, val) in add_entities.iter() {
            for i in 0..self.chunks.len() {
                for j in 0..self.chunks.len() {
                    if &self.chunks[i][j].id == key {
                        self.chunks[i][j].entities.insert(val.id, val.clone());
                    }
                }
            }
        }
    }
}
