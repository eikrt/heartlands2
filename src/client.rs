use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{thread, time};
use crate::world_structs;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32
}
fn main_loop() {
        let x = 0.0;
        let y = 0.0;
        let mut stream = TcpStream::connect("localhost:5000").unwrap();
        loop {
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
            let chunk: world_structs::Chunk = match serde_json::from_str(&res) {
                Ok(v) => v,
                Err(e) => panic!("Invalid sequence: {}", e),
            };

            thread::sleep(time::Duration::from_millis(32));
        }

            println!("Socket connection ended.");
}
pub fn run() {
   main_loop();
}
