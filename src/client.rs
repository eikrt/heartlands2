use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point,Rect};
use sdl2::mouse::{MouseState};
use sdl2::render::{WindowCanvas, Texture, TextureCreator, BlendMode};
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::surface::{Surface};
use sdl2::ttf::Font;
use std::collections::HashMap;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::option::{Option};
use std::iter::FromIterator;
use std::str::from_utf8;
use std::{thread, time};
use std::time::{SystemTime};
use crate::world_structs;
use crate::graphics_utils;
use serde_json;
use lerp::Lerp;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const TILE_SIZE: f32 = 16.0;

fn main_loop() -> Result<(), String> {

    // sdl stuff
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Heartlands", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    canvas.set_blend_mode(BlendMode::Blend);

    // texture stuff
    let texture_creator = canvas.texture_creator();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    // font stuff
    let desc_font_size = 20;
    let ttf_context  = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("fonts/PixelOperator.ttf", desc_font_size)?;

    let tile_gs = graphics_utils::tile_graphics();

    let mut camera = graphics_utils::Camera{
        x: 0.0,
        y: 0.0,
        zoom: 1.0,
        zoom_speed: 0.05,
        move_speed: 20.0
    };
    let bg_color = Color::RGB(0, 0, 0);
    let mut stream = TcpStream::connect("localhost:5000").unwrap();
    let mut running = true ; 
    // controls
    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;
    let mut zoom_button_plus = false;
    let mut zoom_button_minus = false;
    let mut event_pump = sdl_context.event_pump()?;
    let mut compare_time = SystemTime::now();
    let mut update_data = true;
    let mut world_data: Option<world_structs::WorldData> = None;
    // mouse
    let mut mouse_not_moved_for = 0;
    let mut mouse_state = MouseState::new(&event_pump);
    let hover_time = 25;
    // chunks and entities
    let mut chunk_fetch_width = 2;
    let mut chunk_fetch_height= 2;
    let mut chunk_fetch_x = -1;
    let mut chunk_fetch_y = -1;
    let mut chunks: Vec<world_structs::Chunk> = Vec::new();
    let mut entities: Vec<world_structs::Entity> = Vec::new();

    
    // entity textures
    let oak_texture = texture_creator.load_texture("res/oak.png")?;
    let birch_texture = texture_creator.load_texture("res/birch.png")?;
    let appletree_texture = texture_creator.load_texture("res/appletree.png")?;
    let pine_texture = texture_creator.load_texture("res/pine.png")?;
    let spruce_texture = texture_creator.load_texture("res/spruce.png")?;
    let cactus_texture = texture_creator.load_texture("res/cactus.png")?;
    let ant_worker_texture = texture_creator.load_texture("res/ant1.png")?;
    let snail_texture = texture_creator.load_texture("res/snail.png")?;
    // tile textures
    let mut grass_texture = texture_creator.load_texture("res/grass.png")?;
    let mut water_texture = texture_creator.load_texture("res/water.png")?;
    let mut ice_texture = texture_creator.load_texture("res/ice.png")?;
    let mut sand_texture = texture_creator.load_texture("res/sand.png")?;
    // other texture stuff
    
    // description stuff
        
    let entity_descriptions = HashMap::from([
        (world_structs::EntityType::OAK,
         graphics_utils::get_text("Oak".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::EntityType::BIRCH,
         graphics_utils::get_text("Birch".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::EntityType::APPLETREE,
         graphics_utils::get_text("Apple tree".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::EntityType::PINE,
         graphics_utils::get_text("Pine".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::EntityType::SPRUCE,
         graphics_utils::get_text("Spruce".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::EntityType::CACTUS,
         graphics_utils::get_text("Cactus".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::EntityType::WORKER_ANT,
         graphics_utils::get_text("Ant worker".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::EntityType::SNAIL,
         graphics_utils::get_text("Snail".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
    ]);

    let tile_descriptions = HashMap::from([
        (world_structs::TileType::GRASS,
         graphics_utils::get_text("Grass".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::COLD_LAND,
         graphics_utils::get_text("Grass".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::ICE,
         graphics_utils::get_text("Ice".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::WATER,
         graphics_utils::get_text("Water".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::COARSE_LAND,
         graphics_utils::get_text("Coarse grass".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::SAVANNAH_LAND,
         graphics_utils::get_text("Savannah grass".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::SAND,
         graphics_utils::get_text("Sand".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::RED_SAND,
         graphics_utils::get_text("Red sand".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::PERMAFROST,
         graphics_utils::get_text("Frozen ground".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),

        (world_structs::TileType::MUD_HIVE_WALL,
         graphics_utils::get_text("Mud wall".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
        (world_structs::TileType::MUD_HIVE_FLOOR,
         graphics_utils::get_text("Mud floor".to_string(), desc_font_size, &font, &texture_creator).unwrap()
         ),
    ]); 
    let sprite_16 = Rect::new(0,0,(16.0 * camera.zoom) as u32, (16.0 * camera.zoom) as u32);
    let sprite_32 = Rect::new(0,0,(32.0 * camera.zoom) as u32, (32.0 * camera.zoom) as u32);
    // gameplay stuff

    while running  {
    let delta = SystemTime::now().duration_since(compare_time).unwrap();
    let delta_as_millis = delta.as_millis()/10;
        if delta.as_millis()/10 != 0 {
         //   println!("FPS: {}", 100 / (delta.as_millis()/10));
        }
        mouse_not_moved_for += delta_as_millis;
        canvas.set_draw_color(bg_color);
        canvas.clear();

        // canvas.fill_rect(Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT)); 
        // send message to server
        let mut msg: Option<String> = None;
        if update_data {
            msg = Some(serde_json::to_string(&world_structs::WorldRequest {req_type: world_structs::RequestType::DATA, x: 0, y: 0}).unwrap());
        }

        else if !update_data {
            let mut chunk_x = 0;
            let mut chunk_y = 0;
            match world_data {

                Some(ref wd) => {chunk_x = (camera.x / TILE_SIZE/wd.chunk_size as f32) as i32;
                                chunk_y = (camera.y / TILE_SIZE/wd.chunk_size as f32) as i32},
                None => ()
            }
            chunk_x += chunk_fetch_x;
            chunk_y += chunk_fetch_y;
            chunk_fetch_x += 1;


            if chunk_fetch_x > chunk_fetch_width {
                
                chunk_fetch_x = -1;
                chunk_fetch_y += 1;
            }
            if chunk_fetch_y > chunk_fetch_height {
                chunk_fetch_x = -1;
                chunk_fetch_y = -1;
            }
            match world_data {


            
            Some(ref wd) => {
            if chunk_fetch_x > wd.width as i32 -1 {
                chunk_fetch_x = wd.width as i32 - 1;
            }

            if chunk_fetch_y > wd.height as i32 -1 {
                chunk_fetch_y = wd.height as i32 - 1;
            }
            if chunk_fetch_x < 0 as i32{
                chunk_fetch_x = -1;
            }

            if chunk_fetch_y < 0 as i32{
                chunk_fetch_y = -1;
            }
            }
            None => {}
            }
            if chunk_x < 0 {
                chunk_x = 0;
            }
            if chunk_y < 0 {
                chunk_y = 0;
            }
            match world_data {
                Some(ref d) => {
                if chunk_x > d.width as i32-1 {
                    chunk_x = d.width as i32 - 1;
                }
                

                if chunk_y > d.height as i32-1 {
                     chunk_y = d.height as i32- 1;
                }
            }
                
                None => ()
            }
            msg = Some(serde_json::to_string(&world_structs::WorldRequest {req_type: world_structs::RequestType::CHUNK, x: chunk_x, y: chunk_y}).unwrap());
        }
        match msg {
            Some(m) => stream.write(m.as_bytes()),
            None => stream.write("No request".as_bytes()) 
        };

        // receive data from server
        
        let mut buf = [0; 65536];
        match stream.read(&mut buf) {
            Ok(_v) => _v,
            Err(_v) => 0
        };
        let res = match from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid sequence: {}", e),
        }.replace("\0", "").replace("\n", "").to_string();
        let mut response: Option<world_structs::WorldResponse> =  None;
        if update_data {
                        
            world_data = Some(match serde_json::from_str(&res) {
                 Ok(v) => v,
                 Err(e) => panic!("Invalid sequence: {}", e),
            });
        }
        else {

            response = Some(match serde_json::from_str(&res) {
                 Ok(v) => v,
                 Err(e) => panic!("Invalid sequence: {}", e),
            });
            let mut chunk_already_in_chunks = false;
            for chnk in &chunks {
                match response {
                Some(ref r) => {if chnk.points[0][0].x == r.chunk.points[0][0].x && chnk.points[0][0].y == r.chunk.points[0][0].y{
                    chunk_already_in_chunks = true;
                }


                    }
                None => ()
                };
            }
            if !chunk_already_in_chunks {
                match response {
                Some(ref r) => chunks.push(r.chunk.clone()),
                None => ()
            };
        }
            //let mut filtered_entities = Vec::new();
            match response {
                Some(ref mut  r) => {
                    for re in r.entities.clone() {
                            if !entities.is_empty() {
                                let mut index_option = entities.iter().position(|x| x.id == re.id);
                                if index_option != None {
                                    let index = index_option.unwrap();
                                    entities.remove(index);
                                }

                        }
                    }
                    entities.append(&mut r.entities);
                },
                None => ()
            }
            //for e in filtered_entities {

           // }
            
        }
            update_data = false;
        for event in event_pump.poll_iter() {
         match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                running = false;
            }
    // WASD
            Event::KeyDown{keycode: Some(Keycode::W), ..} => {
                
                w = true;

                
            }
            Event::KeyDown{keycode: Some(Keycode::A), ..} => {
                
                
                
                a = true;
            }
            Event::KeyDown{keycode: Some(Keycode::S), ..} => {
                
                
                s = true;
            }
            Event::KeyDown{keycode: Some(Keycode::D), ..} => {
                
                
                d = true;
            }
            Event::KeyDown{keycode: Some(Keycode::Plus), ..} => {
                
                
                zoom_button_plus = true;
            }
            Event::KeyDown{keycode: Some(Keycode::Minus), ..} => {
                
                
                zoom_button_minus = true;
            }
            
            // WASD
            Event::KeyUp{keycode: Some(Keycode::W), ..} => {
                
                w = false;
                
            }
            Event::KeyUp{keycode: Some(Keycode::A), ..} => {
                
                a = false;
            }
            Event::KeyUp{keycode: Some(Keycode::S), ..} => {
                
                
                s = false;
            }
            Event::KeyUp{keycode: Some(Keycode::D), ..} => {
                
                
                d = false;
            }

            Event::KeyUp{keycode: Some(Keycode::Plus), ..} => {
                
                
                zoom_button_plus = false;
            }
            Event::KeyUp{keycode: Some(Keycode::Minus), ..} => {
                
                
                zoom_button_minus = false;
            }
        _ => {}
            }
        }
        if w {
            camera.mov(graphics_utils::MoveDirection::UP);
        }
        if a {
            camera.mov(graphics_utils::MoveDirection::LEFT);
        }
        if s {
            camera.mov(graphics_utils::MoveDirection::DOWN);
        }
        if d {
            camera.mov(graphics_utils::MoveDirection::RIGHT);
        }
        if zoom_button_plus {
            camera.zoom(graphics_utils::MoveDirection::ZOOMIN);
        }
        if zoom_button_minus {
            camera.zoom(graphics_utils::MoveDirection::ZOOMOUT);
        }
        
        let current_mouse_state = event_pump.mouse_state();
        if current_mouse_state != mouse_state {
            mouse_not_moved_for = 0; 
        }
        mouse_state = current_mouse_state;
        // iterate chunks
        for chunk_in_chunks in chunks.iter() {

            for i in 0..chunk_in_chunks.points.len() {
                for j in 0..chunk_in_chunks.points.len() {
                    let p = &chunk_in_chunks.points[i][j];
                    let tx = p.x*TILE_SIZE*camera.zoom-camera.x;
                    let ty = p.y*TILE_SIZE*camera.zoom-camera.y;
                    if tx < -64.0 || ty < -64.0 {
                        continue;
                    }
                    
                    if tx > SCREEN_WIDTH as f32 || ty > SCREEN_HEIGHT as f32 {

                        continue;
                    }
                    let light = 1.0;
                    let r_result = ((tile_gs.get(&p.tile_type).unwrap().sc.r as f32).lerp(tile_gs.get(&p.tile_type).unwrap().tc.r as f32, p.z/512.0) / light) as u8;
                    let g_result = ((tile_gs.get(&p.tile_type).unwrap().sc.g as f32).lerp(tile_gs.get(&p.tile_type).unwrap().tc.g as f32, p.z/512.0) / light) as u8;
                    let b_result = ((tile_gs.get(&p.tile_type).unwrap().sc.b as f32).lerp(tile_gs.get(&p.tile_type).unwrap().tc.b as f32, p.z/512.0) /light) as u8;
                    // canvas.set_draw_color(Color::RGB(r_result,g_result,b_result));
                    
                    let tx = (p.x) * TILE_SIZE * camera.zoom - camera.x;
                    let ty = (p.y) * TILE_SIZE * camera.zoom - camera.y;
                    let position = Point::new(tx as i32,ty as i32);
                    let mut texture = &grass_texture;
                    if p.tile_type == world_structs::TileType::GRASS {
                        texture = &grass_texture;
                    }
                    else if p.tile_type == world_structs::TileType::WATER {
                        texture = &water_texture;
                    }
                    else if p.tile_type == world_structs::TileType::ICE {
                        texture = &ice_texture;
                    }
                    else if p.tile_type == world_structs::TileType::SAND || p.tile_type == world_structs::TileType::RED_SAND {
                        texture = &sand_texture;
                    }
                    graphics_utils::render_with_color(&mut canvas, texture, position, sprite_16, Color::RGBA(r_result,g_result,b_result, 125));
                             match canvas.fill_rect(Rect::new(tx as i32,ty as i32,(TILE_SIZE * camera.zoom) as u32,(TILE_SIZE * camera.zoom) as u32)) {
                        Ok(_v) => (),
                        Err(_v) => (),
                        }
                    
                    }
                }}

        //render entities
        entities.sort_by(|a,b| a.id.cmp(&b.id));
        for entity in &entities {
            let tx_ant = (entity.x) * camera.zoom - camera.x;
            let ty_ant = (entity.y) * camera.zoom - camera.y;
            let tx_tree = (entity.x + TILE_SIZE/2.0) * camera.zoom - camera.x;
            let ty_tree = (entity.y - TILE_SIZE/4.0) * camera.zoom - camera.y;
            canvas.set_draw_color(Color::RGB(0,0,0));
            

            // trees
            if entity.entity_type == world_structs::EntityType::OAK {
                let position = Point::new(tx_tree as i32 as i32 ,ty_tree as i32 as i32);
                graphics_utils::render(&mut canvas, &oak_texture, position, sprite_32);

            } 

            else if entity.entity_type == world_structs::EntityType::SPRUCE {
                let position = Point::new(tx_tree as i32 as i32 ,ty_tree as i32 as i32);
                graphics_utils::render(&mut canvas, &spruce_texture, position, sprite_32);

            }
            else if entity.entity_type == world_structs::EntityType::PINE {
                let position = Point::new(tx_tree as i32 as i32 ,ty_tree as i32 as i32);
                graphics_utils::render(&mut canvas, &pine_texture, position, sprite_32);

            }
            else if entity.entity_type == world_structs::EntityType::BIRCH {
                let position = Point::new(tx_tree as i32 as i32 ,ty_tree as i32 as i32);
                graphics_utils::render(&mut canvas, &birch_texture, position, sprite_32);

            }
            // vegetation

            else if entity.entity_type == world_structs::EntityType::CACTUS {
                let position = Point::new(tx_tree as i32 as i32 ,ty_tree as i32 as i32);
                graphics_utils::render(&mut canvas, &cactus_texture, position, sprite_32);

            }
            // ants and other lifeforms
            else if entity.entity_type == world_structs::EntityType::WORKER_ANT {
                let position = Point::new(tx_ant as i32 as i32 ,ty_ant as i32 as i32);
                graphics_utils::render(&mut canvas, &ant_worker_texture, position, sprite_16);

            }
        }

            let mut hovered_tiletype = world_structs::TileType::GRASS;
            let mut hovered_tile: std::option::Option<world_structs::Point> = None;
            let mut hovered_entity: std::option::Option<world_structs::Entity> = None;
        let mut hovering_entity = false;
        if mouse_not_moved_for > hover_time {

            match world_data {
                Some(ref wd) => {

                    let e_x = (((mouse_state.x() as f32 + camera.x)) as f32);
                    let e_y = (((mouse_state.y() as f32 + camera.y)) as f32);
                    for e in &entities {
                        if e_x > e.x  && e_x < e.x + 16.0 && e_y > e.y && e_y < e.y+ 16.0{
                            hovering_entity = true;
                            hovered_entity = Some(e.clone());
                            ()
                        }
                    }
                    
                    let tile_x = (((mouse_state.x() as f32 + camera.x) / TILE_SIZE) as f32).floor();
                    let tile_y = (((mouse_state.y() as f32 + camera.y) / TILE_SIZE) as f32).floor();
                    for c in &chunks {
                        for row in &c.points {
                            for p in row {
                            if tile_x == p.x && tile_y == p.y {
                                hovered_tile = Some(p.clone());
                                
                            } 
                        }

                    }
                    }
                    true

                    }
                None => false

                };
            }
            if (!hovering_entity) {
                match hovered_tile {
                    Some(ht) => {
                        match tile_descriptions.get(&ht.tile_type) {
                            Some(tt) => {

                                let position = Point::new((mouse_state.x() - tt.text_sprite.width() as i32 / 2),(mouse_state.y() - (tt.text_sprite.height()) as i32));
                                graphics_utils::render_text(&mut canvas, &tt.text_texture, position, tt.text_sprite);


                            },

                            None => ()
                    }
                    },
                    None => ()
                }
            }
            else {
                match hovered_entity {
                    Some(he) => {

                        match entity_descriptions.get(&he.entity_type) {
                        Some(tt) => {
                            let position = Point::new((mouse_state.x() - tt.text_sprite.width() as i32 / 2),(mouse_state.y() - (tt.text_sprite.height()) as i32));
                            graphics_utils::render_text(&mut canvas, &tt.text_texture, position, tt.text_sprite);

                        }
                        None => ()
                    }
                },

                None => ()
            }
            }
        canvas.present();
        compare_time = SystemTime::now();
        thread::sleep(time::Duration::from_millis(20));

        }


        println!("Socket connection ended.");
    Ok(())
}
pub fn run() {

    match main_loop() {
        Ok(_) => println!("Running..."),
        Err(_) => println!("Error:")
    }
}
