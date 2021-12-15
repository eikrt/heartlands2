extern crate kiss3d;
extern crate nalgebra as na;
use na::{Vector3, UnitQuaternion, Point2, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;

use kiss3d::camera::{ArcBall, FirstPerson};
use kiss3d::event::{Action, Key, WindowEvent};
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::world_structs;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32
}
struct Camera {
    x: f32,
    y: f32,
    z: f32
}
fn main_loop() {
        let x = 0.0;
        let y = 0.0;
        let mut stream = TcpStream::connect("localhost:5000").unwrap();
        let mut window = Window::new("Heartlands");
        window.set_light(Light::StickToCamera);
        
        let mut eye = Point3::new(1.0, 1.0, 0.0);
        let mut at = Point3::new(14.0,11.0,310.0);
        let mut arc_ball = ArcBall::new(eye, at);
        arc_ball.set_dist(0.1);
        //arc_ball.set_yaw(0.5);
        let mut camera = Camera {
            x: 14.0,
            y: 11.0,
            z: 310.0
        };
        let mut compare_time = SystemTime::now();
        while window.render_with_camera(&mut arc_ball) {
            let delta = SystemTime::now().duration_since(compare_time).unwrap();
            if delta.as_millis()/10 != 0 {
                println!("FPS: {}", 100 / (delta.as_millis()/10));
            }
            let point = world_structs::Chunk_point {x: 0, y: 0};
            let msg = serde_json::to_string(&point).unwrap(); 
            let mut buf = [0; 10024];
            let w = stream.write(msg.as_bytes());
            stream.read(&mut buf);
            let  res = match from_utf8(&buf) {
                Ok(v) => v,
                Err(e) => panic!("Invalid sequence: {}", e),
            }.replace("\0", "").replace("\n", "").to_string();
            res.trim();

            let curr_yaw = arc_ball.yaw();
            for event in window.events().iter() {
                match event.value {
                    WindowEvent::Key(key, Action::Release, _) => {
                        if key == Key::W {
                             camera.y += 1.0;
                             arc_ball.set_at(Point3::new(camera.x,camera.y,camera.z));
                             

                        } else if key == Key::A {

                             camera.x += 1.0;
                             arc_ball.set_at(Point3::new(camera.x,camera.y,camera.z));
                        }
                         else if key == Key::S {

                             camera.y -= 1.0;
                             arc_ball.set_at(Point3::new(camera.x,camera.y,camera.z));
                        }
                         else if key == Key::D {
                             
                             camera.x -= 1.0;
                             arc_ball.set_at(Point3::new(camera.x,camera.y,camera.z));
                        }
                    }
                    _ => {}
                }
            }
            let chunk: world_structs::Chunk = match serde_json::from_str(&res) {
                Ok(v) => v,
                Err(e) => panic!("Invalid sequence: {}", e),
            };





            for i in 0..chunk.points.len() {
                for j in 0..chunk.points.len() {
                     let a = Point3::new(chunk.points[i][j].x/1.0, chunk.points[i][j].y/1.0, chunk.points[i][j].z/1.0);
                     //println!("{}, {}, {}", chunk.points[i][j].x, chunk.points[i][j].y, chunk.points[i][j].z);
                     window.draw_point(&a, &Point3::new(0.0, 1.0, 0.0));
                }
            }
            compare_time = SystemTime::now();
            thread::sleep(time::Duration::from_millis(16));
            
        }

            println!("Socket connection ended.");
}
pub fn run() {
   main_loop();
}
