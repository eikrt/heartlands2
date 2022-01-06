extern crate websocket;
use crate::graphics_utils;

use bincode;

use crate::client_structs;
use crate::client_structs::{ClientPacket, Player};
use crate::graphics_utils::{Camera, MoveDirection};
use crate::world_structs::{
    ActionType, CategoryType, Chunk, Entity, EntityType, ItemType, ReligionType, TaskType,
    TileType, World, WorldData, HATCH_TIME,
};
use lerp::Lerp;
use rand::Rng;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadSurface, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::mouse::MouseWheelDirection;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::FullscreenType;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::future::Future;
use std::io::stdin;
use std::io::{Read, Write};
use std::iter::FromIterator;
use std::option::Option;
use std::pin::Pin;
use std::str::from_utf8;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};
const SCREEN_WIDTH: u32 = 426;
const SCREEN_HEIGHT: u32 = 240;
const HUD_LOC: u32 = 336;

const TILE_SIZE: f32 = 16.0;
const WORKER_ANIMATION_SPEED: u128 = 25;
const DRONE_ANIMATION_SPEED: u128 = 25;
const QUEEN_ANIMATION_SPEED: u128 = 25;
const SOLDIER_ANIMATION_SPEED: u128 = 25;
const MECHANT_ANIMATION_SPEED: u128 = 25;
const WATER_ANIMATION_SPEED: u128 = 800;
const ANIMATION_RANDOM: u128 = 50;
const CAMERA_BUFFER_TOP: f32 = 64.0;
const CAMERA_BUFFER_LEFT: f32 = 96.0;
const CAMERA_BUFFER_RIGHT: f32 = 96.0;
const CAMERA_BUFFER_BOTTOM: f32 = 100.0;

fn main_loop() -> Result<(), String> {
    // sdl stuff
    let url = "ws://127.0.0.1:5000";
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window("Mechants", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");
    let icon: Surface = LoadSurface::from_file("res/icon2.png").unwrap();
    window.set_icon(icon);
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
    canvas.set_blend_mode(BlendMode::Blend);

    let mut rng = rand::thread_rng();

    //canvas.window_mut().set_fullscreen(FullscreenType::True);

    // canvas.window_mut().set_size(500, 500);
    // canvas.window_mut().set_resizable(true);
    // texture stuff
    let texture_creator = canvas.texture_creator();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    // font stuff
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let desc_font_size = 20;
    let mut font = ttf_context.load_font("fonts/PixelOperator.ttf", desc_font_size)?;

    let hp_font_size = 10;
    let mut hp_font = ttf_context.load_font("fonts/PixelOperator.ttf", hp_font_size)?;
    let tile_gs = graphics_utils::tile_graphics();

    let mut camera = graphics_utils::Camera {
        x: rng.gen_range(256.0..1024.0),
        y: rng.gen_range(256.0..1024.0),
        zoom: 1.0,
        zoom_speed: 0.05,
        move_speed: 20.0,
    };

    let bg_color = Color::RGB(0, 0, 0);
    //let mut stream = TcpStream::connect("localhost:5000").unwrap();

    let mut running = true;
    // controls
    let mut w = false;
    let mut a = false;
    let mut s = false;
    let mut d = false;
    let mut up = false;
    let mut left = false;
    let mut down = false;
    let mut right = false;
    let mut zoom_button_plus = false;
    let mut zoom_button_minus = false;
    let mut event_pump = sdl_context.event_pump()?;
    let mut compare_time = SystemTime::now();
    let mut update_data = true;
    let mut world_data: WorldData = WorldData {
        ..Default::default()
    };
    // mouse
    let mut mouse_not_moved_for = 0;
    let mut mouse_state = MouseState::new(&event_pump);
    let hover_time = 75;
    // chunks and entities
    let mut chunk_fetch_width = 2;
    let mut chunk_fetch_height = 2;
    let mut chunk_fetch_x = -1;
    let mut chunk_fetch_y = -1;
    let mut chunks: Vec<Vec<Chunk>> = Vec::new();
    let mut entities: HashMap<i32, Entity> = HashMap::new();
    // menu buttons
    let mut play_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
        y: 64.0 + 4.0,
        width: 128.0,
        height: 32.0,
    };
    let mut settings_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
        y: 128.0 + 4.0,
        width: 128.0,
        height: 32.0,
    };
    let mut exit_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
        y: 192.0 + 4.0,
        width: 128.0,
        height: 32.0,
    };
    // ui buttons
    let mut normal_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: 4 as f32,
        y: (SCREEN_HEIGHT - 32 - 8) as f32,
        width: 32.0,
        height: 32.0,
    };
    let mut political_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: 4.0 + 32.0 + 4.0,
        y: (SCREEN_HEIGHT - 32 - 8) as f32,
        width: 32.0,
        height: 32.0,
    };

    let mut religion_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: 4.0 + 64.0 + 8.0,
        y: (SCREEN_HEIGHT - 32 - 8) as f32,
        width: 32.0,
        height: 32.0,
    };
    // entity textures
    let oak_texture = texture_creator.load_texture("res/oak.png")?;
    let birch_texture = texture_creator.load_texture("res/birch.png")?;
    let appletree_texture = texture_creator.load_texture("res/appletree.png")?;
    let pine_texture = texture_creator.load_texture("res/pine.png")?;
    let spruce_texture = texture_creator.load_texture("res/spruce.png")?;
    let cactus_texture = texture_creator.load_texture("res/cactus.png")?;
    let ant_egg_texture = texture_creator.load_texture("res/ant_egg.png")?;
    let ant_egg_texture_2 = texture_creator.load_texture("res/ant_egg_2.png")?;
    let ant_egg_texture_3 = texture_creator.load_texture("res/ant_egg_3.png")?;
    let ant_egg_texture_4 = texture_creator.load_texture("res/ant_egg_4.png")?;
    let ant_worker_texture_1 = texture_creator.load_texture("res/ant_worker.png")?;
    let ant_worker_texture_2 = texture_creator.load_texture("res/ant_worker_2.png")?;
    let ant_soldier_texture_1 = texture_creator.load_texture("res/ant1.png")?;
    let ant_soldier_texture_2 = texture_creator.load_texture("res/ant1.png")?;
    let ant_drone_texture_1 = texture_creator.load_texture("res/ant_drone.png")?;
    let ant_drone_texture_2 = texture_creator.load_texture("res/ant_drone_2.png")?;
    let mechant_texture_1 = texture_creator.load_texture("res/mechant.png")?;
    let mechant_texture_2 = texture_creator.load_texture("res/mechant.png")?;
    let plasmant_texture_1 = texture_creator.load_texture("res/plasmant.png")?;
    let plasmant_texture_2 = texture_creator.load_texture("res/plasmant_2.png")?;
    let cultist_ant_texture_1 = texture_creator.load_texture("res/plasmant.png")?;
    let cultist_ant_texture_2 = texture_creator.load_texture("res/plasmant_2.png")?;
    let ant_queen_texture_1 = texture_creator.load_texture("res/ant_queen.png")?;
    let ant_queen_texture_2 = texture_creator.load_texture("res/ant_queen.png")?;
    let snail_texture = texture_creator.load_texture("res/snail.png")?;
    let food_storage_texture = texture_creator.load_texture("res/food_storage.png")?;

    // item textures
    let fruit_texture = texture_creator.load_texture("res/fruit.png")?;
    let wooden_spear_texture = texture_creator.load_texture("res/wooden_spear.png")?;
    let wooden_shovel_texture = texture_creator.load_texture("res/wooden_shovel.png")?;
    // tile textures
    let mut grass_texture = texture_creator.load_texture("res/grass.png")?;
    let mut water_texture = texture_creator.load_texture("res/water.png")?;
    let mut water_texture_2 = texture_creator.load_texture("res/water_2.png")?;
    let mut ice_texture = texture_creator.load_texture("res/ice.png")?;
    let mut sand_texture = texture_creator.load_texture("res/sand.png")?;
    // menu textures
    let mut menu_button_texture = texture_creator.load_texture("res/menu_button.png")?;
    let mut menu_button_hovered_texture =
        texture_creator.load_texture("res/menu_button_hovered.png")?;
    let mut menu_button_pressed_texture =
        texture_creator.load_texture("res/menu_button_pressed.png")?;
    let mut menu_background = texture_creator.load_texture("res/background_image_1.png")?;

    // ui textures

    let mut ui_button_texture = texture_creator.load_texture("res/ui_button.png")?;
    let mut ui_button_hovered_texture =
        texture_creator.load_texture("res/ui_button_hovered.png")?;
    let mut ui_button_pressed_texture =
        texture_creator.load_texture("res/ui_button_pressed.png")?;

    // hud textures
    let mut hud_texture = texture_creator.load_texture("res/hud.png")?;
    // other texture stuff

    // description stuff
    let descriptions_for_entities = graphics_utils::get_descriptions_for_entities();
    let descriptions_for_tiles = graphics_utils::get_descriptions_for_tiles();
    let descriptions_for_religions = graphics_utils::get_descriptions_for_religions();
    let sprite_4 = Rect::new(0, 0, (4.0 * camera.zoom) as u32, (4.0 * camera.zoom) as u32);
    let sprite_1x5 = Rect::new(0, 0, (1.0 * camera.zoom) as u32, (5.0 * camera.zoom) as u32);
    let sprite_1x10 = Rect::new(
        0,
        0,
        (1.0 * camera.zoom) as u32,
        (10.0 * camera.zoom) as u32,
    );
    let sprite_426x48 = Rect::new(0, 0, (426.0) as u32, (48.0) as u32);
    let sprite_2x5 = Rect::new(0, 0, (2.0 * camera.zoom) as u32, (5.0 * camera.zoom) as u32);
    let sprite_8 = Rect::new(0, 0, (8.0 * camera.zoom) as u32, (8.0 * camera.zoom) as u32);
    let sprite_16 = Rect::new(
        0,
        0,
        (16.0 * camera.zoom) as u32,
        (16.0 * camera.zoom) as u32,
    );
    let sprite_32 = Rect::new(
        0,
        0,
        (32.0 * camera.zoom) as u32,
        (32.0 * camera.zoom) as u32,
    );
    let sprite_128x32 = Rect::new(
        0,
        0,
        (128.0 * camera.zoom) as u32,
        (32.0 * camera.zoom) as u32,
    );
    let sprite_720x480 = Rect::new(0, 0, 720.0 as u32, 480.0 as u32);

    // gameplay stuff

    let id = rng.gen_range(0..999999);
    let mut player = client_structs::Player {
        id: id,
        hp: 100,
        energy: 100,
        x: camera.x + 256.0,
        y: camera.y + 128.0,
        stopped: false,
        speed: 5.5,
        dir: 0.0,
        target_x: 0.0,
        target_y: 0.0,
        entity_type: EntityType::CultistAnt,
        religion_type: ReligionType::Nothing,
        category_type: CategoryType::Ant,
        faction: "The Fringe".to_string(),
        faction_id: 0,
        task_type: TaskType::Nothing,
        current_action: ActionType::Idle,
        wielding_item: ItemType::Nothing,
        backpack_item: ItemType::Nothing,
        wearable_item: ItemType::Nothing,
        backpack_amount: 0,
        time: 0,
    };
    let mut map_state = graphics_utils::MapState::Normal;
    let mut main_menu_on = true;
    let mut settings_menu_on = false;
    let mut chunk_graphics_data: HashMap<String, Color> = HashMap::new();
    let mut religion_graphics_data: HashMap<ReligionType, Color> = HashMap::new();
    // network stuff
    let (tx, rx): (Sender<OwnedMessage>, Receiver<OwnedMessage>) = channel();
    let (tx_w, rx_w): (Sender<String>, Receiver<String>) = channel();
    let tx_1 = tx.clone();
    let connect = |url: &str, rx: Receiver<OwnedMessage>, tx_1: Sender<OwnedMessage>| {
        let client = ClientBuilder::new(url)
            .unwrap()
            .add_protocol("rust-websocket")
            .connect_insecure()
            .unwrap();
        println!("Succesfully connected");
        let (mut receiver, mut sender) = client.split().unwrap();

        let send_loop = thread::spawn(move || {
            loop {
                // Send loop
                let message = match rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = sender.send_message(&message);
                        return;
                    }
                    _ => (),
                }
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                }
            }
        });
        let receive_loop = thread::spawn(move || {
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                    _ => {
                        tx_w.send(format!("{:?}", message));
                    }
                }
            }
        });
    };
    connect(url, rx, tx_1);
    while running {
        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();

        let delta_as_millis = delta.as_millis();
        if delta.as_millis() / 10 != 0 {
            //   println!("FPS: {}", 100 / (delta.as_millis()/10));
        }
        mouse_not_moved_for += delta_as_millis;
        canvas.set_draw_color(bg_color);
        canvas.clear();
        // canvas.fill_rect(Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT));
        // send message to server
        //
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                // WASD
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    w = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    a = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    s = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    d = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    up = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    left = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    right = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    down = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    zoom_button_plus = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    zoom_button_minus = true;
                }

                Event::MouseWheel { x, y, .. } => {
                    if y > 0 {
                        zoom_button_minus = true;
                    } else if y < 0 {
                        zoom_button_plus = true;
                    }
                }
                Event::MouseMotion { .. } => {
                    mouse_not_moved_for = 0;
                }
                // WASD
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    w = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    a = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    s = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    d = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    up = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    left = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    right = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    down = false;
                }

                Event::KeyUp {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    zoom_button_plus = false;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    zoom_button_minus = false;
                }

                _ => {}
            }
        }

        mouse_state = event_pump.mouse_state();
        let (width, height) = canvas.output_size().unwrap();
        let ratio_x = SCREEN_WIDTH as f32 / width as f32;
        let ratio_y = SCREEN_HEIGHT as f32 / height as f32;

        let margin_x = 0.0; //(width as f32 - canvas.logical_size().0 as f32 * ratio_x) / 2.0;
        let margin_y = 0.0; //(height as f32 - canvas.logical_size().1 as f32) / 2.0;
        let mouse_x = (((mouse_state.x() as f32 + camera.x) / camera.zoom * ratio_x)
            - margin_x as f32) as f32;
        let mouse_y = (((mouse_state.y() as f32 + camera.y) / camera.zoom * ratio_y)
            - margin_y as f32) as f32;
        let mx = mouse_state.x() as f32;
        let my = mouse_state.y() as f32;
        if main_menu_on {
            //render menu background
            graphics_utils::render(
                &mut canvas,
                &menu_background,
                Point::new(0, 0),
                sprite_720x480,
                1.0,
                ratio_x,
                ratio_y,
            );
            // render buttons
            let position = Point::new(play_button.x as i32, play_button.y as i32);
            play_button.check_if_hovered(mx, my, ratio_x, ratio_y);
            play_button.check_if_pressed(mx, my, mouse_state.left());
            settings_button.check_if_hovered(mx, my, ratio_x, ratio_y);
            settings_button.check_if_pressed(mx, my, mouse_state.left());
            exit_button.check_if_hovered(mx, my, ratio_x, ratio_y);
            exit_button.check_if_pressed(mx, my, mouse_state.left());
            // play button
            if play_button.status == graphics_utils::ButtonStatus::Hovered {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_hovered_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            } else if play_button.status == graphics_utils::ButtonStatus::Pressed {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_pressed_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            } else {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            }
            // settings button
            let position = Point::new(settings_button.x as i32, settings_button.y as i32);
            if settings_button.status == graphics_utils::ButtonStatus::Hovered {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_hovered_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            } else if settings_button.status == graphics_utils::ButtonStatus::Pressed {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_pressed_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            } else {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            }
            // exit button
            let position = Point::new(exit_button.x as i32, exit_button.y as i32);
            if exit_button.status == graphics_utils::ButtonStatus::Hovered {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_hovered_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            } else if exit_button.status == graphics_utils::ButtonStatus::Pressed {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_pressed_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            } else {
                graphics_utils::render(
                    &mut canvas,
                    &menu_button_texture,
                    position,
                    sprite_128x32,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
            }
            // render texts
            let title_text = graphics_utils::get_text(
                "MechAnts".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            let position = Point::new((SCREEN_WIDTH / 2 - 42) as i32, 16 as i32);
            graphics_utils::render_text(
                &mut canvas,
                &title_text.text_texture,
                position,
                title_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let text_margin = 4;
            let play_text = graphics_utils::get_text(
                "Play".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            let position = Point::new(
                play_button.x as i32 + text_margin,
                play_button.y as i32 + text_margin,
            );
            graphics_utils::render_text(
                &mut canvas,
                &play_text.text_texture,
                position,
                play_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let settings_text = graphics_utils::get_text(
                "Settings".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            let position = Point::new(
                settings_button.x as i32 + text_margin,
                settings_button.y as i32 + text_margin,
            );
            graphics_utils::render_text(
                &mut canvas,
                &settings_text.text_texture,
                position,
                settings_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let exit_text = graphics_utils::get_text(
                "Exit".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            let position = Point::new(
                exit_button.x as i32 + text_margin,
                exit_button.y as i32 + text_margin,
            );
            graphics_utils::render_text(
                &mut canvas,
                &exit_text.text_texture,
                position,
                exit_text.text_sprite,
                ratio_x,
                ratio_y,
            );

            if play_button.status == graphics_utils::ButtonStatus::Released {
                main_menu_on = false;
            } else if settings_button.status == graphics_utils::ButtonStatus::Released {
            } else if exit_button.status == graphics_utils::ButtonStatus::Released {
                running = false;
            }
        } else {
            /*if up {
                camera.mov(graphics_utils::MoveDirection::Up, delta_as_millis);
            }
            if left {
                camera.mov(graphics_utils::MoveDirection::Left, delta_as_millis);
            }
            if down {
                camera.mov(graphics_utils::MoveDirection::Down, delta_as_millis);
            }
            if right {
                camera.mov(graphics_utils::MoveDirection::Right, delta_as_millis);
            }*/
            if w {
                player.mov(graphics_utils::MoveDirection::Up, delta_as_millis);
                if player.get_relative_y(&camera) <= CAMERA_BUFFER_TOP {
                    camera.mov(
                        graphics_utils::MoveDirection::Up,
                        player.speed,
                        delta_as_millis,
                    );
                }
            }
            if a {
                player.mov(graphics_utils::MoveDirection::Left, delta_as_millis);
                if player.get_relative_x(&camera) <= CAMERA_BUFFER_LEFT {
                    camera.mov(
                        graphics_utils::MoveDirection::Left,
                        player.speed,
                        delta_as_millis,
                    );
                }
            }
            if s {
                player.mov(graphics_utils::MoveDirection::Down, delta_as_millis);
                if player.get_relative_y(&camera) >= height as f32 - CAMERA_BUFFER_BOTTOM {
                    camera.mov(
                        graphics_utils::MoveDirection::Down,
                        player.speed,
                        delta_as_millis,
                    );
                }
            }
            if d {
                player.mov(graphics_utils::MoveDirection::Right, delta_as_millis);
                if player.get_relative_x(&camera) >= width as f32 - CAMERA_BUFFER_RIGHT {
                    camera.mov(
                        graphics_utils::MoveDirection::Right,
                        player.speed,
                        delta_as_millis,
                    );
                }
            }
            if zoom_button_plus {
                camera.zoom(graphics_utils::MoveDirection::Zoomin, delta_as_millis);
                zoom_button_plus = false;
            }
            if zoom_button_minus {
                camera.zoom(graphics_utils::MoveDirection::Zoomout, delta_as_millis);
                zoom_button_minus = false;
            }
            // tick
            player.tick(delta_as_millis);
            // get entities and chunks from server
            let encoded: Vec<u8> = bincode::serialize(&camera).unwrap();
            let decoded: graphics_utils::Camera = bincode::deserialize(&encoded).unwrap();

            match tx.send(OwnedMessage::Binary(encoded)) {
                Ok(()) => (),
                Err(e) => {
                    break;
                }
            }
            match rx_w.try_recv() {
                Ok(w) => {
                    let cut_string = &w.as_str()[6..w.len() - 2].replace("\\", "");
                    let world_from: World = serde_json::from_str(cut_string).unwrap();
                    chunks = world_from.chunks;
                    world_data = world_from.world_data;
                }
                Err(e) => (),
            }
            // iterate chunks
            for i in 0..chunks.len() {
                for j in 0..chunks[i].len() {
                    if !chunk_graphics_data.contains_key(&chunks[i][j].name) {
                        chunk_graphics_data.insert(
                            chunks[i][j].name.clone(),
                            Color::RGBA(
                                rng.gen_range(0..255),
                                rng.gen_range(0..255),
                                rng.gen_range(0..255),
                                55,
                            ),
                        );
                    }
                    for i in 0..chunks.len() {
                        for j in 0..chunks[i].len() {
                            if !religion_graphics_data.contains_key(&chunks[i][j].religion) {
                                religion_graphics_data.insert(
                                    chunks[i][j].religion.clone(),
                                    Color::RGBA(
                                        rng.gen_range(0..255),
                                        rng.gen_range(0..255),
                                        rng.gen_range(0..255),
                                        55,
                                    ),
                                );
                            }
                        }
                    }
                    for k in 0..chunks[i][j].points.len() {
                        for h in 0..chunks[i][j].points.len() {
                            let p = &chunks[i][j].points[k][h];
                            let tx = p.x * TILE_SIZE * camera.zoom - camera.x;
                            let ty = p.y * TILE_SIZE * camera.zoom - camera.y;
                            if tx < -64.0 || ty < -64.0 {
                                continue;
                            }

                            if tx > SCREEN_WIDTH as f32 || ty > SCREEN_HEIGHT as f32 {
                                continue;
                            }
                            let light = 1.0;
                            let r_result = ((tile_gs.get(&p.tile_type).unwrap().sc.r as f32)
                                .lerp(tile_gs.get(&p.tile_type).unwrap().tc.r as f32, p.z / 512.0)
                                / light) as u8;
                            let g_result = ((tile_gs.get(&p.tile_type).unwrap().sc.g as f32)
                                .lerp(tile_gs.get(&p.tile_type).unwrap().tc.g as f32, p.z / 512.0)
                                / light) as u8;
                            let b_result = ((tile_gs.get(&p.tile_type).unwrap().sc.b as f32)
                                .lerp(tile_gs.get(&p.tile_type).unwrap().tc.b as f32, p.z / 512.0)
                                / light) as u8;
                            // canvas.set_draw_color(Color::RGB(r_result,g_result,b_result));

                            let tx = (p.x) * TILE_SIZE * camera.zoom - camera.x;
                            let ty = (p.y) * TILE_SIZE * camera.zoom - camera.y;
                            let position = Point::new(tx as i32, ty as i32);
                            let mut texture = &grass_texture;
                            if p.tile_type == TileType::Grass {
                                texture = &grass_texture;
                            } else if p.tile_type == TileType::Water {
                                texture = &water_texture;
                                if (time / (WATER_ANIMATION_SPEED)) % 2 == 0 {
                                    texture = &water_texture_2;
                                }
                            } else if p.tile_type == TileType::Ice {
                                texture = &ice_texture;
                            } else if p.tile_type == TileType::Sand
                                || p.tile_type == TileType::RedSand
                            {
                                texture = &sand_texture;
                            }
                            graphics_utils::render_tile_with_color(
                                &mut canvas,
                                texture,
                                position,
                                sprite_16,
                                Color::RGBA(r_result, g_result, b_result, 175),
                                camera.zoom,
                                ratio_x,
                                ratio_y,
                            );
                        }
                    }
                }

                //render entities
                for i in 0..chunks.len() {
                    for j in 0..chunks[i].len() {
                        let mut entities_vals: Vec<Entity> =
                            chunks[i][j].entities.values().cloned().collect();

                        entities_vals.sort_by(|a, b| a.id.cmp(&b.id));
                        for entity in entities_vals.iter() {
                            let tx = (entity.x) * camera.zoom - camera.x;
                            let ty = (entity.y) * camera.zoom - camera.y;
                            let tx_ant = (entity.x) * camera.zoom - camera.x;
                            let ty_ant = (entity.y) * camera.zoom - camera.y;
                            let tx_tree = (entity.x + TILE_SIZE / 2.0) * camera.zoom - camera.x;
                            let ty_tree = (entity.y - TILE_SIZE / 4.0) * camera.zoom - camera.y;
                            if entity.hp < 0 {
                                continue;
                            }

                            canvas.set_draw_color(Color::RGB(0, 0, 0));

                            if tx < -64.0 || ty < -64.0 {
                                continue;
                            }

                            if tx > SCREEN_WIDTH as f32 || ty > SCREEN_HEIGHT as f32 {
                                continue;
                            }

                            // trees
                            if entity.entity_type == EntityType::Oak {
                                let position = Point::new(
                                    tx_tree as i32 - sprite_32.width() as i32 / 2,
                                    ty_tree as i32 - sprite_32.height() as i32 / 2,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &oak_texture,
                                    position,
                                    sprite_32,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::AppleTree {
                                let position = Point::new(
                                    tx_tree as i32 - sprite_32.width() as i32 / 2,
                                    ty_tree as i32 - sprite_32.height() as i32 / 2,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &appletree_texture,
                                    position,
                                    sprite_32,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::Spruce {
                                let position = Point::new(
                                    tx_tree as i32 - sprite_32.width() as i32 / 2,
                                    ty_tree as i32 - sprite_32.height() as i32 / 2,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &spruce_texture,
                                    position,
                                    sprite_32,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::Pine {
                                let position = Point::new(
                                    tx_tree as i32 - sprite_32.width() as i32 / 2,
                                    ty_tree as i32 - sprite_32.height() as i32 / 2,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &pine_texture,
                                    position,
                                    sprite_32,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::Birch {
                                let position = Point::new(
                                    tx_tree as i32 - sprite_32.width() as i32 / 2,
                                    ty_tree as i32 - sprite_32.height() as i32 / 2,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &birch_texture,
                                    position,
                                    sprite_32,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            }
                            // vegetation
                            else if entity.entity_type == EntityType::Cactus {
                                let position = Point::new(
                                    tx_tree as i32 - sprite_32.width() as i32 / 2,
                                    ty_tree as i32 - sprite_32.height() as i32 / 2,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &cactus_texture,
                                    position,
                                    sprite_32,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            }
                            // ants and other lifeforms
                            else if entity.entity_type == EntityType::WorkerAnt {
                                let position = Point::new(
                                    tx_ant as i32 - sprite_16.width() as i32 / 2,
                                    ty_ant as i32 - sprite_16.height() as i32 / 2,
                                );
                                let mut tex = &ant_worker_texture_1;
                                if entity.current_action != ActionType::Idle
                                    && entity.time / (DRONE_ANIMATION_SPEED) % 2 == 0
                                {
                                    tex = &ant_worker_texture_2;
                                }
                                graphics_utils::render(
                                    &mut canvas,
                                    tex,
                                    position,
                                    sprite_16,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::SoldierAnt {
                                let position = Point::new(
                                    tx_ant as i32 - sprite_16.width() as i32 / 2,
                                    ty_ant as i32 - sprite_16.height() as i32 / 2,
                                );
                                let mut tex = &ant_soldier_texture_1;
                                if entity.current_action != ActionType::Idle
                                    && entity.time / (SOLDIER_ANIMATION_SPEED * 10) % 2 == 0
                                {
                                    tex = &ant_soldier_texture_2;
                                }
                                graphics_utils::render(
                                    &mut canvas,
                                    &tex,
                                    position,
                                    sprite_16,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::DroneAnt {
                                let position = Point::new(
                                    tx_ant as i32 - sprite_16.width() as i32 / 2,
                                    ty_ant as i32 - sprite_16.height() as i32 / 2,
                                );
                                let mut tex = &ant_drone_texture_1;
                                if entity.current_action != ActionType::Idle
                                    && entity.time / (DRONE_ANIMATION_SPEED) % 2 == 0
                                {
                                    tex = &ant_drone_texture_2;
                                }
                                graphics_utils::render(
                                    &mut canvas,
                                    &tex,
                                    position,
                                    sprite_16,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::Mechant {
                                let position = Point::new(
                                    tx_ant as i32 - sprite_16.width() as i32 / 2,
                                    ty_ant as i32 - sprite_16.height() as i32 / 2,
                                );
                                let mut tex = &mechant_texture_1;
                                if entity.current_action != ActionType::Idle
                                    && entity.time / (MECHANT_ANIMATION_SPEED) % 2 == 0
                                {
                                    tex = &mechant_texture_2;
                                }
                                graphics_utils::render(
                                    &mut canvas,
                                    &tex,
                                    position,
                                    sprite_16,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::QueenAnt {
                                let position = Point::new(
                                    tx_ant as i32 - sprite_32.width() as i32 / 2,
                                    ty_ant as i32 - sprite_32.height() as i32 / 2,
                                );
                                let mut tex = &ant_queen_texture_1;
                                if entity.current_action != ActionType::Idle
                                    && time / (QUEEN_ANIMATION_SPEED + rng.gen_range(1..2) * 10) % 2
                                        == 0
                                {
                                    tex = &ant_queen_texture_2;
                                }
                                graphics_utils::render(
                                    &mut canvas,
                                    &tex,
                                    position,
                                    sprite_32,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::FoodStorage {
                                let position = Point::new(
                                    tx_ant as i32 - sprite_16.width() as i32 / 2,
                                    ty_ant as i32 - sprite_16.height() as i32 / 2,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &food_storage_texture,
                                    position,
                                    sprite_16,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            } else if entity.entity_type == EntityType::AntEgg {
                                let position = Point::new(
                                    tx_ant as i32 - sprite_8.width() as i32 / 2,
                                    ty_ant as i32 - sprite_8.height() as i32 / 2,
                                );
                                let mut tex = &ant_egg_texture;
                                if entity.time > (HATCH_TIME as f32 * (1.0 / 4.0)) as u128 {
                                    tex = &ant_egg_texture_2;
                                }
                                if entity.time > (HATCH_TIME as f32 * (2.0 / 4.0)) as u128 {
                                    tex = &ant_egg_texture_3;
                                }
                                if entity.time > (HATCH_TIME as f32 * (3.0 / 4.0)) as u128 {
                                    tex = &ant_egg_texture_4;
                                }
                                graphics_utils::render(
                                    &mut canvas,
                                    &tex,
                                    position,
                                    sprite_8,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            }

                            if entity.backpack_item == ItemType::Fruit {
                                let item_position = Point::new(
                                    tx_ant as i32 - sprite_4.width() as i32 / 2 + 4,
                                    ty_ant as i32 - sprite_4.height() as i32 / 2 + 4,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &fruit_texture,
                                    item_position,
                                    sprite_4,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            }
                            if entity.wielding_item == ItemType::WoodenSpear {
                                let item_position = Point::new(
                                    tx_ant as i32 - sprite_1x10.width() as i32 / 2 + 7,
                                    ty_ant as i32 - sprite_1x10.height() as i32 / 2 - 1,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &wooden_spear_texture,
                                    item_position,
                                    sprite_1x10,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            }
                            if entity.wielding_item == ItemType::WoodenShovel {
                                let item_position = Point::new(
                                    tx_ant as i32 - sprite_2x5.width() as i32 / 2 + 7,
                                    ty_ant as i32 - sprite_2x5.height() as i32 / 2 + 2,
                                );
                                graphics_utils::render(
                                    &mut canvas,
                                    &wooden_shovel_texture,
                                    item_position,
                                    sprite_2x5,
                                    camera.zoom,
                                    ratio_x,
                                    ratio_y,
                                );
                            }
                        }
                    }
                }

                // render player

                let mut player_tex = &cultist_ant_texture_1;

                if player.stopped && (player.time / 100) % 2 == 0 {
                    player_tex = &cultist_ant_texture_2;
                }

                let player_position = Point::new(
                    (player.x * camera.zoom - camera.x) as i32,
                    (player.y * camera.zoom - camera.y) as i32,
                );
                graphics_utils::render(
                    &mut canvas,
                    &cultist_ant_texture_1,
                    player_position,
                    sprite_16,
                    camera.zoom,
                    ratio_x,
                    ratio_y,
                );

                // render hover
                let mut hovered_tiletype = TileType::Grass;
                let mut hovered_tile: std::option::Option<crate::world_structs::Point> = None;
                let mut hovered_entity: std::option::Option<Entity> = None;
                let mut hovering_entity = false;
                if mouse_not_moved_for > hover_time {
                    let e_x = (camera.x / ratio_x + mouse_state.x() as f32) * ratio_x;
                    let e_y = (camera.y / ratio_y + mouse_state.y() as f32) * ratio_y;
                    for i in 0..chunks.len() {
                        for j in 0..chunks[i].len() {
                            for e in chunks[i][j].entities.values() {
                                if e_x > e.x && e_x < e.x + 16.0 && e_y > e.y && e_y < e.y + 16.0 {
                                    hovering_entity = true;
                                    hovered_entity = Some(e.clone());
                                    ()
                                }
                            }
                        }
                    }
                    let mouse_x_unscaled = (camera.x / ratio_x + mouse_state.x() as f32) * ratio_x;
                    let mouse_y_unscaled = (camera.y / ratio_y + mouse_state.y() as f32) * ratio_y;
                    let tile_x = (((mouse_x_unscaled) / TILE_SIZE) as f32).floor();
                    let tile_y = (((mouse_y_unscaled) / TILE_SIZE) as f32).floor();
                    for i in 0..chunks.len() {
                        for j in 0..chunks[i].len() {
                            for row in &chunks[i][j].points {
                                for p in row {
                                    if tile_x == p.x && tile_y == p.y {
                                        hovered_tile = Some(p.clone());
                                    }
                                }
                            }
                        }
                    }
                }
                if (!hovering_entity) {
                    match hovered_tile {
                        Some(ht) => {
                            let text = graphics_utils::get_text(
                                descriptions_for_tiles
                                    .get(&ht.tile_type)
                                    .unwrap()
                                    .to_string(),
                                Color::RGBA(55, 185, 90, 255),
                                desc_font_size,
                                &font,
                                &texture_creator,
                            )
                            .unwrap();
                            let position = Point::new(
                                ((mouse_state.x() as f32 * ratio_x
                                    - text.text_sprite.width() as f32 / 2.0)
                                    as i32),
                                ((mouse_state.y() as f32 * ratio_y
                                    - (text.text_sprite.height()) as f32)
                                    as i32),
                            );
                            graphics_utils::render_text(
                                &mut canvas,
                                &text.text_texture,
                                position,
                                text.text_sprite,
                                ratio_x,
                                ratio_y,
                            );
                        }
                        None => (),
                    }
                } else {
                    match hovered_entity {
                        Some(he) => {
                            let mut name = descriptions_for_entities.get(&he.entity_type).unwrap();
                            let mut title = "".to_string();
                            if he.category_type == CategoryType::Ant {
                                title = he.faction;
                                title.push_str("ese ");
                            }

                            title.push_str(name);
                            let text = graphics_utils::get_text(
                                title,
                                Color::RGBA(55, 185, 90, 255),
                                desc_font_size,
                                &font,
                                &texture_creator,
                            )
                            .unwrap();

                            let position = Point::new(
                                (mouse_state.x() as f32 * ratio_x
                                    - text.text_sprite.width() as f32 / 2.0)
                                    as i32,
                                ((mouse_state.y() as f32 * ratio_y
                                    - (text.text_sprite.height()) as f32)
                                    as i32),
                            );
                            graphics_utils::render_text(
                                &mut canvas,
                                &text.text_texture,
                                position,
                                text.text_sprite,
                                ratio_x,
                                ratio_y,
                            );
                        }

                        None => (),
                    }
                }

                // render overlays

                if map_state == graphics_utils::MapState::Political {
                    for i in 0..chunks.len() {
                        for j in 0..chunks[i].len() {
                            let position = Point::new(
                                ((world_data.tile_size as f32
                                    * chunks[i][j].points[0][0].x
                                    * camera.zoom
                                    - camera.x)
                                    / ratio_x) as i32,
                                ((world_data.tile_size as f32
                                    * (chunks[i][j].points[0][0].y * camera.zoom)
                                    - camera.y)
                                    / ratio_y) as i32,
                            );
                            let render_rect = Rect::new(
                                (position.x as f32) as i32,
                                (position.y as f32) as i32,
                                (world_data.chunk_size as i32
                                    * (world_data.tile_size as f32 / ratio_x as f32) as i32)
                                    as u32,
                                (world_data.chunk_size as i32
                                    * (world_data.tile_size as f32 / ratio_y as f32) as i32)
                                    as u32,
                            );
                            match chunk_graphics_data.get(&chunks[i][j].name) {
                                Some(cgd) => {
                                    if chunks[i][j].name == "Neutral" {
                                        graphics_utils::render_rect(
                                            &mut canvas,
                                            position,
                                            render_rect,
                                            Color::RGBA(255, 255, 255, 55),
                                            camera.zoom,
                                        );
                                    } else {
                                        graphics_utils::render_rect(
                                            &mut canvas,
                                            position,
                                            render_rect,
                                            *chunk_graphics_data.get(&chunks[i][j].name).unwrap(),
                                            camera.zoom,
                                        );
                                    }
                                }
                                None => {
                                    graphics_utils::render_rect(
                                        &mut canvas,
                                        position,
                                        render_rect,
                                        Color::RGBA(255, 255, 255, 125),
                                        camera.zoom,
                                    );
                                }
                            }
                            // render chunk faction description
                            let title = chunks[i][j].name.clone();
                            let text = graphics_utils::get_text(
                                title.clone(),
                                Color::RGBA(55, 185, 90, 255),
                                desc_font_size,
                                &font,
                                &texture_creator,
                            )
                            .unwrap();

                            let text_position = Point::new(
                                (((position.x()
                                    + (world_data.chunk_size as f32
                                        * world_data.tile_size as f32
                                        * camera.zoom) as i32
                                        / 2
                                    - title.clone().len() as i32 * desc_font_size as i32 / 4)
                                    as f32)
                                    * ratio_x) as i32,
                                (position.y() as f32 * ratio_y) as i32
                                    + (((world_data.chunk_size as f32
                                        * world_data.tile_size as f32
                                        * camera.zoom)
                                        as i32
                                        / 2) as f32
                                        / 1.0) as i32,
                            );
                            graphics_utils::render_text(
                                &mut canvas,
                                &text.text_texture,
                                text_position,
                                text.text_sprite,
                                ratio_x,
                                ratio_y,
                            );
                        }
                    }
                }
                // religion map
                if map_state == graphics_utils::MapState::Religion {
                    for i in 0..chunks.len() {
                        for j in 0..chunks[i].len() {
                            let position = Point::new(
                                ((world_data.tile_size as f32
                                    * chunks[i][j].points[0][0].x
                                    * camera.zoom
                                    - camera.x)
                                    / ratio_x) as i32,
                                ((world_data.tile_size as f32
                                    * (chunks[i][j].points[0][0].y * camera.zoom)
                                    - camera.y)
                                    / ratio_y) as i32,
                            );
                            let render_rect = Rect::new(
                                (position.x as f32) as i32,
                                (position.y as f32) as i32,
                                (world_data.chunk_size as i32
                                    * (world_data.tile_size as f32 / ratio_x as f32) as i32)
                                    as u32,
                                (world_data.chunk_size as i32
                                    * (world_data.tile_size as f32 / ratio_y as f32) as i32)
                                    as u32,
                            );
                            match religion_graphics_data.get(&chunks[i][j].religion) {
                                Some(cgd) => {
                                    if chunks[i][j].religion == ReligionType::Nothing {
                                        graphics_utils::render_rect(
                                            &mut canvas,
                                            position,
                                            render_rect,
                                            *religion_graphics_data
                                                .get(&chunks[i][j].religion)
                                                .unwrap(),
                                            camera.zoom,
                                        );
                                    } else {
                                        graphics_utils::render_rect(
                                            &mut canvas,
                                            position,
                                            render_rect,
                                            *religion_graphics_data
                                                .get(&chunks[i][j].religion)
                                                .unwrap(),
                                            camera.zoom,
                                        );
                                    }
                                }
                                None => {
                                    graphics_utils::render_rect(
                                        &mut canvas,
                                        position,
                                        render_rect,
                                        Color::RGBA(255, 255, 255, 125),
                                        camera.zoom,
                                    );
                                }
                            }
                            // render chunk faction description
                            let title = descriptions_for_religions
                                .get(&chunks[i][j].religion)
                                .unwrap()
                                .to_string();
                            let text = graphics_utils::get_text(
                                title.clone(),
                                Color::RGBA(55, 185, 90, 255),
                                desc_font_size,
                                &font,
                                &texture_creator,
                            )
                            .unwrap();

                            let text_position = Point::new(
                                (((position.x()
                                    + (world_data.chunk_size as f32
                                        * world_data.tile_size as f32
                                        * camera.zoom) as i32
                                        / 2
                                    - title.clone().len() as i32 * desc_font_size as i32 / 4)
                                    as f32)
                                    * ratio_x) as i32,
                                (position.y() as f32 * ratio_y) as i32
                                    + (((world_data.chunk_size as f32
                                        * world_data.tile_size as f32
                                        * camera.zoom)
                                        as i32
                                        / 2) as f32
                                        / 1.0) as i32,
                            );
                            graphics_utils::render_text(
                                &mut canvas,
                                &text.text_texture,
                                text_position,
                                text.text_sprite,
                                ratio_x,
                                ratio_y,
                            );
                        }
                    }
                }
                // render ui

                let position = Point::new(0 as i32, 192 as i32);
                graphics_utils::render(
                    &mut canvas,
                    &hud_texture,
                    position,
                    sprite_426x48,
                    1.0,
                    ratio_x,
                    ratio_y,
                );

                let hp_text = graphics_utils::get_text(
                    "LIFE: ".to_string(),
                    Color::RGBA(255, 255, 255, 255),
                    hp_font_size,
                    &hp_font,
                    &texture_creator,
                )
                .unwrap();
                let position = Point::new(
                    (SCREEN_WIDTH as f32 - 116.0) as i32,
                    (SCREEN_HEIGHT as f32 - 46.0) as i32,
                );
                graphics_utils::render_text(
                    &mut canvas,
                    &hp_text.text_texture,
                    position,
                    hp_text.text_sprite,
                    ratio_x,
                    ratio_y,
                );
                let magic_text = graphics_utils::get_text(
                    "ENERGY: ".to_string(),
                    Color::RGBA(255, 255, 255, 255),
                    hp_font_size,
                    &hp_font,
                    &texture_creator,
                )
                .unwrap();
                let position = Point::new(
                    (SCREEN_WIDTH - 116).try_into().unwrap(),
                    (SCREEN_HEIGHT - 36).try_into().unwrap(),
                );
                graphics_utils::render_text(
                    &mut canvas,
                    &magic_text.text_texture,
                    position,
                    magic_text.text_sprite,
                    ratio_x,
                    ratio_y,
                );

                let position = Point::new(
                    ((SCREEN_WIDTH - 78) as f32 / ratio_x) as i32,
                    ((SCREEN_HEIGHT - 44) as f32 / ratio_y) as i32,
                );
                let render_rect = Rect::new(
                    (position.x as f32) as i32,
                    (position.y as f32) as i32,
                    ((1.0.lerp(64.0, player.hp as f32 / 100.0)) / ratio_x) as u32,
                    (8.0 / ratio_y) as u32,
                );

                graphics_utils::render_rect(
                    &mut canvas,
                    position,
                    render_rect,
                    Color::RGBA(255, 0, 0, 55),
                    1.0,
                );
                let position = Point::new(
                    ((SCREEN_WIDTH - 78) as f32 / ratio_x) as i32,
                    ((SCREEN_HEIGHT - 34) as f32 / ratio_y) as i32,
                );
                let render_rect = Rect::new(
                    (position.x as f32) as i32,
                    (position.y as f32) as i32,
                    ((1.0.lerp(64.0, player.hp as f32 / 100.0)) / ratio_x) as u32,
                    (8.0 / ratio_y) as u32,
                );

                graphics_utils::render_rect(
                    &mut canvas,
                    position,
                    render_rect,
                    Color::RGBA(0, 255, 100, 55),
                    1.0,
                );
                // political map button
                let position = Point::new(political_button.x as i32, political_button.y as i32);
                political_button.check_if_hovered(
                    mouse_state.x() as f32 * ratio_x,
                    mouse_state.y() as f32 * ratio_y,
                    ratio_x,
                    ratio_y,
                );
                political_button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
                if political_button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_hovered_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if political_button.status == graphics_utils::ButtonStatus::Pressed {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_pressed_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                }
                // religion_button
                let position = Point::new(religion_button.x as i32, religion_button.y as i32);
                religion_button.check_if_hovered(
                    mouse_state.x() as f32 * ratio_x,
                    mouse_state.y() as f32 * ratio_y,
                    ratio_x,
                    ratio_y,
                );
                religion_button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
                if religion_button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_hovered_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if religion_button.status == graphics_utils::ButtonStatus::Pressed {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_pressed_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                }

                // normal map button
                let position = Point::new(normal_button.x as i32, normal_button.y as i32);
                normal_button.check_if_hovered(
                    mouse_state.x() as f32 * ratio_x,
                    mouse_state.y() as f32 * ratio_y,
                    ratio_x,
                    ratio_y,
                );
                normal_button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
                if normal_button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_hovered_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if normal_button.status == graphics_utils::ButtonStatus::Pressed {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_pressed_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                }
                let normal_text_margin = 4;
                let normal_text = graphics_utils::get_text(
                    "N".to_string(),
                    Color::RGBA(255, 255, 255, 255),
                    desc_font_size,
                    &font,
                    &texture_creator,
                )
                .unwrap();
                let position = Point::new(
                    normal_button.x as i32 + 8 + normal_text_margin,
                    normal_button.y as i32 + normal_text_margin,
                );
                graphics_utils::render_text(
                    &mut canvas,
                    &normal_text.text_texture,
                    position,
                    normal_text.text_sprite,
                    ratio_x,
                    ratio_y,
                );
                let political_text_margin = 4;
                let political_text = graphics_utils::get_text(
                    "P".to_string(),
                    Color::RGBA(255, 255, 255, 255),
                    desc_font_size,
                    &font,
                    &texture_creator,
                )
                .unwrap();
                let position = Point::new(
                    political_button.x as i32 + 8 + political_text_margin,
                    political_button.y as i32 + political_text_margin,
                );
                graphics_utils::render_text(
                    &mut canvas,
                    &political_text.text_texture,
                    position,
                    political_text.text_sprite,
                    ratio_x,
                    ratio_y,
                );

                let religion_text_margin = 4;
                let religion_text = graphics_utils::get_text(
                    "R".to_string(),
                    Color::RGBA(255, 255, 255, 255),
                    desc_font_size,
                    &font,
                    &texture_creator,
                )
                .unwrap();
                let position = Point::new(
                    religion_button.x as i32 + 8 + religion_text_margin,
                    religion_button.y as i32 + religion_text_margin,
                );
                graphics_utils::render_text(
                    &mut canvas,
                    &religion_text.text_texture,
                    position,
                    religion_text.text_sprite,
                    ratio_x,
                    ratio_y,
                );
            }
        }
        if normal_button.status == graphics_utils::ButtonStatus::Pressed {
            map_state = graphics_utils::MapState::Normal;
        } else if political_button.status == graphics_utils::ButtonStatus::Pressed {
            map_state = graphics_utils::MapState::Political;
        } else if religion_button.status == graphics_utils::ButtonStatus::Pressed {
            map_state = graphics_utils::MapState::Religion;
        }
        canvas.present();
        thread::sleep(time::Duration::from_millis(10));
    }

    println!("Socket connection ended.");
    Ok(())
}
pub fn run() {
    main_loop();
}
