use simdnoise::*;
use crate::world_structs;
use rand::Rng;
pub fn generate(seed:i32,  width:usize, height:usize, chunk_size:usize, sea_level:f32, name: String) -> world_structs::World {
let tile_size = 32;
let temperature_margin = 5;
let biomes: Vec<world_structs::Biome> = vec![
    world_structs::Biome {
        name: "glacier".to_string(),
        temperature: 0,
        tile_type: "ice".to_string()
    },

    world_structs::Biome {
        name: "tundra".to_string(),
        temperature: 5,
        tile_type: "permafrost".to_string()
    },
    
    world_structs::Biome {
        name: "taiga".to_string(),
        temperature: 10,
        tile_type: "cold_land".to_string()
    },
    
    world_structs::Biome {
        name: "forest".to_string(),
        temperature: 15,
        tile_type: "cold_land".to_string()
    },
    
    world_structs::Biome {
        name: "grasslands".to_string(),
        temperature: 20,
        tile_type: "grass".to_string()
    },
    
    world_structs::Biome {
        name: "mediterraean".to_string(),
        temperature: 25,
        tile_type: "coarse_land".to_string()
    },
    
    world_structs::Biome {
        name: "savannah".to_string(),
        temperature: 30,
        tile_type: "savannah_land".to_string()
    },

    world_structs::Biome {
        name: "desert".to_string(),
        temperature: 35,
        tile_type: "sand".to_string()
    },
    world_structs::Biome {
        name: "red_desert".to_string(),
        temperature: 40,
        tile_type: "red_sand".to_string()
    },
    world_structs::Biome {
        name: "rainforest".to_string(),
        temperature: 45,
        tile_type: "grass".to_string()
    }
];
    let max_temp = 45;

    println!("Generating world...");
    let mut world_chunks: Vec<Vec<world_structs::Chunk>> = Vec::new();
    let mut world_entities: Vec<world_structs::Entity> = Vec::new();
    let mut rng = rand::thread_rng();
    let ground_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.15)
        .with_octaves(9.0 as u8)
        .with_gain(2.0)
        .with_seed(seed)
        .with_lacunarity(0.8)
        .generate_scaled(0.0,512.0);
    let sea_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(1000.15)
        .with_octaves(16.0 as u8)
        .with_gain(2.0)
        .with_seed(seed*2)
        .with_lacunarity(0.4)
        .generate_scaled(0.0,512.0);
    let biome_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.000003)
        .with_octaves(16)
        .with_gain(1.0)
        .with_seed(seed*3)
        .with_lacunarity(2.0)
        .generate_scaled(-0.5,0.5);
    let river_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.02)
        .with_octaves(9)
        .with_gain(1.2)
        .with_seed(seed*4)
        .with_lacunarity(1.3)
        .generate_scaled(0.0,1.0);
    let river_area_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(100.0)
        .with_octaves(9)
        .with_gain(1.2)
        .with_seed(seed*5)
        .with_lacunarity(0.2)
        .generate_scaled(0.0,1.0);
    let tree_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.1)
        .with_octaves(9)
        .with_gain(0.1)
        .with_seed(seed*6)
        .with_lacunarity(5.0)
        .generate_scaled(0.0,0.9);

    let village_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.01)
        .with_octaves(6)
        .with_gain(1.0)
        .with_seed(seed*7)
        .with_lacunarity(2.5)
        .generate_scaled(0.0,1.0);

    let city_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.1)
        .with_octaves(16)
        .with_gain(1.0)
        .with_seed(seed*8)
        .with_lacunarity(2.0)
        .generate_scaled(0.0,1.0);
    let village_building_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.1)
        .with_octaves(32)
        .with_gain(0.0)
        .with_seed(seed*9)
        .with_lacunarity(9.0)
        .generate_scaled(0.0,1.0);

    let city_building_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.1)
        .with_octaves(32)
        .with_gain(0.0)
        .with_seed(seed*10)
        .with_lacunarity(5.0)
        .generate_scaled(0.0,1.0);
    let tree_threshold = 0.4;
    let village_threshold = 0.8;
    let city_threshold = 0.7;

    let village_building_threshold = 0.1;
    let city_building_threshold = 0.1;
    let river_threshhold = 0.5;
    let apply_seas = true;
    let apply_ground = true;
    let apply_water = true;
    let apply_rivers = true;
    let apply_entities = true;
    let apply_settlements = true;
    let apply_villages = true;
    let apply_cities = false;
    let apply_trees= true;
    // biomes and adding tiles
    for i in 0..width {
        world_chunks.push(vec![]);
        for j in 0..height {
            let mut chunk_points: Vec<Vec<world_structs::Point>> = Vec::new();
            for k in 0..chunk_size {
                chunk_points.push(vec![]);
                for h in 0..chunk_size {
                    let rx = ((i*chunk_size) as usize + k) as f32;
                    let ry = ((j*chunk_size) as usize + h) as f32;
                    let rz = 0.0;
                    let mut biome_val = biome_noise[(ry + rx*width as f32 *chunk_size as f32) as usize]; 
                    
                    let dist_from_equator = ((ry - (height as f32 *chunk_size as f32 )/2.0).powf(2.0) as f32).sqrt();
                    let rel = 1.0 - (dist_from_equator /  ((height*chunk_size)/2) as f32);
                    biome_val += rel;
                    let temp =(biome_val * max_temp as f32) as i32;
                    let mut biome = &biomes[0];
                    for b in biomes.iter() { 
                        if temp > b.temperature - temperature_margin && temp < b.temperature + temperature_margin { 
                            biome = b;           
                            break;                     

                    }
                    }
                    let biome_tile_type = &biome.tile_type;
                    chunk_points[k].push(world_structs::Point {
                                            x: rx,
                                            y: ry,
                                            z: rz,
                                            tile_type: biome_tile_type.to_string()});
                }

            }
        
            world_chunks[i as usize].push(world_structs::Chunk {
                                            points: chunk_points 
                                          });

        }
    }

    // SEAS AND BIG SHAPES
    if apply_seas {
        for i in 0..width {
            for j in 0..height {
                for k in 0..chunk_size {
                    for h in 0..chunk_size {
                        let _rx = (i*chunk_size) + k;
                        let _ry = (j*chunk_size) + h;
                        let _rz = sea_noise[_ry + _rx*width*chunk_size];
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
                        let _rx = ((i*chunk_size) as usize + k) as f32;
                        let _ry = ((j*chunk_size) as usize + h) as f32;
                        let _rz = ground_noise[(_ry + _rx*width as f32 *chunk_size as f32) as usize]; 
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
                        let _rx = ((i*chunk_size) as usize + k) as f32;
                        let _ry = ((j*chunk_size) as usize + h) as f32;
                        let _rz = river_noise[(_ry + _rx*width as f32 *chunk_size as f32) as usize]; 
                            let chunk = &mut world_chunks[i as usize][j as usize];
                        let point = &mut chunk.points[k][h];
                        let ra_value = river_area_noise[(_ry + _rx*width as f32 *chunk_size as f32) as usize]; 
                        if ra_value > 0.5 && _rz > river_threshhold && point.z > sea_level {

                            point.z = _rz *512.0;
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
                        let _rx = ((i*chunk_size) as usize + k) as f32;
                        let _ry = ((j*chunk_size) as usize + h) as f32;
                            let chunk = &mut world_chunks[i as usize][j as usize];
                            let point = &mut chunk.points[k][h];

                        if point.z < sea_level {
                            point.z = (512.0 - point.z);
                            point.tile_type = "water".to_string();


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
                    for k in 0..chunk_size {
                        for h in 0..chunk_size {
                            let _rx = ((i*chunk_size) as usize + k) as f32;
                            let _ry = ((j*chunk_size) as usize + h) as f32;
                            let chunk = &mut world_chunks[i as usize][j as usize];
                            let point = &mut chunk.points[k][h];
                            
                            let village_val = village_noise[(_ry + _rx*width as f32 *chunk_size as f32) as usize];
                            let village_building_val = village_building_noise[(_ry + _rx*width as f32 *chunk_size as f32) as usize];
                            if village_val > village_threshold { 
                                if village_building_val > village_building_threshold && point.tile_type != "water" && point.tile_type != "ice" && point.tile_type != "sand" && point.tile_type != "red_sand"{
                                point.tile_type = "mud_hive_floor".to_string();
                                let mut sp_1 = k;
                                let mut sp_2 = h;
                                let mut ep_1 = k;
                                let mut ep_2 = h;
                                ep_1 = k+4;
                                ep_2 = h+4;

                                // floor
                                for x in sp_1..ep_1 {
                                    for y in sp_2..ep_2{
                                        if x < chunk_size && y < chunk_size {
                                            chunk.points[x][y].tile_type = "mud_hive_floor".to_string();

                                    }
                                }

                            }
                                let side = rng.gen_range(0..4);
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
                                }
                                // wall
                                for x in sp_1..ep_1 {
                                    for y in sp_2..ep_2{
                                        if x < chunk_size-1 && y < chunk_size-1 && x > 0 && y > 0 && !(x == door_x && y == door_y) {
                                            if (chunk.points[x-1][y].tile_type != "mud_hive_floor" && chunk.points[x-1][y].tile_type != "mud_hive_wall") || (chunk.points[x+1][y].tile_type != "mud_hive_floor" && chunk.points[x+1][y].tile_type != "mud_hive_wall") || (chunk.points[x][y-1].tile_type != "mud_hive_floor" && chunk.points[x][y-1].tile_type != "mud_hive_wall") || (chunk.points[x][y+1].tile_type != "mud_hive_floor" && chunk.points[x][y+1].tile_type != "mud_hive_wall")   {

                                            chunk.points[x][y].tile_type = "mud_hive_wall".to_string();

                                    }
                                
                                        } else if (x == chunk_size-1 || x == 0) && y < chunk_size {
                                                chunk.points[x][y].tile_type = "mud_hive_wall".to_string();
                                        }else if (y == chunk_size-1 || y == 0)  && x < chunk_size {
                                                chunk.points[x][y].tile_type = "mud_hive_wall".to_string();
                                        }


                            }
                

                    }
                }

            }
            }
        }
        }
        }}
        if apply_cities {

            for i in 0..width {
                for j in 0..height {
                    for k in 0..chunk_size {
                        for h in 0..chunk_size {
                            let _rx = ((i*chunk_size) as usize + k) as f32;
                            let _ry = ((j*chunk_size) as usize + h) as f32;

                            let point = &world_chunks[i as usize][j as usize].points[k][h];
                            
                            let city_val = city_noise[(_ry + _rx*width as f32 *chunk_size as f32) as usize];
                            let city_building_val = city_building_noise[(_ry + _rx*width as f32 *chunk_size as f32) as usize];
                            if city_val > city_threshold{
                                if city_building_val > city_threshold && point.tile_type != "water" && point.tile_type != "ice" && point.tile_type != "sand" && point.tile_type != "red_sand"{
                                world_chunks[i as usize][j as usize].points[k][h].tile_type = "stone_hive_floor".to_string();

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

        // TREES
        if apply_trees {
            
            for i in 0..width {
                for j in 0..height {
                    for k in 0..chunk_size {
                        for h in 0..chunk_size {
                            let _rx = ((i*chunk_size) as usize + k) as f32;
                            let _ry = ((j*chunk_size) as usize + h) as f32;
                            let chunk = &mut world_chunks[i as usize][j as usize];
                            let point = &mut chunk.points[k][h];
                            let mut entity_type = "oak";
                            let mut tree_rand = rng.gen_range(0..2);
                            if point.tile_type == "grass" {
                                if tree_rand == 0 {
                                entity_type = "oak";

                                }
                                else if tree_rand == 1 {
                                    entity_type = "birch";
                                }

                            }
                         else if point.tile_type == "cold_land" {
                                if tree_rand == 0 {
                                entity_type = "spruce";
                            

                            }
                                else if tree_rand == 1 {
                                    entity_type = "pine";
                                }
                            }
                            let tree_val = tree_noise[(_ry + _rx*width as f32 *chunk_size as f32) as usize];
                            if tree_val > tree_threshold && (point.tile_type == "grass" || point.tile_type == "cold_land"){

                            world_entities.push(world_structs::Entity {
                                
                                x: _rx * tile_size as f32,
                                y: _ry * tile_size as f32,
                                entity_type: entity_type.to_string()
                            });

                        }
                

                    }
                }

            }
            }
        }
    }

// SETTLEMENTS

    return world_structs::World {
        chunks: world_chunks,
        entities: world_entities,
        world_data: world_structs::WorldData {
            name: name,
            sea_level: sea_level, 
            width: width,
            height: height,
            chunk_size: chunk_size,
            tile_size: tile_size       
        }
    };

}
