extern crate futures;
extern crate tokio;
extern crate websocket;
use crate::client_structs::{ClientPacket, PlayerAction};
use crate::graphics_utils::Camera;
use crate::world_structs::{
    ActionType, Biome, CategoryType, Chunk, Collider, ColliderType, Entity, EntityType, ItemType,
    Point, Prop, PropType, ReligionType, TaskType, TileType, World, WorldData,
};
use rand::Rng;
use rayon::prelude::*;
use serde_json;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use tokio::runtime;
use tokio::runtime::TaskExecutor;

use websocket::message::OwnedMessage;
use websocket::server::r#async::Server;
use websocket::server::InvalidConnection;

use futures::future::{self, Loop};
use futures::{Future, Sink, Stream};

use std::sync::{Arc, RwLock};

struct ClientState {
    x: f32,
    y: f32,
}
pub fn serve(world: World) {
    let runtime = runtime::Builder::new().build().unwrap();
    let executor = runtime.executor();
    let server =
        Server::bind("127.0.0.1:5000", &runtime.reactor()).expect("Failed to create server");
    println!("Server running!");
    let connections = Arc::new(RwLock::new(HashMap::new()));
    let world = Arc::new(RwLock::from(world.clone()));
    let client_states = Arc::new(RwLock::from(HashMap::new()));
    let counter = Arc::new(RwLock::new(0));
    let connections_inner = connections.clone();
    let world_inner = world.clone();
    let client_states_inner = client_states.clone();
    let executor_inner = executor.clone();
    let connection_handler = server
        .incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(move |(upgrade, addr)| {
            let connections_inner = connections_inner.clone();
            let world = world_inner.clone();
            let client_states = client_states_inner.clone();
            let counter_inner = counter.clone();
            let executor_to_inner = executor_inner.clone();
            let accept = upgrade
                .accept()
                .and_then(move |(framed, _)| {
                    let (sink, stream) = framed.split();

                    {
                        let mut c = counter_inner.write().unwrap();
                        *c += 1;
                    }

                    let id = *counter_inner.read().unwrap();
                    println!("Client connected!");
                    connections_inner.write().unwrap().insert(id, sink);
                    let c = *counter_inner.read().unwrap();
                    let f = stream
                        .for_each(move |msg| {
                            process_message(
                                c.try_into().unwrap(),
                                &msg,
                                client_states.clone(),
                                &world,
                            );
                            Ok(())
                        })
                        .map_err(|_| ());

                    executor_to_inner.spawn(f);

                    Ok(())
                })
                .map_err(|_| ());

            executor_inner.spawn(accept);
            Ok(())
        })
        .map_err(|_| ());

    // game loop
    let send_handler = future::loop_fn((), move |_| {
        let connections_inner = connections.clone();
        let executor = executor.clone();
        let world_inner = world.clone();
        let client_states_inner = client_states.clone();
        world.write().unwrap().update_entities();
        world
            .write()
            .unwrap()
            .update_political_and_religion_situation();
        tokio::timer::Delay::new(Instant::now() + Duration::from_millis(10))
            .map_err(|_| ())
            .and_then(move |_| {
                let mut conn = connections_inner.write().unwrap();
                let ids = conn.iter().map(|(k, v)| k.clone()).collect::<Vec<_>>();
                for id in ids.iter() {
                    let sink = conn.remove(id).unwrap();

                    world_inner.write().unwrap().world_data.day_night_cycle_time += 10;
                    let world = world_inner.read().unwrap();
                    let client_states = client_states_inner.clone();
                    let mut x = 0.0;
                    let mut y = 0.0;
                    match client_states.write().unwrap().get(id) {
                        Some(c) => {
                            x = c.x;
                            y = c.y;
                        }
                        None => (),
                    }
                    let serial_world = world.get(x as i32, y as i32);
                    let connections = connections_inner.clone();
                    let id = id.clone();

                    // send state to client
                    let f = sink
                        .send(OwnedMessage::Text(serial_world))
                        .and_then(move |sink| {
                            connections.write().unwrap().insert(id.clone(), sink);
                            Ok(())
                        })
                        .map_err(|_| ());

                    executor.spawn(f);
                }

                match true {
                    true => Ok(Loop::Continue(())),
                    false => Ok(Loop::Break(())),
                }
            })
    });

    runtime
        .block_on_all(connection_handler.select(send_handler))
        .map_err(|_| println!("Error!"))
        .unwrap();
}

// update state
fn process_message(
    id: u32,
    msg: &OwnedMessage,
    client_states: Arc<RwLock<HashMap<i32, ClientState>>>,
    world: &Arc<RwLock<World>>,
) {
    let mut rng = rand::thread_rng();
    if !client_states.write().unwrap().contains_key(&(id as i32)) {
        client_states
            .write()
            .unwrap()
            .insert((id as i32), ClientState { x: 0.0, y: 0.0 });
    }
    if let OwnedMessage::Binary(ref txt) = *msg {
        //let cut_string = txt.as_str()[0..txt.len() - 0].replace("\\", "");
        let decoded: ClientPacket = bincode::deserialize(&txt).unwrap();
        let packet: ClientPacket = decoded;

        client_states
            .write()
            .unwrap()
            .entry(id as i32)
            .and_modify(|e| {
                e.x = packet.camera.x;
                e.y = packet.camera.y
            });
        let player = packet.player;
        let id = rng.gen_range(0..999999);
        if player.shoot_data.shooting {
            if player.shoot_data.action_type == PlayerAction::Meteoroid {
                (*world).write().unwrap().colliders.push(Collider {
                    x: player.shoot_data.mx as f32,
                    y: player.shoot_data.my as f32 - 222.0,
                    id: id,
                    life_y: player.shoot_data.my as f32,
                    speed: 32.0,
                    dir: 3.14 / 2.0,
                    time: 0,
                    lifetime: 1000,
                    collider_type: ColliderType::Meteoroid,
                    owner_id: player.id,
                    hp: 1,
                    lethal: false,
                });
            }
        } else if player.shoot_data.action_type == PlayerAction::Raft {
            (*world).write().unwrap().props.push(Prop {
                x: (player.shoot_data.mx as f32 / 16.0).floor() * 16.0,
                y: (player.shoot_data.my as f32 / 16.0).floor() * 16.0,
                prop_type: PropType::Raft,
            });
        } else if player.shoot_data.action_type == PlayerAction::Siphon {
            (*world).write().unwrap().colliders.push(Collider {
                x: player.shoot_data.mx as f32,
                y: player.shoot_data.my as f32,
                id: id,
                life_y: player.shoot_data.my as f32 + 50.0,
                speed: 0.0,
                dir: 3.14 / 2.0,
                collider_type: ColliderType::SoulTrap,
                owner_id: player.id,
                time: 0,
                lifetime: 100,
                hp: 1,
                lethal: false,
            });
        }
        for row in (*world).write().unwrap().chunks.iter_mut() {
            for chunk in row.iter_mut() {
                for (key, val) in &mut chunk.entities {
                    for (k, v) in packet.faction_relations.iter() {
                        if &val.faction == k {
                            if v < &0 {
                                val.target_id = player.id;
                            }
                        }
                    }
                }
            }
        }
        let mut player_in = false;
        for p in &(*world).write().unwrap().players {
            if p.id == player.id {
                player_in = true;
            }
        }
        if player_in {
            let index = (*world)
                .write()
                .unwrap()
                .players
                .iter()
                .position(|r| r.id == player.id)
                .unwrap();
            (*world).write().unwrap().players[index].x = player.x;
            (*world).write().unwrap().players[index].y = player.y;
            (*world).write().unwrap().players[index].energy = player.energy;
        } else {
            (*world).write().unwrap().players.push(player);
        }
        // client_states.write().unwrap().entry(id.unwrap().y = camera.y;
    }
    /*if let OwnedMessage::Text(ref txt) = *msg {
        println!("Received msg '{}' from id {}", txt, id);

        if txt == "right" {
            world
                .write()
                .unwrap()
                .entry(id)
                .and_modify(|e| e.pos.0 += 10);
        } else if txt == "left" {
            world
                .write()
                .unwrap()
                .entry(id)
                .and_modify(|e| e.pos.0 -= 10);
        } else if txt == "down" {
            world
                .write()
                .unwrap()
                .entry(id)
                .and_modify(|e| e.pos.1 += 10);
        } else if txt == "up" {
            world
                .write()
                .unwrap()
                .entry(id)
                .and_modify(|e| e.pos.1 -= 10);
        }
    }*/
}
