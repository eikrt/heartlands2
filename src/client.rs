use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Point,Rect};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseWheelDirection;
use sdl2::mouse::{MouseState};
use sdl2::render::{WindowCanvas, Texture, TextureCreator, BlendMode};
use sdl2::image::{LoadSurface,LoadTexture, InitFlag};
use sdl2::surface::{Surface};
use sdl2::ttf::Font;
use std::collections::HashMap;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::option::{Option};
use std::iter::FromIterator;
use std::str::from_utf8;
use std::{thread, time};
use std::sync::mpsc;
use std::time::{SystemTime};
use crate::world_structs;
use crate::graphics_utils;
use serde_json;
use lerp::Lerp;
use rand::Rng;
const SCREEN_WIDTH: u32 = 720;
const SCREEN_HEIGHT: u32 = 480;
const TILE_SIZE: f32 = 16.0;

fn main_loop() -> Result<(), String> {

    // sdl stuff
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem.window("Mechants", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let icon: Surface = LoadSurface::from_file("res/icon2.png").unwrap();
    window.set_icon(icon);
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
    let mut world_data_state: Option<world_structs::WorldData> = None;
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
    let mut chunks_state: Vec<world_structs::Chunk> = Vec::new();
    let mut entities_state: Vec<world_structs::Entity> = Vec::new();
    // menu buttons
    let mut play_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::HOVERED,
        previous_status: graphics_utils::ButtonStatus::HOVERED,
        x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
        y: 64.0 + 32.0,
        width: 128.0,
        height: 32.0
    };
    let mut settings_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::HOVERED,
        previous_status: graphics_utils::ButtonStatus::HOVERED,
        x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
        y: 128.0 + 32.0,
        width: 128.0,
        height: 32.0
    };
    let mut exit_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::HOVERED,
        previous_status: graphics_utils::ButtonStatus::HOVERED,
        x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
        y: 192.0 + 32.0,
        width: 128.0,
        height: 32.0
    };
    // ui buttons 
    let mut normal_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::HOVERED,
        previous_status: graphics_utils::ButtonStatus::HOVERED,
        x: 4 as f32,
        y: (SCREEN_HEIGHT - 32 - 4) as f32,
        width: 32.0,
        height: 32.0
    };
    let mut political_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::HOVERED,
        previous_status: graphics_utils::ButtonStatus::HOVERED,
        x: 4.0 + 32.0 + 4.0,
        y: (SCREEN_HEIGHT - 32 - 4) as f32,
        width: 32.0,
        height: 32.0
    };
    // entity textures
    let oak_texture = texture_creator.load_texture("res/oak.png")?;
    let birch_texture = texture_creator.load_texture("res/birch.png")?;
    let appletree_texture = texture_creator.load_texture("res/appletree.png")?;
    let pine_texture = texture_creator.load_texture("res/pine.png")?;
    let spruce_texture = texture_creator.load_texture("res/spruce.png")?;
    let cactus_texture = texture_creator.load_texture("res/cactus.png")?;
    let ant_worker_texture = texture_creator.load_texture("res/ant1.png")?;
    let ant_soldier_texture = texture_creator.load_texture("res/ant1.png")?;
    let ant_drone_texture = texture_creator.load_texture("res/ant_drone.png")?;
    let mechant_texture = texture_creator.load_texture("res/mechant.png")?;
    let ant_queen_texture= texture_creator.load_texture("res/ant_queen.png")?;
    let snail_texture = texture_creator.load_texture("res/snail.png")?;
    let food_storage_texture= texture_creator.load_texture("res/food_storage.png")?;
    // tile textures
    let mut grass_texture = texture_creator.load_texture("res/grass.png")?;
    let mut water_texture = texture_creator.load_texture("res/water.png")?;
    let mut ice_texture = texture_creator.load_texture("res/ice.png")?;
    let mut sand_texture = texture_creator.load_texture("res/sand.png")?;
    // menu textures
    let mut menu_button_texture = texture_creator.load_texture("res/menu_button.png")?;
    let mut menu_button_hovered_texture = texture_creator.load_texture("res/menu_button_hovered.png")?;
    let mut menu_button_pressed_texture = texture_creator.load_texture("res/menu_button_pressed.png")?;
    let mut menu_background = texture_creator.load_texture("res/background_image_1.png")?;

    // ui textures

    let mut ui_button_texture = texture_creator.load_texture("res/ui_button.png")?;
    let mut ui_button_hovered_texture = texture_creator.load_texture("res/ui_button_hovered.png")?;
    let mut ui_button_pressed_texture = texture_creator.load_texture("res/ui_button_pressed.png")?;
    // other texture stuff
    
    // description stuff
    let descriptions_for_entities = graphics_utils::get_descriptions_for_entities();
    let descriptions_for_tiles = graphics_utils::get_descriptions_for_tiles();
    let sprite_16 = Rect::new(0,0,(16.0 * camera.zoom) as u32, (16.0 * camera.zoom) as u32);
    let sprite_32 = Rect::new(0,0,(32.0 * camera.zoom) as u32, (32.0 * camera.zoom) as u32);
    let sprite_128x32 = Rect::new(0,0,(128.0 * camera.zoom) as u32, (32.0 * camera.zoom) as u32);
    let sprite_720x480 = Rect::new(0,0,720.0 as u32, 480.0 as u32);

    // gameplay stuff
    
    let mut rng = rand::thread_rng();
    let mut map_state = graphics_utils::MapState::NORMAL;
    let mut main_menu_on = true;
    let mut settings_menu_on = false;
    let mut chunk_graphics_data: HashMap<String, Color> = HashMap::new();

    // network stuff

    let (sender, receiver): (mpsc::Sender<(Option<world_structs::WorldData>, Vec<world_structs::Chunk>, Vec<world_structs::Entity>)>, mpsc::Receiver<(Option<world_structs::WorldData>, Vec<world_structs::Chunk>, Vec<world_structs::Entity>)>) = mpsc::channel();
    thread::spawn(move || {
    let mut rng = rand::thread_rng();
        while true {
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
                     Err(e) => panic!("Failed to get world data"),
                });
            }
            else {
                response = Some(match serde_json::from_str(&res) {
                     Ok(v) => v,
                     Err(e) => world_structs::WorldResponse{
                         chunk: world_structs::Chunk{
                             points: vec![],
                             name: "error".to_string(),
                             id: rng.gen_range(0..999999),    
                         },
                        entities: vec![],
                        valid: false
                     },
                });
                if !response.as_ref().unwrap().valid {
                    continue;
                    println!("jumped");
                }
                /*let mut chunk_already_in_chunks = false;
                for chnk in &chunks {
                    match response {
                    Some(ref r) => {if chnk.points[0][0].x == r.chunk.points[0][0].x && chnk.points[0][0].y == r.chunk.points[0][0].y{
                       // chunk_already_in_chunks = true;
                    }


                        }
                    None => ()
                    };
                }*/
                    match response {
                    Some(ref r) => {
                        
                        let mut index_option = chunks.iter().position(|x| x.id == r.chunk.id);
                        if index_option != None {
                            let index = index_option.unwrap();
                            chunks.remove(index);
                        }
                        chunks.push(r.chunk.clone())
                        },
                    None => ()
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
                sender.send((world_data.clone(), chunks.clone(), entities.clone())).unwrap();
        }
    });
    while running  {
    let delta = SystemTime::now().duration_since(compare_time).unwrap();
    compare_time = SystemTime::now();
    let delta_as_millis = delta.as_millis();
        if delta.as_millis()/10 != 0 {
         //   println!("FPS: {}", 100 / (delta.as_millis()/10));
        }
        mouse_not_moved_for += delta_as_millis;
        canvas.set_draw_color(bg_color);
        canvas.clear();
        // canvas.fill_rect(Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT)); 
        // send message to server
        //
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
            
            Event::MouseWheel{x,y, ..} => {
                
                if y > 0 {   
                    zoom_button_minus = true;

                }
                else if y < 0 {   
                    zoom_button_plus = true;

                }
            }
            Event::MouseMotion{ ..} => {
                
                
                mouse_not_moved_for = 0; 
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

        mouse_state = event_pump.mouse_state();
        let mouse_x = (mouse_state.x() as f32 + camera.x) / camera.zoom;
        let mouse_y = (mouse_state.y() as f32 + camera.y) / camera.zoom;


        if main_menu_on {
            //render menu background
            graphics_utils::render(&mut canvas, &menu_background, Point::new(0,0), sprite_720x480, 1.0);
            // render buttons
            let position = Point::new(play_button.x as i32,play_button.y as i32);
            play_button.check_if_hovered(mouse_x, mouse_y);
            play_button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
            settings_button.check_if_hovered(mouse_x, mouse_y);
            settings_button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
            exit_button.check_if_hovered(mouse_x, mouse_y);
            exit_button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
            // play button 
            if play_button.status == graphics_utils::ButtonStatus::HOVERED {
                graphics_utils::render(&mut canvas, &menu_button_hovered_texture, position, sprite_128x32, 1.0);

            }

            else if play_button.status == graphics_utils::ButtonStatus::PRESSED {
                graphics_utils::render(&mut canvas, &menu_button_pressed_texture, position, sprite_128x32, 1.0);

            }
            else {

                graphics_utils::render(&mut canvas, &menu_button_texture, position, sprite_128x32, 1.0);
            }
            // settings button
            let position = Point::new(settings_button.x as i32,settings_button.y as i32);
            if settings_button.status == graphics_utils::ButtonStatus::HOVERED {
                graphics_utils::render(&mut canvas, &menu_button_hovered_texture, position, sprite_128x32, 1.0);

            }

            else if settings_button.status == graphics_utils::ButtonStatus::PRESSED {
                graphics_utils::render(&mut canvas, &menu_button_pressed_texture, position, sprite_128x32, 1.0);

            }
            else {

                graphics_utils::render(&mut canvas, &menu_button_texture, position, sprite_128x32, 1.0);
            }
            // exit button
            let position = Point::new(exit_button.x as i32,exit_button.y as i32);
            if exit_button.status == graphics_utils::ButtonStatus::HOVERED {
                graphics_utils::render(&mut canvas, &menu_button_hovered_texture, position, sprite_128x32, 1.0);

            }

            else if exit_button.status == graphics_utils::ButtonStatus::PRESSED {
                graphics_utils::render(&mut canvas, &menu_button_pressed_texture, position, sprite_128x32, 1.0);

            }
            else {

                graphics_utils::render(&mut canvas, &menu_button_texture, position, sprite_128x32, 1.0);
            }
            // render texts
            let title_text = graphics_utils::get_text("MECHANTS".to_string(), Color::RGBA(255, 255, 255, 255), desc_font_size, &font, &texture_creator).unwrap();
            let position = Point::new((SCREEN_WIDTH / 2 - 42) as i32, 32 as i32);
            graphics_utils::render_text(&mut canvas, &title_text.text_texture, position, title_text.text_sprite);
            let text_margin = 4;
            let play_text = graphics_utils::get_text("Play".to_string(), Color::RGBA(255, 255, 255, 255), desc_font_size, &font, &texture_creator).unwrap();
            let position = Point::new(play_button.x as i32 + text_margin, play_button.y as i32 + text_margin);
            graphics_utils::render_text(&mut canvas, &play_text.text_texture, position, play_text.text_sprite);
            let settings_text = graphics_utils::get_text("Settings".to_string(), Color::RGBA(255, 255, 255, 255), desc_font_size, &font, &texture_creator).unwrap();
            let position = Point::new(settings_button.x as i32+ text_margin, settings_button.y as i32+ text_margin,);
            graphics_utils::render_text(&mut canvas, &settings_text.text_texture, position, settings_text.text_sprite);
            let exit_text = graphics_utils::get_text("Exit".to_string(), Color::RGBA(255, 255, 255, 255), desc_font_size, &font, &texture_creator).unwrap();
            let position = Point::new(exit_button.x as i32+ text_margin, exit_button.y as i32+ text_margin,);
            graphics_utils::render_text(&mut canvas, &exit_text.text_texture, position, exit_text.text_sprite);

            if play_button.status == graphics_utils::ButtonStatus::RELEASED {
               main_menu_on = false;
            }

            else if settings_button.status == graphics_utils::ButtonStatus::RELEASED {
            }
            else if exit_button.status == graphics_utils::ButtonStatus::RELEASED {
               running = false;
            }

        }
        else {


        if w {
            camera.mov(graphics_utils::MoveDirection::UP, delta_as_millis);
        }
        if a {
            camera.mov(graphics_utils::MoveDirection::LEFT, delta_as_millis);
        }
        if s {
            camera.mov(graphics_utils::MoveDirection::DOWN, delta_as_millis);
        }
        if d {
            camera.mov(graphics_utils::MoveDirection::RIGHT, delta_as_millis);
        }
        if zoom_button_plus {
            camera.zoom(graphics_utils::MoveDirection::ZOOMIN, delta_as_millis);
            zoom_button_plus = false;
        }
        if zoom_button_minus {
            camera.zoom(graphics_utils::MoveDirection::ZOOMOUT, delta_as_millis);
            zoom_button_minus = false;
        }
         
        world_data_state = receiver.recv().unwrap().0;
        chunks_state = Vec::new();//receiver.recv().unwrap().1;
        entities_state = Vec::new();//receiver.recv().unwrap().2;
        println!("{}", delta_as_millis);
        // iterate chunks
        for chunk_in_chunks in chunks_state.iter() {
            if !chunk_graphics_data.contains_key(&chunk_in_chunks.name) {
            chunk_graphics_data.insert(
                chunk_in_chunks.name.clone(),
                Color::RGBA(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), 125)

            );
            }
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
                    graphics_utils::render_with_color(&mut canvas, texture, position, sprite_16, Color::RGBA(r_result,g_result,b_result, 125), camera.zoom);
                             match canvas.fill_rect(Rect::new(tx as i32,ty as i32,(TILE_SIZE * camera.zoom) as u32,(TILE_SIZE * camera.zoom) as u32)) {
                        Ok(_v) => (),
                        Err(_v) => (),
                        }
                    
                    }
                }}

        //render entities
        entities_state.sort_by(|a,b| a.id.cmp(&b.id));
        for entity in &entities_state {
            let tx_ant = (entity.x) * camera.zoom - camera.x;
            let ty_ant = (entity.y) * camera.zoom - camera.y;
            let tx_tree = (entity.x + TILE_SIZE/2.0) * camera.zoom - camera.x;
            let ty_tree = (entity.y - TILE_SIZE/4.0) * camera.zoom - camera.y;
            canvas.set_draw_color(Color::RGB(0,0,0));
            

            // trees
            if entity.entity_type == world_structs::EntityType::OAK {
                let position = Point::new(tx_tree as i32 - sprite_32.width() as i32 / 2,ty_tree as i32 - sprite_32.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &oak_texture, position, sprite_32, camera.zoom);

            } 

            else if entity.entity_type == world_structs::EntityType::APPLETREE {
                let position = Point::new(tx_tree as i32 - sprite_32.width() as i32 / 2,ty_tree as i32 - sprite_32.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &appletree_texture, position, sprite_32, camera.zoom);

            } 

            else if entity.entity_type == world_structs::EntityType::SPRUCE {
                let position = Point::new(tx_tree as i32 - sprite_32.width() as i32 / 2,ty_tree as i32 - sprite_32.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &spruce_texture, position, sprite_32, camera.zoom);

            }
            else if entity.entity_type == world_structs::EntityType::PINE {
                let position = Point::new(tx_tree as i32 - sprite_32.width() as i32 / 2,ty_tree as i32 - sprite_32.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &pine_texture, position, sprite_32, camera.zoom);

            }
            else if entity.entity_type == world_structs::EntityType::BIRCH {
                let position = Point::new(tx_tree as i32 - sprite_32.width() as i32 / 2,ty_tree as i32 - sprite_32.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &birch_texture, position, sprite_32, camera.zoom);

            }
            // vegetation

            else if entity.entity_type == world_structs::EntityType::CACTUS {
                let position = Point::new(tx_tree as i32 - sprite_32.width() as i32 / 2,ty_tree as i32 - sprite_32.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &cactus_texture, position, sprite_32, camera.zoom);

            }
            // ants and other lifeforms
            else if entity.entity_type == world_structs::EntityType::WORKER_ANT {
                let position = Point::new(tx_ant as i32 - sprite_16.width() as i32 / 2,ty_ant as i32 - sprite_16.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &ant_worker_texture, position, sprite_16, camera.zoom);

            }
            else if entity.entity_type == world_structs::EntityType::SOLDIER_ANT {
                let position = Point::new(tx_ant as i32 - sprite_16.width() as i32 / 2,ty_ant as i32 - sprite_16.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &ant_soldier_texture, position, sprite_16, camera.zoom);

            }
            else if entity.entity_type == world_structs::EntityType::DRONE_ANT {
                let position = Point::new(tx_ant as i32 - sprite_16.width() as i32 / 2,ty_ant as i32 - sprite_16.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &ant_drone_texture, position, sprite_16, camera.zoom);

            }
            else if entity.entity_type == world_structs::EntityType::MECHANT {
                let position = Point::new(tx_ant as i32 - sprite_16.width() as i32 / 2,ty_ant as i32 - sprite_16.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &mechant_texture, position, sprite_16, camera.zoom);

            }
            else if entity.entity_type == world_structs::EntityType::QUEEN_ANT {
                let position = Point::new(tx_ant as i32 - sprite_32.width() as i32 / 2,ty_ant as i32 - sprite_32.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &ant_queen_texture, position, sprite_32, camera.zoom);

            }
            else if entity.entity_type == world_structs::EntityType::FOOD_STORAGE {
                let position = Point::new(tx_ant as i32 - sprite_16.width() as i32 / 2,ty_ant as i32 - sprite_16.height() as i32 / 2);
                graphics_utils::render(&mut canvas, &food_storage_texture, position, sprite_16, camera.zoom);

            }
        }

            let mut hovered_tiletype = world_structs::TileType::GRASS;
            let mut hovered_tile: std::option::Option<world_structs::Point> = None;
            let mut hovered_entity: std::option::Option<world_structs::Entity> = None;
        let mut hovering_entity = false;
        if mouse_not_moved_for > hover_time {

            match world_data_state {
                Some(ref wd) => {

                    let e_x = mouse_x;
                    let e_y = mouse_y;
                    for e in &entities_state {
                        if e_x > e.x  && e_x < e.x + 16.0 && e_y > e.y && e_y < e.y+ 16.0{
                            hovering_entity = true;
                            hovered_entity = Some(e.clone());
                            ()
                        }
                    }
                    
                    let tile_x = (((mouse_x) / TILE_SIZE) as f32).floor();
                    let tile_y = (((mouse_y) / TILE_SIZE) as f32).floor();
                    for c in &chunks_state {
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
                        /*
                        match tile_descriptions.get(&ht.tile_type) {
                            Some(tt) => {

                                let position = Point::new((mouse_state.x() - tt.text_sprite.width() as i32 / 2),(mouse_state.y() - (tt.text_sprite.height()) as i32));
                                graphics_utils::render_text(&mut canvas, &tt.text_texture, position, tt.text_sprite);


                            },

                            None => ()
                    }*/

                        let text = graphics_utils::get_text(descriptions_for_tiles.get(&ht.tile_type).unwrap().to_string(), Color::RGBA(55, 185, 90, 255), desc_font_size, &font, &texture_creator).unwrap();
                        let position = Point::new((mouse_state.x() - text.text_sprite.width() as i32 / 2),(mouse_state.y() - (text.text_sprite.height()) as i32));
                        graphics_utils::render_text(&mut canvas, &text.text_texture, position, text.text_sprite);
                    },
                    None => ()
                }
            }
            else {
                match hovered_entity {
                   /* Some(he) => {

                        match entity_descriptions.get(&he.entity_type) {
                        Some(tt) => {
                            let position = Point::new((mouse_state.x() - tt.text_sprite.width() as i32 / 2),(mouse_state.y() - (tt.text_sprite.height()) as i32));
                            graphics_utils::render_text(&mut canvas, &tt.text_texture, position, tt.text_sprite);

                        }
                        None => ()
                    }
                },*/
                    Some(he) => {
                        let mut name = descriptions_for_entities.get(&he.entity_type).unwrap();
                        let mut title = "".to_string();
                        if he.entity_type == world_structs::EntityType::WORKER_ANT || he.entity_type == world_structs::EntityType::DRONE_ANT ||he.entity_type == world_structs::EntityType::SOLDIER_ANT || he.entity_type == world_structs::EntityType::QUEEN_ANT || he.entity_type == world_structs::EntityType::MECHANT  {
                            title = he.faction;
                            title.push_str("ese ");
                        }

                        title.push_str(name);
                        let text = graphics_utils::get_text(title,Color::RGBA(55, 185, 90, 255), desc_font_size, &font, &texture_creator).unwrap();

                        let position = Point::new((mouse_state.x() - text.text_sprite.width() as i32 / 2),(mouse_state.y() - (text.text_sprite.height()) as i32));
                        graphics_utils::render_text(&mut canvas, &text.text_texture, position, text.text_sprite);
                    }

                None => ()
            }
            }

        // render overlays

        if map_state == graphics_utils::MapState::POLITICAL {
            for c in &chunks_state {
                match world_data_state {

                    Some(ref wd) => {
                    
                        let position = Point::new((wd.tile_size as f32* c.points[0][0].x * camera.zoom - camera.x)  as i32, (wd.tile_size as f32 * c.points[0][0].y* camera.zoom - camera.y) as i32);
                        let render_rect = Rect::new(position.x, position.y, (wd.chunk_size as i32 * wd.tile_size) as u32, (wd.chunk_size as i32 * wd.tile_size) as u32);
                        match chunk_graphics_data.get(&c.name) {
                        Some(cgd) => {
                            if c.name == "Neutral" {
                                graphics_utils::render_rect(&mut canvas, position, render_rect, Color::RGBA(255,255,255,125), camera.zoom);
                            }
                            else {
                                graphics_utils::render_rect(&mut canvas, position, render_rect, *chunk_graphics_data.get(&c.name).unwrap(), camera.zoom);

                            }
                        },
                        None => {
                            graphics_utils::render_rect(&mut canvas, position, render_rect, Color::RGBA(255,255,255,125), camera.zoom);

                            }
                        }
                        // render chunk faction description
                        let title = c.name.clone();
                        let text = graphics_utils::get_text(title.clone(),Color::RGBA(55, 185, 90, 255), desc_font_size, &font, &texture_creator).unwrap();

                        let text_position = Point::new(position.x() + (wd.chunk_size as f32 * wd.tile_size as f32 * camera.zoom) as i32 / 2 - title.clone().len() as i32*desc_font_size as i32 / 4, position.y() + (wd.chunk_size as f32 * wd.tile_size as f32 * camera.zoom) as i32 / 2);
                        graphics_utils::render_text(&mut canvas, &text.text_texture, text_position, text.text_sprite);

                },
                    None => ()
            }
            }
        }

        // render ui
        // political map button 
        let position = Point::new(political_button.x as i32,political_button.y as i32);
        political_button.check_if_hovered(mouse_state.x() as f32, mouse_state.y() as f32);
        political_button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
        if political_button.status == graphics_utils::ButtonStatus::HOVERED {
            graphics_utils::render(&mut canvas, &ui_button_hovered_texture, position, sprite_32, 1.0);

        }

        else if political_button.status == graphics_utils::ButtonStatus::PRESSED {
            graphics_utils::render(&mut canvas, &ui_button_pressed_texture, position, sprite_32, 1.0);

        }
        else {

            graphics_utils::render(&mut canvas, &ui_button_texture, position, sprite_32, 1.0);
        }

        // normal map button 
        let position = Point::new(normal_button.x as i32,normal_button.y as i32);
        normal_button.check_if_hovered(mouse_state.x() as f32, mouse_state.y() as f32);
        normal_button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
        if normal_button.status == graphics_utils::ButtonStatus::HOVERED {
            graphics_utils::render(&mut canvas, &ui_button_hovered_texture, position, sprite_32, 1.0);

        }

        else if normal_button.status == graphics_utils::ButtonStatus::PRESSED {
            graphics_utils::render(&mut canvas, &ui_button_pressed_texture, position, sprite_32, 1.0);

        }
        else {

            graphics_utils::render(&mut canvas, &ui_button_texture, position, sprite_32, 1.0);
        }
        let normal_text_margin = 4;
        let normal_text = graphics_utils::get_text("N".to_string(), Color::RGBA(255, 255, 255, 255), desc_font_size, &font, &texture_creator).unwrap();
        let position = Point::new(normal_button.x as i32 + 8 + normal_text_margin, normal_button.y as i32 + normal_text_margin);
        graphics_utils::render_text(&mut canvas, &normal_text.text_texture, position, normal_text.text_sprite);

        let political_text_margin = 4;
        let political_text = graphics_utils::get_text("P".to_string(), Color::RGBA(255, 255, 255, 255), desc_font_size, &font, &texture_creator).unwrap();
        let position = Point::new(political_button.x as i32 + 8 + political_text_margin, political_button.y as i32 + political_text_margin);
        graphics_utils::render_text(&mut canvas, &political_text.text_texture, position, political_text.text_sprite);
       }
       
        if normal_button.status == graphics_utils::ButtonStatus::RELEASED {
            map_state = graphics_utils::MapState::NORMAL;
        }
        else if political_button.status == graphics_utils::ButtonStatus::RELEASED {
            map_state = graphics_utils::MapState::POLITICAL;
        }
        canvas.present();
        thread::sleep(time::Duration::from_millis(10));

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
