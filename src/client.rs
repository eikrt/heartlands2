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
use serde_json;
use lerp::Lerp;
use std::collections::HashMap;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const TILE_SIZE: f32 = 32.0;

struct TileGraphics {
    sc: Color,
    tc: Color
}
struct Camera {
    x: f32,
    y: f32,
    zoom: f32,
    speed: f32,
}
fn main_loop() -> Result<(), String> {


    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Heartlands", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let mut camera = Camera{
        x: 0.0,
        y: 0.0,
        zoom: 1.0,
        speed: 5120.0
    };
    let bg_color = Color::RGB(0, 0, 0);
    let tile_gs = HashMap::from([
        ("grass".to_string(),
        TileGraphics {

           sc: Color::RGB(128,64,55),
           tc: Color::RGB(128,128,55)
        }),
    
        ("water".to_string(),
        TileGraphics {
           sc: Color::RGB(0,20,55),
           tc: Color::RGB(0,20,255)
        }),
    
        ("ice".to_string(),
        TileGraphics {
           sc: Color::RGB(128,64,55),
           tc: Color::RGB(128,128,55)
        }),
    
        ("permafrost".to_string(),
        TileGraphics {
           sc: Color::RGB(128,64,55),
           tc: Color::RGB(128,128,55)
        }),
    
        ("coarse_land".to_string(),
        TileGraphics {
           sc: Color::RGB(128,64,55),
           tc: Color::RGB(128,128,55)
        }),
        ("savannah_land".to_string(),
        TileGraphics {
           sc: Color::RGB(128,64,55),
           tc: Color::RGB(128,128,55)
        }),
    
        ("sand".to_string(),
        TileGraphics {
           sc: Color::RGB(128,64,55),
           tc: Color::RGB(128,128,55)
        }),
        ("red_sand".to_string(),
        TileGraphics {
           sc: Color::RGB(128,64,55),
           tc: Color::RGB(128,128,55)
        })]);
    let mut stream = TcpStream::connect("localhost:5000").unwrap();
    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;
    let mut running = true ; 
    let mut event_pump = sdl_context.event_pump()?;
    let mut compare_time = SystemTime::now();
    while running {
    let delta = SystemTime::now().duration_since(compare_time).unwrap();
    let delta_as_millis = delta.as_millis()/10;
        if delta.as_millis()/10 != 0 {
         //   println!("FPS: {}", 100 / (delta.as_millis()/10));
        }

        canvas.set_draw_color(bg_color);
        canvas.clear();
        let point = world_structs::Chunk_point {x: 0, y: 0};
        let msg = serde_json::to_string(&point).unwrap(); 
        let mut buf = [0; 20048];
        let _write = stream.write(msg.as_bytes());
        match stream.read(&mut buf) {
            Ok(_v) => _v,
            Err(_v) => 0
        };
        let res = match from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid sequence: {}", e),
        }.replace("\0", "").replace("\n", "").to_string();

        let chunk: world_structs::Chunk = match serde_json::from_str(&res) {
            Ok(v) => v,
            Err(e) => panic!("Invalid sequence: {}", e),
        };
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
        _ => {}
            }
        }
        if w {
                camera.y -= (camera.speed * delta_as_millis as f32) / 1000.0;
        }
        if a {

                camera.x -= (camera.speed  * delta_as_millis as f32) / 1000.0;
        }
        if s {

                camera.y += (camera.speed  * delta_as_millis as f32) / 1000.0;
        }
        if d {

                camera.x += (camera.speed  * delta_as_millis as f32) / 1000.0;
        }
        for i in 0..chunk.points.len() {
            for j in 0..chunk.points.len() {
                let p = &chunk.points[i][j];
                let r_result = (tile_gs.get(&p.tile_type).unwrap().sc.r as f32).lerp(tile_gs.get(&p.tile_type).unwrap().tc.r as f32, p.z/512.0) as u8;
                let g_result = (tile_gs.get(&p.tile_type).unwrap().sc.g as f32).lerp(tile_gs.get(&p.tile_type).unwrap().tc.g as f32, p.z/512.0) as u8;
                let b_result = (tile_gs.get(&p.tile_type).unwrap().sc.b as f32).lerp(tile_gs.get(&p.tile_type).unwrap().tc.b as f32, p.z/512.0) as u8;
                canvas.set_draw_color(Color::RGB(r_result,g_result,b_result));
                
                match canvas.fill_rect(Rect::new((p.x*TILE_SIZE-camera.x) as i32,(p.y*TILE_SIZE-camera.y) as i32,TILE_SIZE as u32,TILE_SIZE as u32)) {
                    Ok(_v) => (),
                    Err(_v) => (),
                }
                
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
