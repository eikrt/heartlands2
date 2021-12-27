use crate::world_structs;
use std::thread;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use std::io::{Read, Write};
use std::time::{SystemTime};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::mpsc;
use std::str::from_utf8;
use serde_json;
/*async fn handle(mut stream: TcpStream, mut world: world_structs::World){

    let mut entities: Vec<world_structs::Entity> = Vec::new();
    let mut compare_time = SystemTime::now();
    let mut data = [0 as u8; 65536];
    let mut update_political_change = 0.0;
    let mut update_political_time = 10.0;
    let ant_number_to_change_ownership = 3;
    let mut is_ok = true;
    //let (sender, receiver): (mpsc::Sender<world_structs::World>, mpsc::Receiver<world_structs::World>)  = mpsc::channel();
    let (mut sender, mut receiver) = mpsc::channel(100);
    println!("dsf");
    tokio::spawn(async move {
        while true {
//            world.update_entities();
            println!("dsf");
          sender.send(world.clone()).await;
        }
    });
    'main: while match stream.read(&mut data) {
        Ok(_size) => {
            // network stuff
            let world_from = receiver.recv().await.unwrap();
            let res = match from_utf8(&data) {
                Ok(_v) => _v,
                Err(e) => panic!("Invalid sequence: {}", e),
            }.replace("\0", "").replace("\n", "").to_string();
            let res_obj: world_structs::WorldRequest = match serde_json::from_str(&res) {
                Ok(v) => v,
                Err(e) => {is_ok = false;
                            world_structs::WorldRequest{
                            x: 0,
                            y: 0,
                            req_type: world_structs::RequestType::CHUNK
                    }
                    },
            };
            if res_obj.req_type == world_structs::RequestType::CHUNK {
                let response = world_structs::WorldResponse {
                    chunk: world_from.chunks[res_obj.x as usize][res_obj.y as usize].clone(),
                    entities: world_from.get_entities_for_chunk(world_from.chunks[res_obj.x as usize][res_obj.y as usize].clone()),
                    valid: true
                };
                let msg = serde_json::to_string(&response).unwrap();
                stream.write(msg.as_bytes()).unwrap();
            }

            else if res_obj.req_type == world_structs::RequestType::DATA{
                let msg = serde_json::to_string(&world_from.world_data).unwrap();
                stream.write(msg.as_bytes()).unwrap();
            }
            // game tick
            
            let delta = SystemTime::now().duration_since(compare_time).unwrap();
            let delta_as_millis = delta.as_millis();
            /*
            update_political_change += delta_as_millis as f32;
            if update_political_change > update_political_time {
                let mut biggest_value_data = (0,0,"Neutral".to_string());
                for row in world_clone.chunks.clone().iter().enumerate() {
                    for c in world_clone.chunks[row.0].clone().iter().enumerate() {
        
                    let mut entity_types: HashMap<String, i32> = HashMap::new();
                    let entities_for_chunks = world_clone.get_entities_for_chunk(c.1.clone());
                    if (entities_for_chunks.len() as i32) < ant_number_to_change_ownership {
                        world_clone.chunks[row.0][c.0].name = "Neutral".to_string();
                    }
                    for e in &entities_for_chunks {
                            if !entity_types.contains_key(&e.faction) {

                        if e.entity_type == world_structs::EntityType::DRONE_ANT {
                                entity_types.insert(
                                                        e.faction.clone(),
                                                        0
                                                    );

                            }
                            }
                            else {

                            if e.entity_type == world_structs::EntityType::DRONE_ANT {
                                    *entity_types.get_mut(&e.faction).unwrap() += 1;

                                }
                            }
                            let mut biggest_value = ("Neutral".to_string(), 0);
                            for (key, value) in &entity_types {
                                if value > &biggest_value.1 {
                                   biggest_value = (key.to_string(), *value) 
                                }
                            }
                            biggest_value_data = (row.0, c.0, biggest_value.0);
                            if biggest_value.1 <= ant_number_to_change_ownership {

                                biggest_value_data = (row.0, c.0, "Neutral".to_string());
                            }
                            world_clone.chunks[biggest_value_data.0][biggest_value_data.1].name = biggest_value_data.2;
                            
                    }
                     
                }

                }
                update_political_change = 0.0;
            }*/

            // end of tick stuff
            compare_time = SystemTime::now();

            data = [0 as u8; 65536];
            true
        },
        Err(_) => {
            println!("Error occurred, closing connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
/*pub async fn serve(world: world_structs::World, _port: i32) {
        let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
        println!("Server listening on port 5000");
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                process(socket).await;
            });
        }
}*/
*/
#[tokio::main]
pub async fn serve(mut world: world_structs::World, _port:i32) {

    let (mut sender, mut receiver) = mpsc::channel(100);
    let mut sender2 = sender.clone();
        tokio::spawn(async move {
            while true {
              world.update_entities();
              sender.send(world.clone()).await;
            }
        });
    if let Ok(mut tcp_listener) = TcpListener::bind("127.0.0.1:5000").await {
        println!("Running socket server...");
        while let Ok((mut tcp_stream, _socket_addr)) = tcp_listener.accept().await {
            println!("Client connected");
            while let Some(message) = receiver.recv().await {

            

                let mut data = [0; 65536];
                loop {
                    let world_from = receiver.recv().await.unwrap();

                    let n = match tcp_stream.read(&mut data).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => {
                                    n
                        },
                        Err(e) => {
                            println!("Failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    let res = match from_utf8(&data) {
                        Ok(_v) => _v,
                        Err(e) => panic!("Invalid sequence: {}", e),
                    }.replace("\0", "").replace("\n", "").to_string();
                    let res_obj: world_structs::WorldRequest = match serde_json::from_str(&res) {
                        Ok(v) => v,
                        Err(e) => {
                                    world_structs::WorldRequest{
                                    x: 0,
                                    y: 0,
                                    req_type: world_structs::RequestType::CHUNK
                            }
                            },
                    };
                    
                    if res_obj.req_type == world_structs::RequestType::CHUNK {
                        let response = world_structs::WorldResponse {
                            chunk: world_from.chunks[res_obj.x as usize][res_obj.y as usize].clone(),
                            entities: world_from.get_entities_for_chunk(world_from.chunks[res_obj.x as usize][res_obj.y as usize].clone()),
                            valid: true
                        };
                        let msg = serde_json::to_string(&response).unwrap();
                        tcp_stream.write_all(msg.as_bytes()).await;
                    }

                    else if res_obj.req_type == world_structs::RequestType::DATA{
                        let msg = serde_json::to_string(&world_from.world_data).unwrap();
                        tcp_stream.write_all(msg.as_bytes()).await;
                    }
                    // game tick
                    
                    /*let delta = SystemTime::now().duration_since(compare_time).unwrap();
                    let delta_as_millis = delta.as_millis();*/

                    /*if let Err(e) = tcp_stream.write_all(&data[0..n]).await {
                        eprintln!("Failed to write to socket; err = {:?}", e);
                        return;
                    }*/
                }
            }
        }
    }
}
