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
        let mut stream = TcpStream::connect("localhost:5000").unwrap();
        let mut window = Window::new("Heartlands");
        window.set_light(Light::StickToCamera);
        

        let mut dist = 3.0;
        let mut at = Point3::new(8.0,8.0,0.0);
        let mut eye = Point3::new(8.0, 8.0, 333.0);
        let mut arc_ball = ArcBall::new(eye, at);
        let mut first_person = FirstPerson::new(eye,at);
        //arc_ball.set_yaw(0.5);
        //arc_ball.set_pitch(1.54);
        //first_person.move_dir(false,true,true,true);
        //first_person.set_up_axis(Vector3::new(0.0,0.0,0.0));

        let mut compare_time = SystemTime::now();
        while window.render_with_camera(&mut arc_ball) {
            let delta = SystemTime::now().duration_since(compare_time).unwrap();
            if delta.as_millis()/10 != 0 {
             //   println!("FPS: {}", 100 / (delta.as_millis()/10));
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

            for event in window.events().iter() {
                match event.value {
                    WindowEvent::Key(key, Action::Release, _) => {
                        if key == Key::W {
                             at.y += 1.0;
                             eye.y += 1.0;
                             arc_ball.look_at(eye,at);

                        } else if key == Key::A {

                             at.x -= 1.0;
                             eye.x -= 1.0;
                             arc_ball.look_at(eye,at);
                        }
                         else if key == Key::S {

                             at.y -= 1.0;
                             eye.y -= 1.0;
                             arc_ball.look_at(eye,at);
                        }
                         else if key == Key::D {
                             
                             at.x += 1.0;
                             eye.x += 1.0;
                             arc_ball.look_at(eye,at);
                        }
                        else if key == Key::Up {
                             eye.y -= 1.0;
                             arc_ball.look_at(eye,at);

                        } else if key == Key::Left {

                             eye.x += 1.0;
                             arc_ball.look_at(eye,at);
                        }
                         else if key == Key::Down {

                             eye.y += 1.0;
                             arc_ball.look_at(eye,at);
                        }
                         else if key == Key::Right {
                             
                             eye.x -= 1.0;
                             arc_ball.look_at(eye,at);
                        }
                    }
                    _ => {}
                }
            }

        //arc_ball.set_dist(dist);
            let chunk: world_structs::Chunk = match serde_json::from_str(&res) {
                Ok(v) => v,
                Err(e) => panic!("Invalid sequence: {}", e),
            };



            //println!("{}", arc_ball.pitch());
            for i in 0..chunk.points.len() {
                for j in 0..chunk.points.len() {
                     let a = Point3::new(chunk.points[i][j].y/1.0, chunk.points[i][j].x/1.0, chunk.points[i][j].z/1.0);
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
