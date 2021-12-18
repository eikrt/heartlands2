use simdnoise::*;
use crate::world_structs;

pub fn generate(seed:i32,  width:usize, height:usize, chunk_size:usize, sea_level:f32, name: String) -> world_structs::World {
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
        tile_type: "grass".to_string()
    },
    
    world_structs::Biome {
        name: "forest".to_string(),
        temperature: 15,
        tile_type: "grass".to_string()
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
    let apply_seas = true;
    let apply_ground = true;
    let apply_water = true;

    // BIOMES and adding tiles
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
                        //let _rz = sea_noise[h + k*width*chunk_size + i*chunk_size]; 
                        world_chunks[i as usize][j as usize].points[k][h].z = _rz;

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
                        world_chunks[i as usize][j as usize].points[k][h].z += _rz;

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

                        if world_chunks[i as usize][j as usize].points[k][h].z < sea_level {
                            world_chunks[i as usize][j as usize].points[k][h].z = 512.0 - world_chunks[i as usize][j as usize].points[k][h].z;
                            world_chunks[i as usize][j as usize].points[k][h].tile_type = "water".to_string();


                    }
            

                }
            }
        }
        }
    }
    return world_structs::World {
        chunks: world_chunks,
        world_data: world_structs::WorldData {
            name: name,
            sea_level: sea_level, 
            width: width,
            height: height,
            chunk_size: chunk_size

        }
    };

}
