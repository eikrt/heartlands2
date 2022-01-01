extern crate futures;
extern crate tokio;
extern crate websocket;
use crate::graphics_utils;
use crate::world_structs;
use serde_json;
use tokio::runtime;
use tokio::runtime::TaskExecutor;

use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use websocket::message::OwnedMessage;
use websocket::server::r#async::Server;
use websocket::server::InvalidConnection;

use futures::future::{self, Loop};
use futures::{Future, Sink, Stream};

use std::sync::{Arc, RwLock};

pub fn serve(world: world_structs::World) {
    let runtime = runtime::Builder::new().build().unwrap();
    let executor = runtime.executor();
    let server =
        Server::bind("127.0.0.1:5000", &runtime.reactor()).expect("Failed to create server");
    println!("Server running!");
    let connections = Arc::new(RwLock::new(HashMap::new()));
    let world = Arc::new(RwLock::from(world.clone()));
    let counter = Arc::new(RwLock::new(0));
    let connections_inner = connections.clone();
    let world_inner = world.clone();
    let executor_inner = executor.clone();
    let connection_handler = server
        .incoming()
        .map_err(|InvalidConnection { error, .. }| error)
        .for_each(move |(upgrade, addr)| {
            let connections_inner = connections_inner.clone();
            let world = world_inner.clone();
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
                            process_message(c.try_into().unwrap(), &msg, world.clone());
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

        world.write().unwrap().update_entities();
        tokio::timer::Delay::new(Instant::now() + Duration::from_millis(100))
            .map_err(|_| ())
            .and_then(move |_| {
                let mut conn = connections_inner.write().unwrap();
                let ids = conn.iter().map(|(k, v)| k.clone()).collect::<Vec<_>>();

                for id in ids.iter() {
                    let sink = conn.remove(id).unwrap();

                    let world = world_inner.read().unwrap();
                    let serial_world = format!("{}", world);
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
fn process_message(id: u32, msg: &OwnedMessage, world: Arc<RwLock<world_structs::World>>) {
    if let OwnedMessage::Text(ref txt) = *msg {
        let cut_string = txt.as_str()[0..txt.len() - 0].replace("\\", "");
        let camera: graphics_utils::Camera = serde_json::from_str(&cut_string).unwrap();
        world.write().unwrap().v_x = camera.x as i32;
        world.write().unwrap().v_y = camera.y as i32;
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
