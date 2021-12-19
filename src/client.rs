use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};
use std::time::{SystemTime};
use crate::world_structs;
use crate::graphics_utils;
use serde_json;
use lerp::Lerp;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const TILE_SIZE: f32 = 32.0;



fn main_loop() -> Result<(), String> {


    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Heartlands", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
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

    let mut chunk_fetch_width = 3;
    let mut chunk_fetch_height= 3;
    let mut chunk_fetch_x = -1;
    let mut chunk_fetch_y = -1;
    let mut chunks: Vec<world_structs::Chunk> = Vec::new();
    while running  {
    let delta = SystemTime::now().duration_since(compare_time).unwrap();
    let _delta_as_millis = delta.as_millis()/10;
        if delta.as_millis()/10 != 0 {
         //   println!("FPS: {}", 100 / (delta.as_millis()/10));
        }

        canvas.set_draw_color(bg_color);
        canvas.clear();

        // send message to server
        let mut msg: Option<String> = None;
        if update_data {
            msg = Some(serde_json::to_string(&world_structs::WorldRequest {req_type: "data".to_string(), x: 0, y: 0}).unwrap());
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
            msg = Some(serde_json::to_string(&world_structs::WorldRequest {req_type: "chunk".to_string(), x: chunk_x, y: chunk_y}).unwrap());
        }
        match msg {
            Some(m) => stream.write(m.as_bytes()),
            None => stream.write("No request".as_bytes()) 
        };

        // receive data from server
        
        let mut buf = [0; 40048];
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
            let mut already_in_chunks = false;
            for chnk in &chunks {
                match response {
                Some(ref r) => {if chnk.points[0][0].x == r.chunk.points[0][0].x && chnk.points[0][0].y == r.chunk.points[0][0].y{
                    already_in_chunks = true;
                }


                    }
                None => ()
                };
            }
            if !already_in_chunks {
                match response {
                Some(r) => chunks.push(r.chunk),
                None => ()
            };
        }
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
            camera.mov(0);
        }
        if a {
            camera.mov(1);
        }
        if s {
            camera.mov(2);
        }
        if d {
            camera.mov(3);
        }
        if zoom_button_plus {
            camera.zoom('+');
        }
        if zoom_button_minus {
            camera.zoom('-');
        }
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
                    canvas.set_draw_color(Color::RGB(r_result,g_result,b_result));
                    
                             match canvas.fill_rect(Rect::new(tx as i32,ty as i32,(TILE_SIZE * camera.zoom) as u32,(TILE_SIZE * camera.zoom) as u32)) {
                        Ok(_v) => (),
                        Err(_v) => (),
                        }
                    
                    }
                }}

            
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
