use crate::world_structs;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::Arc;
use std::str::from_utf8;
use serde_json;
fn handle(mut stream: TcpStream, world: Arc<world_structs::World>) {
    let mut data = [0 as u8; 256];
    let world_clone = &*world;
    while match stream.read(&mut data) {
        Ok(size) => {

            let mut res = match from_utf8(&data) {
                Ok(v) => v,
                Err(e) => panic!("Invalid sequence: {}", e),
            }.replace("\0", "").replace("\n", "").to_string();
            res.trim();
            let res_obj: world_structs::Chunk_point = match serde_json::from_str(&res) {
                Ok(v) => v,
                Err(e) => panic!("Invalid sequence: {}", e),
            };
            let msg = serde_json::to_string(&world_clone.chunks[res_obj.x as usize][res_obj.y as usize]).unwrap(); 
            stream.write(msg.as_bytes()).unwrap();
            true
        },
        Err(_) => {
            println!("Error occurred, closing connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
pub fn serve(world: world_structs::World, port: i32) {
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:5000").unwrap();
        println!("Server listening on port 5000");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {

                    let world_clone = world.clone();
                    println!("New connection with: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        handle(stream, Arc::new(world_clone));
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        drop(listener); 

    });
}