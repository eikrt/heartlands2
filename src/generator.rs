use crate::client_structs::Player;
use crate::world_structs::{
    ActionType, Biome, CategoryType, Chunk, Entity, EntityType, Faction, ItemType, Point,
    ReligionType, TaskType, TileType, World, WorldData,
};
use rand::seq::IteratorRandom;
use rand::Rng;
use simdnoise::*;
use std::collections::HashMap;
use std::env;
use std::fs;
const DRONE_MIN: i32 = 2;
const DRONE_MAX: i32 = 3;
const SOLDIER_MIN: i32 = 1;
const SOLDIER_MAX: i32 = 2;
const WORKER_MIN: i32 = 1;
const WORKER_MAX: i32 = 2;
pub fn generate(
    seed: i32,
    width: usize,
    height: usize,
    chunk_size: usize,
    sea_level: f32,
    name: String,
) -> World {
    let tile_size = 16;
    let temperature_margin = 5;
    let biomes: Vec<Biome> = vec![
        Biome {
            name: "glacier".to_string(),
            temperature: 0,
            tile_type: TileType::Ice,
        },
        Biome {
            name: "tundra".to_string(),
            temperature: 5,
            tile_type: TileType::PermaFrost,
        },
        Biome {
            name: "taiga".to_string(),
            temperature: 10,
            tile_type: TileType::ColdLand,
        },
        Biome {
            name: "forest".to_string(),
            temperature: 15,
            tile_type: TileType::ColdLand,
        },
        Biome {
            name: "grasslands".to_string(),
            temperature: 20,
            tile_type: TileType::Grass,
        },
        Biome {
            name: "mediterraean".to_string(),
            temperature: 25,
            tile_type: TileType::CoarseLand,
        },
        Biome {
            name: "savannah".to_string(),
            temperature: 30,
            tile_type: TileType::SavannahLand,
        },
        Biome {
            name: "desert".to_string(),
            temperature: 35,
            tile_type: TileType::Sand,
        },
        Biome {
            name: "red_desert".to_string(),
            temperature: 40,
            tile_type: TileType::RedSand,
        },
        Biome {
            name: "rainforest".to_string(),
            temperature: 45,
            tile_type: TileType::Grass,
        },
    ];
    let max_temp = 45;

    println!("Generating world...");
    let mut world_chunks: Vec<Vec<Chunk>> = Vec::new();
    let mut rng = rand::thread_rng();
    let ground_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(0.15)
        .with_octaves(9.0 as u8)
        .with_gain(2.0)
        .with_seed(seed)
        .with_lacunarity(0.8)
        .generate_scaled(0.0, 512.0);
    let sea_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(1000.15)
        .with_octaves(16.0 as u8)
        .with_gain(2.0)
        .with_seed(seed * 2)
        .with_lacunarity(0.4)
        .generate_scaled(0.0, 512.0);
    let biome_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(0.000003)
        .with_octaves(16)
        .with_gain(1.0)
        .with_seed(seed * 3)
        .with_lacunarity(2.0)
        .generate_scaled(-0.5, 0.5);
    let river_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(0.02)
        .with_octaves(9)
        .with_gain(1.2)
        .with_seed(seed * 4)
        .with_lacunarity(1.3)
        .generate_scaled(0.0, 1.0);
    let river_area_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(100.0)
        .with_octaves(9)
        .with_gain(1.2)
        .with_seed(seed * 5)
        .with_lacunarity(0.2)
        .generate_scaled(0.0, 1.0);
    let tree_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(0.1)
        .with_octaves(9)
        .with_gain(0.1)
        .with_seed(seed * 6)
        .with_lacunarity(5.0)
        .generate_scaled(0.0, 0.9);

    let vegetation_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(0.1)
        .with_octaves(9)
        .with_gain(0.1)
        .with_seed(seed * 7)
        .with_lacunarity(5.0)
        .generate_scaled(0.0, 0.9);
    let village_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(1.0)
        .with_octaves(8)
        .with_gain(1.0)
        .with_seed(seed * 8)
        .with_lacunarity(5.0)
        .generate_scaled(0.0, 1.0);

    let city_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(0.1)
        .with_octaves(16)
        .with_gain(1.0)
        .with_seed(seed * 9)
        .with_lacunarity(2.0)
        .generate_scaled(0.0, 1.0);
    let village_building_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(0.1)
        .with_octaves(32)
        .with_gain(0.0)
        .with_seed(seed * 10)
        .with_lacunarity(9.0)
        .generate_scaled(0.0, 1.0);

    let city_building_noise = NoiseBuilder::fbm_2d(chunk_size * width, chunk_size * height)
        .with_freq(0.1)
        .with_octaves(32)
        .with_gain(0.0)
        .with_seed(seed * 10)
        .with_lacunarity(5.0)
        .generate_scaled(0.0, 1.0);
    let tree_threshold = 0.4;
    let vegetation_threshold = 0.8;
    let village_threshold = 0.8;
    let city_threshold = 0.7;

    let village_building_threshold = 0.5;
    let _city_building_threshold = 0.1;
    let river_threshhold = 0.5;
    let apply_seas = true;
    let apply_ground = true;
    let apply_water = true;
    let apply_rivers = true;
    let apply_entities = true;
    let apply_settlements = true;
    let apply_villages = true;
    let apply_cities = false;
    let apply_trees = true;
    let apply_vegetation = true;
    let apply_objectives = true;
    // biomes and adding tiles
    for i in 0..width {
        world_chunks.push(vec![]);
        for j in 0..height {
            let mut chunk_points: Vec<Vec<Point>> = Vec::new();
            for k in 0..chunk_size {
                chunk_points.push(vec![]);
                for h in 0..chunk_size {
                    let rx = ((i * chunk_size) as usize + k) as f32;
                    let ry = ((j * chunk_size) as usize + h) as f32;
                    let rz = 0.0;
                    let mut biome_val =
                        biome_noise[(ry + rx * width as f32 * chunk_size as f32) as usize];

                    let dist_from_equator =
                        ((ry - (height as f32 * chunk_size as f32) / 2.0).powf(2.0) as f32).sqrt();
                    let rel = 1.0 - (dist_from_equator / ((height * chunk_size) / 2) as f32) - 0.2;
                    biome_val += rel;
                    let temp = (biome_val * max_temp as f32) as i32;

                    let mut biome = &biomes[0];
                    for b in biomes.iter() {
                        if temp > b.temperature - temperature_margin
                            && temp < b.temperature + temperature_margin
                        {
                            biome = b;
                            break;
                        }
                    }
                    chunk_points[k].push(Point {
                        x: rx,
                        y: ry,
                        z: rz,
                        tile_type: biome.tile_type.clone(),
                    });
                }
            }

            let religion_type: ReligionType = rand::random();

            world_chunks[i as usize].push(Chunk {
                x: i as i32,
                y: j as i32,
                points: chunk_points,
                entities: HashMap::new(),
                name: get_chunk_name(),
                religion: religion_type,
                id: rng.gen_range(0..999999),
            });
        }
    }
    for i in 0..width {
        for j in 0..height {
            let mut faction_type = world_chunks[i][j].name.clone().to_string();
            let mut minus_x = 1;
            let mut minus_y = 1;
            let mut plus_x = 1;
            let mut plus_y = 1;
            if i as i32 - minus_x >= 0 {
                if world_chunks[i - 1][j].name != "Neutral".to_string() {
                    faction_type = world_chunks[i - 1][j].name.clone();
                }
            }
            if j as i32 - minus_y >= 0 {
                if world_chunks[i][j - 1].name != "Neutral".to_string() {
                    faction_type = world_chunks[i][j - 1].name.clone();
                }
            }
            if i as i32 + plus_x <= (world_chunks[i].len() - 1 as usize) as i32 {
                if world_chunks[i + 1][j].name != "Neutral".to_string() {
                    faction_type = world_chunks[i + 1][j].name.clone();
                }
            }
            if j as i32 + plus_y <= (world_chunks.len() - 1 as usize) as i32 {
                if world_chunks[i][j + 1].name != "Neutral".to_string() {
                    faction_type = world_chunks[i][j + 1].name.clone();
                }
            }
            world_chunks[i][j].name = faction_type;
        }
    }
    for i in width..0 {
        for j in height..0 {
            let mut faction_type = world_chunks[i][j].name.clone().to_string();
            let mut minus_x = 1;
            let mut minus_y = 1;
            let mut plus_x = 1;
            let mut plus_y = 1;
            if i as i32 - minus_x >= 0 {
                if world_chunks[i - 1][j].name != "Neutral".to_string() {
                    faction_type = world_chunks[i - 1][j].name.clone();
                }
            }
            if j as i32 - minus_y >= 0 {
                if world_chunks[i][j - 1].name != "Neutral".to_string() {
                    faction_type = world_chunks[i][j - 1].name.clone();
                }
            }
            if i as i32 + plus_x <= (world_chunks[i].len() - 1 as usize) as i32 {
                if world_chunks[i + 1][j].name != "Neutral".to_string() {
                    faction_type = world_chunks[i + 1][j].name.clone();
                }
            }
            if j as i32 + plus_y <= (world_chunks.len() - 1 as usize) as i32 {
                if world_chunks[i][j + 1].name != "Neutral".to_string() {
                    faction_type = world_chunks[i][j + 1].name.clone();
                }
            }
            world_chunks[i][j].name = faction_type;
        }
    }
    // SEAS AND BIG SHAPES
    if apply_seas {
        for i in 0..width {
            for j in 0..height {
                for k in 0..chunk_size {
                    for h in 0..chunk_size {
                        let _rx = (i * chunk_size) + k;
                        let _ry = (j * chunk_size) + h;
                        let _rz = sea_noise[_ry + _rx * width * chunk_size];
                        let chunk = &mut world_chunks[i as usize][j as usize];
                        let point = &mut chunk.points[k][h];
                        point.z = _rz;
                    }
                }
            }
        }
    }

    // DETAILS
    if apply_ground {
        for i in 0..width {
            for j in 0..height {
                for k in 0..chunk_size {
                    for h in 0..chunk_size {
                        let _rx = ((i * chunk_size) as usize + k) as f32;
                        let _ry = ((j * chunk_size) as usize + h) as f32;
                        let _rz =
                            ground_noise[(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                        let chunk = &mut world_chunks[i as usize][j as usize];
                        let point = &mut chunk.points[k][h];
                        point.z += _rz;
                    }
                }
            }
        }
    }
    // RIVERS
    if apply_rivers {
        for i in 0..width {
            for j in 0..height {
                for k in 0..chunk_size {
                    for h in 0..chunk_size {
                        let _rx = ((i * chunk_size) as usize + k) as f32;
                        let _ry = ((j * chunk_size) as usize + h) as f32;
                        let _rz =
                            river_noise[(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                        let chunk = &mut world_chunks[i as usize][j as usize];
                        let point = &mut chunk.points[k][h];
                        let ra_value = river_area_noise
                            [(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                        if ra_value > 0.5 && _rz > river_threshhold && point.z > sea_level {
                            point.z = _rz * 512.0;
                        }
                    }
                }
            }
        }
    }
    // DETAILS
    if apply_water {
        for i in 0..width {
            for j in 0..height {
                for k in 0..chunk_size {
                    for h in 0..chunk_size {
                        let _rx = ((i * chunk_size) as usize + k) as f32;
                        let _ry = ((j * chunk_size) as usize + h) as f32;
                        let chunk = &mut world_chunks[i as usize][j as usize];
                        let point = &mut chunk.points[k][h];

                        if point.z < sea_level {
                            point.z = (512.0 - point.z);
                            point.tile_type = TileType::Water;
                        }
                    }
                }
            }
        }
    }

    if apply_settlements {
        if apply_villages {
            for i in 0..width {
                for j in 0..height {
                    let mut chunk_entities = HashMap::new();
                    for k in 0..chunk_size {
                        for h in 0..chunk_size {
                            let _rx = ((i * chunk_size) as usize + k) as f32;
                            let _ry = ((j * chunk_size) as usize + h) as f32;
                            let chunk = &mut world_chunks[i as usize][j as usize];
                            let point = &mut chunk.points[k][h];

                            let village_val = village_noise
                                [(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                            let village_building_val = village_building_noise
                                [(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                            if village_val > village_threshold {
                                if village_building_val > village_building_threshold
                                    && point.tile_type != TileType::Water
                                    && point.tile_type != TileType::Ice
                                    && point.tile_type != TileType::Sand
                                    && point.tile_type != TileType::RedSand
                                {
                                    point.tile_type = TileType::MudHiveFloor;

                                    for _l in 0..rng.gen_range(WORKER_MIN..WORKER_MAX) {
                                        let id = rng.gen_range(0..999999);
                                        chunk_entities.insert(
                                            id,
                                            Entity {
                                                id: id,
                                                target_id: 0,
                                                hp: 100,
                                                x: (_rx + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                y: (_ry + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                stopped: false,
                                                speed: 1.5,
                                                dir: 0.0,
                                                target_x: 0.0,
                                                target_y: 0.0,
                                                entity_type: EntityType::WorkerAnt,
                                                category_type: CategoryType::Ant,
                                                faction: chunk.name.clone().to_string(),
                                                faction_id: chunk.id,
                                                religion_type: chunk.religion.clone(),
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
                                    for l in 0..rng.gen_range(SOLDIER_MIN..SOLDIER_MAX) {
                                        let id = rng.gen_range(0..999999);
                                        chunk_entities.insert(
                                            id,
                                            Entity {
                                                id: id,
                                                target_id: 0,
                                                hp: 100,
                                                x: (_rx + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                y: (_ry + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                stopped: false,
                                                speed: 4.5,
                                                dir: 0.0,
                                                target_x: 0.0,
                                                target_y: 0.0,
                                                entity_type: EntityType::SoldierAnt,
                                                category_type: CategoryType::Ant,
                                                religion_type: chunk.religion.clone(),
                                                task_type: TaskType::Nothing,
                                                faction: chunk.name.clone().to_string(),
                                                faction_id: chunk.id,
                                                current_action: ActionType::Idle,
                                                wielding_item: ItemType::WoodenSpear,
                                                backpack_item: ItemType::Nothing,
                                                wearable_item: ItemType::Nothing,
                                                backpack_amount: 0,
                                                time: 0,
                                            },
                                        );
                                    }
                                    for _l in 0..rng.gen_range(DRONE_MIN..DRONE_MAX) {
                                        let id = rng.gen_range(0..999999);
                                        chunk_entities.insert(
                                            id,
                                            Entity {
                                                id: id,
                                                target_id: 0,
                                                x: (_rx + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                y: (_ry + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                hp: 100,
                                                stopped: false,
                                                speed: 1.5,
                                                dir: 0.0,
                                                target_x: 0.0,
                                                target_y: 0.0,
                                                entity_type: EntityType::DroneAnt,
                                                task_type: TaskType::Nothing,
                                                category_type: CategoryType::Ant,
                                                faction: chunk.name.clone().to_string(),
                                                faction_id: chunk.id,
                                                current_action: ActionType::Idle,
                                                religion_type: chunk.religion.clone(),
                                                wielding_item: ItemType::Nothing,
                                                backpack_item: ItemType::Nothing,
                                                wearable_item: ItemType::Nothing,
                                                backpack_amount: 0,
                                                time: 0,
                                            },
                                        );
                                    }
                                    /*for _l in 0..rng.gen_range(1..2) {
                                        let id = rng.gen_range(0..999999);
                                        chunk_entities.insert(
                                            id,
                                            Entity {
                                                id: id,
                                                x: (_rx + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                y: (_ry + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                stopped: false,
                                                speed: 0.5,
                                                dir: 0.0,
                                                hp: 100,
                                                target_x: 0.0,
                                                target_y: 0.0,
                                                entity_type: EntityType::Mechant,
                                                category_type: CategoryType::Ant,
                                                religion_type: ReligionType::Plasma,
                                                task_type: TaskType::Nothing,
                                                faction: chunk.name.clone().to_string(),
                                                faction_id: chunk.id,
                                                current_action: ActionType::Idle,
                                                wielding_item: ItemType::Nothing,
                                                backpack_item: ItemType::Nothing,
                                                wearable_item: ItemType::Nothing,
                                                backpack_amount: 0,
                                                time: 0,
                                            },
                                        );
                                    }*/
                                    let mut has_queen = false;
                                    for e in chunk_entities.values() {
                                        if e.faction == chunk.name
                                            && e.entity_type == EntityType::QueenAnt
                                        {
                                            has_queen = true;
                                        }
                                    }
                                    if !has_queen {
                                        let id = rng.gen_range(0..999999);
                                        chunk_entities.insert(
                                            id,
                                            Entity {
                                                id: id,
                                                target_id: 0,
                                                hp: 100,
                                                x: (_rx + rng.gen_range(4.0..8.0))
                                                    * tile_size as f32,
                                                y: (_ry + rng.gen_range(4.0..8.0))
                                                    * tile_size as f32,
                                                stopped: false,
                                                speed: 0.0,
                                                dir: 0.0,
                                                target_x: 0.0,
                                                target_y: 0.0,
                                                entity_type: EntityType::FoodStorage,
                                                task_type: TaskType::Nothing,
                                                category_type: CategoryType::Furniture,
                                                religion_type: ReligionType::Nothing,
                                                faction: chunk.name.clone().to_string(),
                                                faction_id: chunk.id,
                                                current_action: ActionType::Idle,
                                                wielding_item: ItemType::Nothing,
                                                backpack_item: ItemType::Nothing,
                                                wearable_item: ItemType::Nothing,
                                                backpack_amount: 0,
                                                time: 0,
                                            },
                                        );
                                        let id = rng.gen_range(0..999999);
                                        chunk_entities.insert(
                                            id,
                                            Entity {
                                                id: id,
                                                target_id: 0,
                                                hp: 100,
                                                x: (_rx + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                y: (_ry + rng.gen_range(1.0..4.0))
                                                    * tile_size as f32,
                                                stopped: false,
                                                speed: 0.5,
                                                dir: 0.0,
                                                target_x: 0.0,
                                                target_y: 0.0,
                                                entity_type: EntityType::QueenAnt,
                                                category_type: CategoryType::Ant,
                                                religion_type: chunk.religion.clone(),
                                                faction: chunk.name.clone().to_string(),
                                                faction_id: chunk.id,
                                                task_type: TaskType::Nothing,
                                                current_action: ActionType::Idle,
                                                wielding_item: ItemType::Nothing,
                                                backpack_item: ItemType::Nothing,
                                                wearable_item: ItemType::Nothing,
                                                backpack_amount: 0,
                                                time: 0,
                                            },
                                        );
                                    }

                                    let sp_1 = k;
                                    let sp_2 = h;
                                    let ep_1 = k + 4;
                                    let ep_2 = h + 4;

                                    // floor
                                    for x in sp_1..ep_1 {
                                        for y in sp_2..ep_2 {
                                            if x < chunk_size && y < chunk_size {
                                                chunk.points[x][y].tile_type =
                                                    TileType::MudHiveFloor;
                                            }
                                        }
                                    }
                                    /*let side = rng.gen_range(0..4);
                                    let mut door_x = 0;
                                    let mut door_y = 0;
                                    match side {
                                    0 => {
                                        door_x = rng.gen_range(sp_1+1..ep_1-1);

                                        door_y = sp_2;

                                        },
                                    1 => {
                                        door_y = rng.gen_range(sp_2+1..ep_2-1);

                                        door_x = sp_1;

                                        },
                                    2 => {
                                        door_x = rng.gen_range(sp_1+1..ep_1-1);

                                        door_y = sp_2;

                                        },
                                    3 => {
                                        door_y = rng.gen_range(sp_2+1..ep_2-1);

                                        door_x = ep_1;

                                        },
                                    _ => {}
                                    }*/
                                    let door_x = 999;
                                    let door_y = 999;
                                    // wall
                                    for x in sp_1..ep_1 {
                                        for y in sp_2..ep_2 {
                                            if x < chunk_size - 1
                                                && y < chunk_size - 1
                                                && x > 0
                                                && y > 0
                                                && !(x == door_x && y == door_y)
                                            {
                                                if (chunk.points[x - 1][y].tile_type
                                                    != TileType::MudHiveFloor
                                                    && chunk.points[x - 1][y].tile_type
                                                        != TileType::MudHiveWall)
                                                    || (chunk.points[x + 1][y].tile_type
                                                        != TileType::MudHiveFloor
                                                        && chunk.points[x + 1][y].tile_type
                                                            != TileType::MudHiveFloor)
                                                    || (chunk.points[x][y - 1].tile_type
                                                        != TileType::MudHiveFloor
                                                        && chunk.points[x][y - 1].tile_type
                                                            != TileType::MudHiveWall)
                                                    || (chunk.points[x][y + 1].tile_type
                                                        != TileType::MudHiveFloor
                                                        && chunk.points[x][y + 1].tile_type
                                                            != TileType::MudHiveWall)
                                                {
                                                    chunk.points[x][y].tile_type =
                                                        TileType::MudHiveWall;
                                                }
                                            } else if (x == chunk_size - 1 || x == 0)
                                                && y < chunk_size
                                            {
                                                chunk.points[x][y].tile_type =
                                                    TileType::MudHiveWall;
                                            } else if (y == chunk_size - 1 || y == 0)
                                                && x < chunk_size
                                            {
                                                chunk.points[x][y].tile_type =
                                                    TileType::MudHiveWall;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    world_chunks[i][j].entities.extend(chunk_entities);
                    // world_chunks[i][j].entities.extend(&mut chunk_entities);
                }
            }
        }
        if apply_cities {
            for i in 0..width {
                for j in 0..height {
                    for k in 0..chunk_size {
                        for h in 0..chunk_size {
                            let _rx = ((i * chunk_size) as usize + k) as f32;
                            let _ry = ((j * chunk_size) as usize + h) as f32;

                            let point = &world_chunks[i as usize][j as usize].points[k][h];

                            let city_val =
                                city_noise[(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                            let city_building_val = city_building_noise
                                [(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                            if city_val > city_threshold {
                                if city_building_val > city_threshold
                                    && point.tile_type != TileType::Water
                                    && point.tile_type != TileType::Ice
                                    && point.tile_type != TileType::Sand
                                    && point.tile_type != TileType::RedSand
                                {
                                    world_chunks[i as usize][j as usize].points[k][h].tile_type =
                                        TileType::StoneHiveFloor;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // ENTITIES
    if apply_entities {
        // Vegetation
        if apply_vegetation {
            for i in 0..width {
                for j in 0..height {
                    let mut chunk_entities = HashMap::new();
                    for k in 0..chunk_size {
                        for h in 0..chunk_size {
                            let _rx = ((i * chunk_size) as usize + k) as f32;
                            let _ry = ((j * chunk_size) as usize + h) as f32;
                            let chunk = &mut world_chunks[i as usize][j as usize];
                            let point = &mut chunk.points[k][h];
                            let mut entity_type = EntityType::Cactus;
                            if point.tile_type == TileType::Sand {
                                entity_type = EntityType::Cactus;
                            }
                            let vegetation_val = vegetation_noise
                                [(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                            if vegetation_val > vegetation_threshold
                                && (point.tile_type == TileType::Sand)
                            {
                                let id = rng.gen_range(0..999999);
                                chunk_entities.insert(
                                    id,
                                    Entity {
                                        id: id,
                                        target_id: 0,
                                        x: _rx * tile_size as f32,
                                        y: _ry * tile_size as f32,
                                        hp: 100,
                                        dir: 0.0,
                                        target_x: 0.0,
                                        target_y: 0.0,
                                        speed: 0.0,
                                        stopped: true,
                                        entity_type: entity_type,
                                        category_type: CategoryType::Vegetation,
                                        faction: chunk.name.clone().to_string(),
                                        religion_type: ReligionType::Nothing,
                                        faction_id: chunk.id,
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
                    world_chunks[i][j].entities.extend(chunk_entities);
                }
            }
        }
        // Trees
        if apply_trees {
            for i in 0..width {
                for j in 0..height {
                    let mut chunk_entities = HashMap::new();
                    for k in 0..chunk_size {
                        for h in 0..chunk_size {
                            let _rx = ((i * chunk_size) as usize + k) as f32;
                            let _ry = ((j * chunk_size) as usize + h) as f32;
                            let chunk = &mut world_chunks[i as usize][j as usize];
                            let point = &mut chunk.points[k][h];
                            let mut entity_type = EntityType::Oak;
                            let tree_rand = rng.gen_range(0..2);
                            if point.tile_type == TileType::Grass {
                                if tree_rand == 0 {
                                    entity_type = EntityType::Oak;
                                    if rng.gen_range(0..6) == 1 {
                                        entity_type = EntityType::AppleTree;
                                    }
                                } else if tree_rand == 1 {
                                    entity_type = EntityType::Birch;
                                }
                            } else if point.tile_type == TileType::ColdLand {
                                if tree_rand == 0 {
                                    entity_type = EntityType::Spruce;
                                } else if tree_rand == 1 {
                                    entity_type = EntityType::Pine;
                                }
                            }
                            let tree_val =
                                tree_noise[(_ry + _rx * width as f32 * chunk_size as f32) as usize];
                            if tree_val > tree_threshold
                                && (point.tile_type == TileType::Grass
                                    || point.tile_type == TileType::ColdLand)
                            {
                                let id = rng.gen_range(0..999999);
                                chunk_entities.insert(
                                    id,
                                    Entity {
                                        id: id,
                                        target_id: 0,
                                        x: _rx * tile_size as f32,
                                        y: _ry * tile_size as f32,
                                        hp: 100,
                                        speed: 0.0,
                                        dir: 0.0,
                                        target_x: 0.0,
                                        target_y: 0.0,
                                        stopped: true,
                                        entity_type: entity_type,
                                        category_type: CategoryType::Tree,
                                        religion_type: ReligionType::Nothing,
                                        faction: chunk.name.clone().to_string(),
                                        faction_id: chunk.id,
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
                    /*chunk_entities
                    .into_iter()
                    .map(|(k.try_into.unwrap(), v)| world_chunks[i][j].entities.insert(k, v));*/
                    world_chunks[i][j].entities.extend(chunk_entities);
                }
            }
        }
    }
    if apply_objectives {
        let random_x = rng.gen_range(0..world_chunks.len());
        let random_y = rng.gen_range(0..world_chunks[0].len());
        let chunk = &mut world_chunks[random_x][random_y];
        let random_coord_x = rng.gen_range(0..chunk.points.len()) as f32;
        let random_coord_y = rng.gen_range(0..chunk.points[0].len()) as f32;
        let id = rng.gen_range(0..999999);
        chunk.entities.insert(
            id,
            Entity {
                id: id,
                target_id: 0,
                x: (chunk.points[0][0].x + random_coord_x) * tile_size as f32,
                y: (chunk.points[0][0].y + random_coord_y) * tile_size as f32,
                hp: 100,
                speed: 0.0,
                dir: 0.0,
                target_x: 0.0,
                target_y: 0.0,
                stopped: true,
                entity_type: EntityType::HolyMonument,
                category_type: CategoryType::Tree,
                religion_type: ReligionType::Nothing,
                faction: chunk.name.clone().to_string(),
                faction_id: chunk.id,
                current_action: ActionType::Idle,
                task_type: TaskType::Nothing,
                backpack_item: ItemType::Nothing,
                wearable_item: ItemType::Nothing,
                wielding_item: ItemType::Nothing,
                backpack_amount: 0,
                time: 0,
            },
        );
        for i in 0..3 {
            let random_x = rng.gen_range(0..world_chunks.len());
            let random_y = rng.gen_range(0..world_chunks[0].len());
            let chunk = &mut world_chunks[random_x][random_y];
            let random_coord_x = rng.gen_range(0..chunk.points.len()) as f32;
            let random_coord_y = rng.gen_range(0..chunk.points[0].len()) as f32;
            let id = rng.gen_range(0..999999);
            chunk.entities.insert(
                id,
                Entity {
                    id: id,
                    target_id: 0,
                    x: (chunk.points[0][0].x + random_coord_x) * tile_size as f32,
                    y: (chunk.points[0][0].y + random_coord_y) * tile_size as f32,
                    hp: 100,
                    speed: 0.0,
                    dir: 0.0,
                    target_x: 0.0,
                    target_y: 0.0,
                    stopped: true,
                    entity_type: EntityType::HolyObject,
                    category_type: CategoryType::Tree,
                    religion_type: ReligionType::Nothing,
                    faction: chunk.name.clone().to_string(),
                    faction_id: chunk.id,
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
        for i in 0..16 {
            let random_x = rng.gen_range(0..world_chunks.len());
            let random_y = rng.gen_range(0..world_chunks[0].len());
            let mut chunk = &mut world_chunks[random_x][random_y];
            let random_coord_x = rng.gen_range(0..chunk.points.len()) as f32;
            let random_coord_y = rng.gen_range(0..chunk.points[0].len()) as f32;
            let id = rng.gen_range(0..999999);
            chunk.entities.insert(
                id,
                Entity {
                    id: id,
                    target_id: 0,
                    x: (chunk.points[0][0].x + random_coord_x) * tile_size as f32,
                    y: (chunk.points[0][0].y + random_coord_y) * tile_size as f32,
                    hp: 100,
                    speed: 1.0,
                    dir: 0.0,
                    target_x: 0.0,
                    target_y: 0.0,
                    stopped: false,
                    entity_type: EntityType::FungusMonster,
                    category_type: CategoryType::Monster,
                    religion_type: ReligionType::Nothing,
                    faction: "Evil".to_string(),
                    faction_id: chunk.id,
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
    // relations
    let mut factions = HashMap::new();
    for row in world_chunks.iter() {
        for chunk in row.iter() {
            if !factions.contains_key(&chunk.name) {
                let relations = HashMap::new();
                factions.insert(
                    chunk.name.clone(),
                    Faction {
                        name: chunk.name.clone(),
                        relations,
                    },
                );
            }
        }
    }
    return World {
        chunks: world_chunks,
        world_data: WorldData {
            name: name,
            sea_level: sea_level,
            width: width,
            height: height,
            chunk_size: chunk_size,
            tile_size: tile_size,
            is_default: false,
        },

        players: Vec::new(),
        colliders: Vec::new(),
        props: Vec::new(),
        factions: factions,
        v_x: 0,
        v_y: 0,
        v_w: 3,
        v_h: 2,
    };
}
fn get_chunk_name() -> String {
    let mut rng = rand::thread_rng();
    let filename = "words/words.txt";
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let content_vec: Vec<&str> = contents.split("\n").collect();
    let mut word: String = content_vec[rng.gen_range(0..content_vec.len() - 1)]
        .chars()
        .rev()
        .collect::<String>();
    word.remove(word.len() - 1);
    let letters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut char_1 = letters[rng.gen_range(0..letters.len() - 1)];
    if letters.len() < 2 {
        char_1 = 'a';
    }
    word.push(char_1);
    if word.len() - 1 != 0 {
        word.remove(rng.gen_range(0..word.len() - 1));
    } else {
        word.remove(0);
    }
    word = word.to_lowercase();
    let first_letter = word.chars().nth(0).unwrap();
    word.replace_range(
        0..1,
        &first_letter.to_uppercase().nth(0).unwrap().to_string(),
    );
    return word.to_string();
}
