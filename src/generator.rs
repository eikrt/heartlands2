use simdnoise::*;
use crate::world_structs;



pub fn generate(seed:i32,  width:usize, height:usize, chunk_size:usize, sea_level:f32, name: String) -> world_structs::World {

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
        .with_freq(0.05)
        .with_octaves(3.0 as u8)
        .with_gain(2.0)
        .with_seed(seed)
        .with_lacunarity(16.0)
        .generate_scaled(0.0,512.0);
    let sea_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.05)
        .with_octaves(16.0 as u8)
        .with_gain(2.0)
        .with_seed(seed)
        .with_lacunarity(0.5)
        .generate_scaled(0.0,512.0);
    let biome_noise = NoiseBuilder::fbm_2d(chunk_size*width, chunk_size*height)
        .with_freq(0.05)
        .with_octaves(8)
        .with_gain(2.0)
        .with_seed(seed)
        .with_lacunarity(2.0)
        .generate_scaled(0.0,512.0);
    let apply_seas = true;
    let apply_ground = true;
    for i in 0..width {
        world_chunks.push(vec![]);
        for j in 0..height {
            let mut chunk_points: Vec<Vec<world_structs::Point>> = Vec::new();
            for k in 0..chunk_size {
                chunk_points.push(vec![]);
                for h in 0..chunk_size {
                    let rx = ((i*chunk_size) as usize + k) as f32;
                    let ry = ((j*chunk_size) as usize + h) as f32;
                    //let rz = biome_noise[(rx + j * chunk_size) as usize]; 
                    let rz = 0.0;
                    chunk_points[k].push(world_structs::Point {
                                            x: rx,
                                            y: ry,
                                            z: rz
                                        });
                }

            }
        
            world_chunks[i as usize].push(world_structs::Chunk {
                                            points: chunk_points 
                                          });

        }
    }
    if apply_seas {
        for i in 0..width {
            for j in 0..height {
                for k in 0..chunk_size {
                    for h in 0..chunk_size {
                        let rx = ((i*chunk_size) as usize + k) as f32;
                        let ry = ((j*chunk_size) as usize + h) as f32;
                        let rz = sea_noise[(rx as usize  + j as usize * chunk_size) as usize]; 
                        world_chunks[i as usize][j as usize].points[k][h].z = rz;

                }
            

            }
        }
    }
    }
    if apply_ground { 
        for i in 0..width {
            for j in 0..height {
                for k in 0..chunk_size {
                    for h in 0..chunk_size {
                        let rx = ((i*chunk_size) as usize + k) as f32;
                        let ry = ((j*chunk_size) as usize + h) as f32;
                        let rz = sea_noise[(rx as usize + j * chunk_size) as usize]; 
                        world_chunks[i as usize][j as usize].points[k][h].z += rz;

                }
            

            }
        }
    }
    }
    return world_structs::World {
        chunk_size: chunk_size,
        chunks: world_chunks,
        name: name,
        sea_level: sea_level 
    };

}
