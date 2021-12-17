use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::world_structs;
use serde::{Serialize, Deserialize};
use serde_json;
use lerp::Lerp;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const TILE_SIZE: f32 = 32.0;
#[derive(Serialize, Deserialize, Debug)]
struct Camera {
    x: f32,
    y: f32,
    z: f32
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
    let bg_color = Color::RGB(0, 0, 0);
    let tile_color1 = Color::RGB(128,64,55);
    let tile_color2 = Color::RGB(128,128,55);
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
        if delta.as_millis()/10 != 0 {
         //   println!("FPS: {}", 100 / (delta.as_millis()/10));
        }
        canvas.set_draw_color(bg_color);
        canvas.clear();
        let point = world_structs::Chunk_point {x: 0, y: 0};
        let msg = serde_json::to_string(&point).unwrap(); 
        let mut buf = [0; 20048];
        let write = stream.write(msg.as_bytes());
        stream.read(&mut buf);
        let res = match from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid sequence: {}", e),
        }.replace("\0", "").replace("\n", "").to_string();
        res.trim();


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

        for i in 0..chunk.points.len() {
            for j in 0..chunk.points.len() {
                let p = &chunk.points[i][j];
                let r_result = ((tile_color1.r as f32).lerp((tile_color2.r as f32), p.z/512.0) as u8);
                let g_result = ((tile_color1.g as f32).lerp((tile_color2.g as f32), p.z/512.0) as u8);
                let b_result = ((tile_color1.b as f32).lerp((tile_color2.b as f32), p.z/512.0) as u8);
                println!("{}", p.z/512.0);
                canvas.set_draw_color(Color::RGB(r_result,g_result,b_result));
                canvas.fill_rect(Rect::new((p.x*TILE_SIZE) as i32,(p.y*TILE_SIZE) as i32,TILE_SIZE as u32,TILE_SIZE as u32));
                
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
   main_loop();
}
