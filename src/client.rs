extern crate websocket;
use crate::graphics_utils;
use std::sync::{Arc, Mutex};

use bincode;
extern crate ears;
use crate::client::ears::AudioTags;
use crate::client_structs;
use crate::client_structs::{ClientPacket, Player, PlayerAction, ShootData};
use crate::graphics_utils::{Button, ButtonStatus, Camera, MoveDirection};
use crate::world_structs::{
    ActionType, CategoryType, Chunk, Collider, ColliderType, Entity, EntityType, ItemType, Prop,
    PropType, ReligionType, TaskType, TileType, World, WorldData, HATCH_TIME,
};
use ears::{AudioController, Music, Sound};
use lerp::Lerp;
use rand::Rng;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadSurface, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag as AudioInitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::mouse::MouseState;
use sdl2::mouse::MouseWheelDirection;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::FullscreenType;
use sdl2::Sdl;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::future::Future;
use std::io::stdin;
use std::io::{Read, Write};
use std::iter::FromIterator;
use std::option::Option;
use std::path::Path;
use std::pin::Pin;
use std::str::from_utf8;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};
const ENTITY_SIZE: f32 = 8.0;
const SCREEN_WIDTH: u32 = 426;
const SCREEN_HEIGHT: u32 = 240;
const HUD_LOC: u32 = 336;
const MAP_UI_LOC: u32 = 336;

const TILE_SIZE: f32 = 16.0;
const WORKER_ANIMATION_SPEED: u128 = 25;
const DRONE_ANIMATION_SPEED: u128 = 25;
const QUEEN_ANIMATION_SPEED: u128 = 25;
const SOLDIER_ANIMATION_SPEED: u128 = 25;
const MECHANT_ANIMATION_SPEED: u128 = 25;
const PLAYER_ANIMATION_SPEED: u128 = 100;
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
        .window("Tales of Terrant", SCREEN_WIDTH, SCREEN_HEIGHT)
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
    // audio
    let music_path_1 = "music/tribal_hero.flac";
    let music_path_2 = "music/sundown_of_terrant.flac";
    let footstep_path = "sound/footstep.flac";
    let start_fanfare_path = "sound/start_fanfare.flac";
    let button_click_path = "sound/button_click.flac";
    let meteoroid_spawn_path = "sound/meteoroid_spawn.flac";
    let meteoroid_explode_path = "sound/meteoroid_explode.flac";
    let soul_trapped_path = "sound/soul_trapped.flac";
    let soul_trap_build_path = "sound/soul_trap_build.flac";
    let raft_build_path = "sound/raft_build.flac";
    let menu_next_path = "sound/menu_next.flac";
    let mut songs = vec![
        Music::new(music_path_1).unwrap(),
        Music::new(music_path_2).unwrap(),
    ];
    let mut sounds_volume = 0.7;
    let mut song_volume = 0.0;
    let mut button_click = Sound::new(button_click_path).unwrap();
    button_click.set_volume(sounds_volume);
    let mut start_fanfare = Sound::new(start_fanfare_path).unwrap();
    let mut menu_next = Sound::new(menu_next_path).unwrap();
    let mut meteoroid_spawn = Sound::new(meteoroid_spawn_path).unwrap();
    let mut meteoroid_explode = Sound::new(meteoroid_explode_path).unwrap();
    let mut soul_trapped = Sound::new(soul_trapped_path).unwrap();
    let mut soul_trap_build = Sound::new(soul_trap_build_path).unwrap();
    let mut raft_build = Sound::new(raft_build_path).unwrap();
    let mut player_footstep = Sound::new(footstep_path).unwrap();
    player_footstep.set_volume(sounds_volume);
    let songs_len = songs.len();
    let mut current_song = &mut songs[0];
    let wiki_text_paths = [
        "text/terrant.md",
        "text/desert.md",
        "text/forest.md",
        "text/grasslands.md",
        "text/mediterraean.md",
        "text/red_desert.md",
        "text/savannah.md",
        "text/taiga.md",
        "text/tundra.md",
        "text/ants.md",
    ];
    let mut wiki_index = 0;
    let mut wiki_texts = vec![];
    let mut wiki_text_contents: Vec<Vec<String>> = vec![];

    for path in wiki_text_paths.iter() {
        wiki_text_contents.push(vec![fs::read_to_string(path).unwrap()]);
    }
    for vector in wiki_text_contents.iter() {
        let mut wiki_text_lines: Vec<&str> = vec![];
        for content in vector.iter() {
            let split_string: Vec<&str> = content.lines().collect();
            for line in split_string {
                wiki_text_lines.push(line);
            }
            wiki_texts.push(wiki_text_lines.clone());
        }
    }
    let mut already_collided_to_entity = HashMap::new();
    //canvas.window_mut().set_fullscreen(FullscreenType::True);

    // canvas.window_mut().set_size(500, 500);
    // canvas.window_mut().set_resizable(true);
    // texture stuff
    let texture_creator = canvas.texture_creator();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    // font stuff
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let desc_font_size = 20;
    let main_title_font_size = 17;
    let mut main_title_font =
        ttf_context.load_font("fonts/PixelOperator.ttf", main_title_font_size)?;
    let mut font = ttf_context.load_font("fonts/PixelOperator.ttf", desc_font_size)?;

    let hp_font_size = 10;
    let mut hp_font = ttf_context.load_font("fonts/PixelOperator.ttf", hp_font_size)?;
    let wiki_text_font_size = 12;
    let mut wiki_text_font =
        ttf_context.load_font("fonts/PixelOperator.ttf", wiki_text_font_size)?;
    let wiki_h1_font_size = 20;
    let mut wiki_h1_font = ttf_context.load_font("fonts/PixelOperator.ttf", wiki_h1_font_size)?;
    let wiki_h2_font_size = 16;
    let mut wiki_h2_font = ttf_context.load_font("fonts/PixelOperator.ttf", wiki_h2_font_size)?;
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
    let mut client_time: u128 = 0;
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
    let mut green_flashing = false;
    let green_flashing_time = 2000;
    let mut green_flashing_change = 0;
    let mut entities: HashMap<i32, Entity> = HashMap::new();
    let mut colliders: Vec<Collider> = Vec::new();
    let mut props: Vec<Prop> = Vec::new();
    let mut players: Vec<Player> = Vec::new();
    let mut settings_buttons = vec![Button {
        status: graphics_utils::ButtonStatus::Hovered, // play button
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: SCREEN_WIDTH as f32 - 148.0 - 8.0,
        y: (SCREEN_HEIGHT as f32 - 42.0 - 8.0) as f32,
        width: 128.0,
        height: 32.0,
    }];
    let mut settings_action_buttons = vec![
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // play button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: 16.0,
            y: 16.0,
            width: 128.0,
            height: 32.0,
        },
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // play button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: 16.0,
            y: 60.0,
            width: 128.0,
            height: 32.0,
        },
    ];
    let mut manual_buttons = vec![Button {
        status: graphics_utils::ButtonStatus::Hovered, // play button
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: SCREEN_WIDTH as f32 - 148.0 - 8.0,
        y: (SCREEN_HEIGHT as f32 - 42.0 - 8.0) as f32,
        width: 128.0,
        height: 32.0,
    }];
    let mut wiki_buttons = vec![
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // play button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: 24.0,
            y: (SCREEN_HEIGHT as f32 - 48.0) as f32,
            width: 32.0,
            height: 32.0,
        },
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // play button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: 64.0,
            y: (SCREEN_HEIGHT as f32 - 48.0) as f32,
            width: 32.0,
            height: 32.0,
        },
    ];

    let mut menu_buttons: Vec<Button> = vec![
        // menu buttons
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // play button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
            y: 62.0,
            width: 128.0,
            height: 32.0,
        },
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // settings button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
            y: 62.0 + 32.0 + 8.0,
            width: 128.0,
            height: 32.0,
        },
        Button {
            status: graphics_utils::ButtonStatus::Hovered, //  manual button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
            y: 62.0 + 64.0 + 16.0,
            width: 128.0,
            height: 32.0,
        },
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // exit
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: SCREEN_WIDTH as f32 / 2.0 - 64.0,
            y: 62.0 + 96.0 + 24.0,
            width: 128.0,
            height: 32.0,
        },
    ];
    let mut action_icon_buttons: Vec<Button> = vec![
        Button {
            status: graphics_utils::ButtonStatus::Hovered, //
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: 4.0,
            y: SCREEN_HEIGHT as f32 - 44.0,
            width: 11.0,
            height: 11.0,
        },
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // play button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: 4.0,
            y: SCREEN_HEIGHT as f32 - 28.0,
            width: 11.0,
            height: 11.0,
        },
        Button {
            status: graphics_utils::ButtonStatus::Hovered, // play button
            previous_status: graphics_utils::ButtonStatus::Hovered,
            x: 20.0,
            y: SCREEN_HEIGHT as f32 - 44.0,
            width: 11.0,
            height: 11.0,
        },
    ];
    // universal menu buttons
    // settings menu buttons
    // manual menu buttons
    // ui buttons
    let mut normal_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: 4 as f32,
        y: (SCREEN_HEIGHT - 22 - 8 - 128) as f32,
        width: 32.0,
        height: 32.0,
    };
    let mut political_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: 4.0,
        y: (SCREEN_HEIGHT - 22 - 8 - 92) as f32,
        width: 32.0,
        height: 32.0,
    };

    let mut religion_button = graphics_utils::Button {
        status: graphics_utils::ButtonStatus::Hovered,
        previous_status: graphics_utils::ButtonStatus::Hovered,
        x: 4.0,
        y: (SCREEN_HEIGHT - 22 - 64) as f32,
        width: 32.0,
        height: 32.0,
    };
    // collider textures

    let meteoroid_texture = texture_creator.load_texture("res/meteoroid.png")?;
    let soul_trap_texture = texture_creator.load_texture("res/soul_trap.png")?;
    // prop textures
    let raft_texture = texture_creator.load_texture("res/raft.png")?;
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
    let ant_worker_texture_front_1 = texture_creator.load_texture("res/ant_worker_front_1.png")?;
    let ant_worker_texture_front_2 = texture_creator.load_texture("res/ant_worker_front_2.png")?;
    let ant_worker_texture_back_1 = texture_creator.load_texture("res/ant_worker_back_1.png")?;
    let ant_worker_texture_back_2 = texture_creator.load_texture("res/ant_worker_back_2.png")?;
    let ant_worker_texture_side_1 = texture_creator.load_texture("res/ant_worker_side_1.png")?;
    let ant_worker_texture_side_2 = texture_creator.load_texture("res/ant_worker_side_2.png")?;
    let ant_worker_texture_side_mirror_1 =
        texture_creator.load_texture("res/ant_worker_side_mirror_1.png")?;
    let ant_worker_texture_side_mirror_2 =
        texture_creator.load_texture("res/ant_worker_side_mirror_2.png")?;
    let plasmant_texture_front_1 = texture_creator.load_texture("res/plasmant_front_1.png")?;
    let plasmant_texture_front_2 = texture_creator.load_texture("res/plasmant_front_2.png")?;
    let plasmant_texture_back_1 = texture_creator.load_texture("res/plasmant_back_1.png")?;
    let plasmant_texture_back_2 = texture_creator.load_texture("res/plasmant_back_2.png")?;
    let plasmant_texture_side_1 = texture_creator.load_texture("res/plasmant_side_1.png")?;
    let plasmant_texture_side_2 = texture_creator.load_texture("res/plasmant_side_2.png")?;
    let plasmant_texture_side_mirror_1 =
        texture_creator.load_texture("res/plasmant_side_mirror_1.png")?;
    let plasmant_texture_side_mirror_2 =
        texture_creator.load_texture("res/plasmant_side_mirror_2.png")?;
    let ant_soldier_texture_1 = texture_creator.load_texture("res/ant_worker_side_1.png")?;
    let ant_soldier_texture_2 = texture_creator.load_texture("res/ant_worker_side_2.png")?;
    let ant_drone_texture_1 = texture_creator.load_texture("res/ant_drone.png")?;
    let ant_drone_texture_2 = texture_creator.load_texture("res/ant_drone_2.png")?;
    let mechant_texture_1 = texture_creator.load_texture("res/mechant.png")?;
    let mechant_texture_2 = texture_creator.load_texture("res/mechant.png")?;
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
    let mut wiki_text_background = texture_creator.load_texture("res/wiki_text_background.png")?;

    // ui textures

    let mut ui_button_texture = texture_creator.load_texture("res/ui_button.png")?;
    let mut ui_button_hovered_texture =
        texture_creator.load_texture("res/ui_button_hovered.png")?;
    let mut ui_button_pressed_texture =
        texture_creator.load_texture("res/ui_button_pressed.png")?;

    let mut action_icon_button_texture =
        texture_creator.load_texture("res/action_icon_button.png")?;
    let mut action_icon_button_hovered_texture =
        texture_creator.load_texture("res/action_icon_button_hovered.png")?;
    let mut action_icon_button_pressed_texture =
        texture_creator.load_texture("res/action_icon_button_pressed.png")?;
    let mut raft_icon_texture = texture_creator.load_texture("res/raft_icon.png")?;
    let mut meteoroid_icon_texture = texture_creator.load_texture("res/meteoroid_icon.png")?;
    let mut siphon_icon_texture = texture_creator.load_texture("res/siphon_icon.png")?;

    // hud textures
    let mut hud_texture = texture_creator.load_texture("res/hud.png")?;
    let mut map_ui_texture = texture_creator.load_texture("res/map_ui.png")?;
    // other texture stuff
    let mut banner_texture = texture_creator.load_texture("res/banner.png")?;
    // effects
    let mut green_flash_texture = texture_creator.load_texture("res/green_flash.png")?;
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
    let sprite_64x112 = Rect::new(0, 0, (48.0) as u32, (112.0) as u32);
    let sprite_158x212 = Rect::new(0, 0, (158.0) as u32, (212.0) as u32);
    let sprite_2x5 = Rect::new(0, 0, (2.0 * camera.zoom) as u32, (5.0 * camera.zoom) as u32);
    let sprite_8 = Rect::new(0, 0, (8.0 * camera.zoom) as u32, (8.0 * camera.zoom) as u32);
    let sprite_12 = Rect::new(
        0,
        0,
        (12.0 * camera.zoom) as u32,
        (12.0 * camera.zoom) as u32,
    );
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
    let sprite_698x212 = Rect::new(0, 0, (392) as u32, 212 as u32);
    let sprite_720x480 = Rect::new(0, 0, 720.0 as u32, 480.0 as u32);
    let sprite_426x240 = Rect::new(0, 0, 426.0 as u32, 240.0 as u32);

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
        faction: "The Fringe".to_string(),
        faction_id: 0,
        backpack_amount: 0,
        time: 0,
        shoot_change_1: 0,
        shoot_data: ShootData {
            mx: 0,
            my: 0,
            shooting: false,
            action_type: PlayerAction::Meteoroid,
        },
    };
    let mut up_collision = false;
    let mut down_collision = false;
    let mut left_collision = false;
    let mut right_collision = false;
    let mut player_action = PlayerAction::Nothing;
    let mut map_state = graphics_utils::MapState::Normal;
    let mut main_menu_on = true;
    let mut banner_on = true;
    let mut settings_menu_on = false;
    let mut manual_menu_on = false;
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
    start_fanfare.play();
    while running {
        if current_song.get_state() == ears::State::Stopped {
            current_song = &mut songs[rng.gen_range(0..songs_len)];
            current_song.set_volume(song_volume);
            current_song.play();
        }
        let delta = SystemTime::now().duration_since(compare_time).unwrap();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        compare_time = SystemTime::now();

        let delta_as_millis = delta.as_millis();
        if delta.as_millis() / 10 != 0 {
            //println!("FPS: {}", 100 / (delta.as_millis() / 10));
            //println!("{}", delta_as_millis);
        }
        client_time += delta_as_millis;
        mouse_not_moved_for += delta_as_millis;
        canvas.set_draw_color(bg_color);
        canvas.clear();
        // canvas.fill_rect(Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT));
        // send message to server
        //
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    main_menu_on = true;
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
                    //zoom_button_plus = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    //zoom_button_minus = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    menu_next.play();
                    banner_on = false;
                }
                Event::MouseWheel { x, y, .. } => {
                    if y > 0 {
                        //   zoom_button_minus = true;
                    } else if y < 0 {
                        //   zoom_button_plus = true;
                    }
                }
                Event::MouseMotion { .. } => {
                    mouse_not_moved_for = 0;
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Middle {
                        if player_action.clone() == PlayerAction::Meteoroid {
                            player.shoot_meteoroid(x, y);
                        } else if player_action.clone() == PlayerAction::Raft {
                            // player.build_bridge
                            player.build_raft(x, y);
                        } else if player_action.clone() == PlayerAction::Siphon {
                            // player.build_bridge
                            player.build_soul_trap(x, y);
                        }
                    } else if mouse_btn == sdl2::mouse::MouseButton::Right {
                    }
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
        let mouse_x_unscaled = (camera.x / ratio_x + mouse_state.x() as f32) * ratio_x;
        let mouse_y_unscaled = (camera.y / ratio_y + mouse_state.y() as f32) * ratio_y;
        let mx = mouse_state.x() as f32 * ratio_x;
        let my = mouse_state.y() as f32 * ratio_y;
        if banner_on {
            graphics_utils::render(
                &mut canvas,
                &banner_texture,
                Point::new(0, 0),
                sprite_426x240,
                1.0,
                ratio_x,
                ratio_y,
            );
        } else if main_menu_on {
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
            for button in menu_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                button.check_if_hovered(mx, my, ratio_x, ratio_y);
                button.check_if_pressed(mx, my, mouse_state.left());
            }

            // play button
            for button in menu_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                if button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &menu_button_hovered_texture,
                        position,
                        sprite_128x32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if button.status == graphics_utils::ButtonStatus::Pressed {
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
            }
            let position = Point::new(menu_buttons[0].x as i32, menu_buttons[0].y as i32);
            // render text
            let title_text = graphics_utils::get_text(
                "Tales of Terrant: The Cult of Plasmic Ocean".to_string(),
                Color::RGBA(255, 255, 255, 255),
                main_title_font_size,
                &main_title_font,
                &texture_creator,
            )
            .unwrap();
            let position = Point::new((SCREEN_WIDTH / 2 - 175) as i32, 16 as i32);
            let text_margin = 4;
            graphics_utils::render_text(
                &mut canvas,
                &title_text.text_texture,
                position,
                title_text.text_sprite,
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
                menu_buttons[1].x as i32 + text_margin,
                menu_buttons[1].y as i32 + text_margin,
            );
            graphics_utils::render_text(
                &mut canvas,
                &settings_text.text_texture,
                position,
                settings_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let position = Point::new((SCREEN_WIDTH / 2 - 120) as i32, 16 as i32);
            let play_text = graphics_utils::get_text(
                "Play".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            let position = Point::new(
                menu_buttons[0].x as i32 + text_margin,
                menu_buttons[0].y as i32 + text_margin,
            );
            graphics_utils::render_text(
                &mut canvas,
                &play_text.text_texture,
                position,
                play_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let position = Point::new(
                menu_buttons[1].x as i32 + text_margin,
                menu_buttons[1].y as i32 + text_margin,
            );
            let manual_text = graphics_utils::get_text(
                "Manual".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            let position = Point::new(
                menu_buttons[2].x as i32 + text_margin,
                menu_buttons[2].y as i32 + text_margin,
            );
            let text_margin = 4;
            graphics_utils::render_text(
                &mut canvas,
                &manual_text.text_texture,
                position,
                manual_text.text_sprite,
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
                menu_buttons[3].x as i32 + text_margin,
                menu_buttons[3].y as i32 + text_margin,
            );
            graphics_utils::render_text(
                &mut canvas,
                &exit_text.text_texture,
                position,
                exit_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let position = Point::new(
                menu_buttons[3].x as i32 + text_margin,
                menu_buttons[3].y as i32 + text_margin,
            );
            graphics_utils::render_text(
                &mut canvas,
                &exit_text.text_texture,
                position,
                exit_text.text_sprite,
                ratio_x,
                ratio_y,
            );

            if menu_buttons[0].status == ButtonStatus::Released {
                main_menu_on = false;
            } else if menu_buttons[1].status == ButtonStatus::Released {
                main_menu_on = false;
                settings_menu_on = true;
            }
            if menu_buttons[1].status == ButtonStatus::Released {
            } else if menu_buttons[1].status == ButtonStatus::Released {
                main_menu_on = false;
                settings_menu_on = true;
            } else if menu_buttons[2].status == ButtonStatus::Released {
                main_menu_on = false;
                manual_menu_on = true;
            } else if menu_buttons[3].status == ButtonStatus::Released {
                running = false;
            }
        } else if settings_menu_on {
            graphics_utils::render(
                &mut canvas,
                &menu_background,
                Point::new(0, 0),
                sprite_720x480,
                1.0,
                ratio_x,
                ratio_y,
            );
            for button in settings_action_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                if button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &menu_button_hovered_texture,
                        position,
                        sprite_128x32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if button.status == graphics_utils::ButtonStatus::Pressed {
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
            }
            for button in settings_action_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                button.check_if_hovered(mx, my, ratio_x, ratio_y);
                button.check_if_pressed(mx, my, mouse_state.left());
            }
            let position = Point::new(
                settings_action_buttons[0].x as i32 + 4,
                settings_action_buttons[0].y as i32 + 4,
            );
            let text_margin = 4;
            let back_text = graphics_utils::get_text(
                "Music".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();

            graphics_utils::render_text(
                &mut canvas,
                &back_text.text_texture,
                position,
                back_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let position = Point::new(
                settings_action_buttons[1].x as i32 + 4,
                settings_action_buttons[1].y as i32 + 4,
            );
            let text_margin = 4;
            let back_text = graphics_utils::get_text(
                "Sounds".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();

            graphics_utils::render_text(
                &mut canvas,
                &back_text.text_texture,
                position,
                back_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            if settings_action_buttons[0].status == ButtonStatus::Released {
                if current_song.is_playing() {
                    current_song.pause();
                } else {
                    current_song.play();
                }
            }
            if settings_action_buttons[1].status == ButtonStatus::Released {
                if sounds_volume == 0.0 {
                    sounds_volume = 0.7;
                } else {
                    sounds_volume = 0.0;
                }
            }
            for button in settings_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                if button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &menu_button_hovered_texture,
                        position,
                        sprite_128x32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if button.status == graphics_utils::ButtonStatus::Pressed {
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
            }

            for button in settings_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                button.check_if_hovered(mx, my, ratio_x, ratio_y);
                button.check_if_pressed(mx, my, mouse_state.left());
            }
            if settings_buttons[0].status == ButtonStatus::Released {
                main_menu_on = true;
                settings_menu_on = false;
                manual_menu_on = false;
            }
            for button in settings_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                if button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &menu_button_hovered_texture,
                        position,
                        sprite_128x32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if button.status == graphics_utils::ButtonStatus::Pressed {
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
            }
            for button in settings_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                if button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &menu_button_hovered_texture,
                        position,
                        sprite_128x32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if button.status == graphics_utils::ButtonStatus::Pressed {
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
            }
            let position = Point::new(
                settings_buttons[0].x as i32 + 4,
                settings_buttons[0].y as i32 + 4,
            );
            let text_margin = 4;
            let back_text = graphics_utils::get_text(
                "Back".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();

            graphics_utils::render_text(
                &mut canvas,
                &back_text.text_texture,
                position,
                back_text.text_sprite,
                ratio_x,
                ratio_y,
            );
        } else if manual_menu_on {
            graphics_utils::render(
                &mut canvas,
                &menu_background,
                Point::new(0, 0),
                sprite_720x480,
                1.0,
                ratio_x,
                ratio_y,
            );
            // wiki text
            graphics_utils::render(
                &mut canvas,
                &wiki_text_background,
                Point::new(16, 16),
                sprite_698x212,
                1.0,
                ratio_x,
                ratio_y,
            );
            // wiki texts
            let vector = &wiki_texts[wiki_index];
            let mut line_number = 0;
            for line in vector.iter() {
                let mut header_factor = 0;
                line_number += 1;
                let mut l = line.clone().to_string();
                let mut wiki_text: Option<graphics_utils::Text> = None;
                let mut retained_l = l.clone();
                retained_l.retain(|x| x != '#');
                if &l[..2] == "##" {
                    wiki_text = graphics_utils::get_text(
                        retained_l,
                        Color::RGBA(255, 255, 255, 255),
                        wiki_h1_font_size,
                        &wiki_h1_font,
                        &texture_creator,
                    );
                    header_factor = 1;
                } else if &l[..1] == "#" {
                    wiki_text = graphics_utils::get_text(
                        retained_l,
                        Color::RGBA(255, 255, 255, 255),
                        wiki_h1_font_size,
                        &wiki_h1_font,
                        &texture_creator,
                    );
                    header_factor = 1;
                } else {
                    wiki_text = graphics_utils::get_text(
                        l,
                        Color::RGBA(255, 255, 255, 255),
                        wiki_text_font_size,
                        &wiki_text_font,
                        &texture_creator,
                    );
                }
                let position =
                    Point::new(32 as i32 - header_factor * 8, 8 as i32 + line_number * 14);
                let w_text = wiki_text.unwrap();
                graphics_utils::render_text(
                    &mut canvas,
                    &w_text.text_texture,
                    position,
                    w_text.text_sprite,
                    ratio_x,
                    ratio_y,
                );
            }
            for button in wiki_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                if button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &ui_button_hovered_texture,
                        position,
                        sprite_32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if button.status == graphics_utils::ButtonStatus::Pressed {
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
                let position = Point::new(button.x as i32, button.y as i32);
                button.check_if_hovered(mx, my, ratio_x, ratio_y);
                button.check_if_pressed(mx, my, mouse_state.left());
            }

            let forward_text = graphics_utils::get_text(
                "->".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            let back_text = graphics_utils::get_text(
                "<-".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            let position = Point::new(wiki_buttons[0].x as i32 + 4, wiki_buttons[0].y as i32 + 4);
            graphics_utils::render_text(
                &mut canvas,
                &back_text.text_texture,
                position,
                back_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let position = Point::new(wiki_buttons[1].x as i32 + 4, wiki_buttons[1].y as i32 + 4);
            graphics_utils::render_text(
                &mut canvas,
                &forward_text.text_texture,
                position,
                back_text.text_sprite,
                ratio_x,
                ratio_y,
            );
            let back_text = graphics_utils::get_text(
                "Back".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();

            let position = Point::new(manual_buttons[0].x as i32, manual_buttons[0].y as i32);
            let text_margin = 4;

            for button in manual_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                button.check_if_hovered(mx, my, ratio_x, ratio_y);
                button.check_if_pressed(mx, my, mouse_state.left());
            }
            for button in manual_buttons.iter_mut() {
                let position = Point::new(button.x as i32, button.y as i32);
                if button.status == graphics_utils::ButtonStatus::Hovered {
                    graphics_utils::render(
                        &mut canvas,
                        &menu_button_hovered_texture,
                        position,
                        sprite_128x32,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                } else if button.status == graphics_utils::ButtonStatus::Pressed {
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
            }

            let position = Point::new(
                manual_buttons[0].x as i32 + 4,
                manual_buttons[0].y as i32 + 4,
            );
            let text_margin = 4;
            let back_text = graphics_utils::get_text(
                "Back".to_string(),
                Color::RGBA(255, 255, 255, 255),
                desc_font_size,
                &font,
                &texture_creator,
            )
            .unwrap();
            graphics_utils::render_text(
                &mut canvas,
                &back_text.text_texture,
                position,
                back_text.text_sprite,
                ratio_x,
                ratio_y,
            );

            if manual_buttons[0].status == ButtonStatus::Released {
                main_menu_on = true;
                settings_menu_on = false;
                manual_menu_on = false;
            }
            if wiki_buttons[0].status == ButtonStatus::Released {
                if wiki_index > 0 {
                    wiki_index -= 1;
                }
            }
            if wiki_buttons[1].status == ButtonStatus::Released {
                if wiki_index < wiki_texts.len() - 1 {
                    wiki_index += 1;
                }
            }

            // main loop no menus
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

            if player.shoot_data.shooting {
                player.shoot_data.mx = mouse_x_unscaled as i32;
                player.shoot_data.my = mouse_y_unscaled as i32;
                player.shoot_data.action_type = player_action.clone();
            }
            let packet = ClientPacket {
                player: player.clone(),
                camera: camera.clone(),
            };
            if player.shoot_data.shooting {
                if player_action == PlayerAction::Meteoroid {
                    meteoroid_spawn.play();
                } else if player_action == PlayerAction::Siphon {
                    soul_trap_build.play();
                } else if player_action == PlayerAction::Raft {
                    raft_build.play();
                }
                player.shoot_data.shooting = false;
            }
            let encoded: Vec<u8> = bincode::serialize(&packet).unwrap();
            //let decoded: ClientPacket = bincode::deserialize(&encoded).unwrap();
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
                    colliders = world_from.colliders;
                    props = world_from.props;
                    players = world_from.players;
                    for p in players.iter() {
                        if p.id == player.id && p.energy == player.energy {
                            player.energy = p.energy;
                        }
                    }
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
                            // tile collision
                            let player_x_left = player.x;
                            let player_x_right = player.x + ENTITY_SIZE;
                            let player_y_up = player.y;
                            let player_y_down = player.y + ENTITY_SIZE;

                            if player.x + ENTITY_SIZE / 2.0 > p.x * TILE_SIZE
                                && player.x + ENTITY_SIZE / 2.0 < p.x * TILE_SIZE + TILE_SIZE
                                && player_y_up - player.speed * delta_as_millis as f32 / 100.0
                                    > p.y * TILE_SIZE
                                && player_y_up - player.speed * delta_as_millis as f32 / 100.0
                                    < p.y * TILE_SIZE + TILE_SIZE
                            {
                                if p.tile_type == TileType::Water {
                                    up_collision = true;
                                } else {
                                    up_collision = false;
                                }
                            }
                            if player.x + ENTITY_SIZE / 2.0 > p.x * TILE_SIZE
                                && player.x + ENTITY_SIZE / 2.0 < p.x * TILE_SIZE + TILE_SIZE
                                && player_y_down + player.speed * delta_as_millis as f32 / 100.0
                                    > p.y * TILE_SIZE
                                && player_y_down + player.speed * delta_as_millis as f32 / 100.0
                                    < p.y * TILE_SIZE + TILE_SIZE
                            {
                                if p.tile_type == TileType::Water {
                                    down_collision = true;
                                } else {
                                    down_collision = false;
                                }
                            }
                            if player_x_left - player.speed * delta_as_millis as f32 / 100.0
                                > p.x * TILE_SIZE
                                && player_x_left - player.speed * delta_as_millis as f32 / 100.0
                                    < p.x * TILE_SIZE + TILE_SIZE
                                && player.y + ENTITY_SIZE / 2.0 > p.y * TILE_SIZE
                                && player.y + ENTITY_SIZE / 2.0 < p.y * TILE_SIZE + TILE_SIZE
                            {
                                if p.tile_type == TileType::Water {
                                    left_collision = true;
                                } else {
                                    left_collision = false;
                                }
                            }
                            if player_x_right + player.speed * delta_as_millis as f32 / 100.0
                                > p.x * TILE_SIZE
                                && player_x_right + player.speed * delta_as_millis as f32 / 100.0
                                    < p.x * TILE_SIZE + TILE_SIZE
                                && player.y + ENTITY_SIZE / 2.0 > p.y * TILE_SIZE
                                && player.y + ENTITY_SIZE / 2.0 < p.y * TILE_SIZE + TILE_SIZE
                            {
                                if p.tile_type == TileType::Water {
                                    right_collision = true;
                                } else {
                                    right_collision = false;
                                }
                            }
                        }
                    }
                }
                if green_flashing {
                    green_flashing_change += delta_as_millis;
                    graphics_utils::render(
                        &mut canvas,
                        &green_flash_texture,
                        Point::new(0, 0),
                        sprite_426x240,
                        1.0,
                        ratio_x,
                        ratio_y,
                    );
                    if green_flashing_change > green_flashing_time {
                        green_flashing = false;
                        green_flashing_change = 0;
                    }
                }
                // remove entities and colliders
                for collider in colliders.iter() {
                    if collider.lethal {
                        green_flashing = true;
                        if !meteoroid_explode.is_playing() {
                            meteoroid_explode.play();
                        }
                    }
                }
                //render entities
                let mut colliders_clone = colliders.clone();
                for i in 0..chunks.len() {
                    for j in 0..chunks[i].len() {
                        let mut entities_vals: Vec<Entity> =
                            chunks[i][j].entities.values().cloned().collect();

                        entities_vals.sort_by(|a, b| a.id.cmp(&b.id));
                        for entity in entities_vals.iter() {
                            if entity.hp < 0 {
                                continue;
                            }
                            'collide_loop: for collider in &colliders {
                                let id = already_collided_to_entity.get(&collider.id);
                                match id {
                                    Some(_) => continue 'collide_loop,
                                    None => (),
                                }
                                if collider.collider_type == ColliderType::SoulTrap {
                                    if collider.x > entity.x
                                        && collider.x < entity.x + 8.0
                                        && collider.y > entity.y
                                        && collider.y < entity.y + 8.0
                                    {
                                        if player.energy + 10 <= 100 {
                                            soul_trapped.play();
                                            player.energy += 10;
                                        }
                                        let id = collider.id;
                                        already_collided_to_entity.insert(collider.id, entity.id);
                                    }
                                }
                            }
                            let tx = (entity.x) * camera.zoom - camera.x;
                            let ty = (entity.y) * camera.zoom - camera.y;
                            let tx_ant = (entity.x) * camera.zoom - camera.x;
                            let ty_ant = (entity.y) * camera.zoom - camera.y;
                            let tx_tree = (entity.x + TILE_SIZE / 2.0) * camera.zoom - camera.x;
                            let ty_tree = (entity.y - TILE_SIZE / 4.0) * camera.zoom - camera.y;

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
                                let mut tex = &ant_worker_texture_side_1;
                                if entity.dir >= std::f64::consts::PI as f32 * (0.0)
                                    && entity.dir <= std::f64::consts::PI as f32 * (1.0)
                                {
                                    if entity.current_action != ActionType::Idle
                                        && entity.time / (DRONE_ANIMATION_SPEED) % 2 == 0
                                    {
                                        tex = &ant_worker_texture_side_1;
                                    } else {
                                        tex = &ant_worker_texture_side_2;
                                    }
                                } else if entity.dir >= std::f64::consts::PI as f32 * (1.0)
                                    && entity.dir <= std::f64::consts::PI as f32 * (2.0)
                                {
                                    if entity.current_action != ActionType::Idle
                                        && entity.time / (DRONE_ANIMATION_SPEED) % 2 == 0
                                    {
                                        tex = &ant_worker_texture_side_mirror_2;
                                    } else {
                                        tex = &ant_worker_texture_side_mirror_1;
                                    }
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

                for collider in colliders.iter() {
                    if collider.hp < 0 {
                        continue;
                    }
                    let tx_c = (collider.x) * camera.zoom - camera.x;
                    let ty_c = (collider.y) * camera.zoom - camera.y;
                    if collider.collider_type == ColliderType::Meteoroid {
                        let position = Point::new(tx_c as i32, ty_c as i32);
                        let mut tex = &meteoroid_texture;
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
                    if collider.collider_type == ColliderType::SoulTrap {
                        let position = Point::new(tx_c as i32, ty_c as i32);
                        let mut tex = &soul_trap_texture;
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
                }

                // render props
                for prop in props.iter() {
                    let tx_c = (prop.x) * camera.zoom - camera.x;
                    let ty_c = (prop.y) * camera.zoom - camera.y;
                    if prop.prop_type == PropType::Raft {
                        let position = Point::new(tx_c as i32, ty_c as i32);
                        let mut tex = &raft_texture;
                        graphics_utils::render(
                            &mut canvas,
                            &tex,
                            position,
                            sprite_16,
                            camera.zoom,
                            ratio_x,
                            ratio_y,
                        );
                    }
                }

                // collide props
                for prop in props.iter() {
                    let p = &prop;
                    let player_x_left = player.x;
                    let player_x_right = player.x + ENTITY_SIZE;
                    let player_y_up = player.y;
                    let player_y_down = player.y + ENTITY_SIZE;

                    if player.x + ENTITY_SIZE / 2.0 > p.x
                        && player.x + ENTITY_SIZE / 2.0 < p.x + TILE_SIZE
                        && player_y_up - player.speed * delta_as_millis as f32 / 100.0 > p.y
                        && player_y_up - player.speed * delta_as_millis as f32 / 100.0
                            < p.y + TILE_SIZE
                    {
                        if p.prop_type == PropType::Raft {
                            up_collision = false;
                        } else {
                            up_collision = true;
                        }
                    }
                    if player.x + ENTITY_SIZE / 2.0 > p.x
                        && player.x + ENTITY_SIZE / 2.0 < p.x + TILE_SIZE
                        && player_y_down + player.speed * delta_as_millis as f32 / 100.0 > p.y
                        && player_y_down + player.speed * delta_as_millis as f32 / 100.0
                            < p.y + TILE_SIZE
                    {
                        if p.prop_type == PropType::Raft {
                            down_collision = false;
                        } else {
                            down_collision = true;
                        }
                    }
                    if player_x_left - player.speed * delta_as_millis as f32 / 100.0 > p.x
                        && player_x_left - player.speed * delta_as_millis as f32 / 100.0
                            < p.x + TILE_SIZE
                        && player.y + ENTITY_SIZE / 2.0 > p.y
                        && player.y + ENTITY_SIZE / 2.0 < p.y + TILE_SIZE
                    {
                        if p.prop_type == PropType::Raft {
                            left_collision = false;
                        } else {
                            left_collision = true;
                        }
                    }
                    if player_x_right + player.speed * delta_as_millis as f32 / 100.0 > p.x
                        && player_x_right + player.speed * delta_as_millis as f32 / 100.0
                            < p.x + TILE_SIZE
                        && player.y + ENTITY_SIZE / 2.0 > p.y
                        && player.y + ENTITY_SIZE / 2.0 < p.y + TILE_SIZE
                    {
                        if p.prop_type == PropType::Raft {
                            right_collision = false;
                        } else {
                            right_collision = true;
                        }
                    }
                }
                // render player
                if !player.stopped && player.time % 100 == 0 {
                    if !player_footstep.is_playing() {
                        player_footstep.set_volume(sounds_volume);
                        player_footstep.play();
                    }
                }
                let mut tex = &plasmant_texture_side_1;
                if !player.stopped && (client_time / PLAYER_ANIMATION_SPEED) % 2 == 0 {
                    tex = &plasmant_texture_side_2;
                }

                if player.dir >= std::f64::consts::PI as f32 * (0.0)
                    && player.dir <= std::f64::consts::PI as f32 * (1.0)
                {
                    if !player.stopped && player.time / (PLAYER_ANIMATION_SPEED) % 2 == 0 {
                        tex = &plasmant_texture_side_1;
                    } else {
                        tex = &plasmant_texture_side_2;
                    }
                } else if player.dir >= std::f64::consts::PI as f32 * (1.0)
                    && player.dir <= std::f64::consts::PI as f32 * (2.0)
                {
                    if !player.stopped && player.time / (PLAYER_ANIMATION_SPEED) % 2 == 0 {
                        tex = &plasmant_texture_side_mirror_2;
                    } else {
                        tex = &plasmant_texture_side_mirror_1;
                    }
                }
                let player_position = Point::new(
                    (player.x * camera.zoom - camera.x) as i32,
                    (player.y * camera.zoom - camera.y) as i32,
                );
                graphics_utils::render(
                    &mut canvas,
                    &tex,
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

                /*let position = Point::new(0 as i32, 112 as i32);
                graphics_utils::render(
                    &mut canvas,
                    &map_ui_texture,
                    position,
                    sprite_64x112,
                    1.0,
                    ratio_x,
                    ratio_y,
                );*/
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
                    ((1.0.lerp(64.0, player.energy as f32 / 100.0)) / ratio_x) as u32,
                    (8.0 / ratio_y) as u32,
                );

                graphics_utils::render_rect(
                    &mut canvas,
                    position,
                    render_rect,
                    Color::RGBA(0, 255, 100, 55),
                    1.0,
                );
                // icon buttons
                for button in action_icon_buttons.iter_mut() {
                    let position = Point::new(button.x as i32, button.y as i32);
                    button.check_if_hovered(
                        mouse_state.x() as f32 * ratio_x,
                        mouse_state.y() as f32 * ratio_y,
                        ratio_x,
                        ratio_y,
                    );
                    button.check_if_pressed(mouse_x, mouse_y, mouse_state.left());
                    if button.status == graphics_utils::ButtonStatus::Hovered {
                        graphics_utils::render(
                            &mut canvas,
                            &action_icon_button_hovered_texture,
                            position,
                            sprite_12,
                            1.0,
                            ratio_x,
                            ratio_y,
                        );
                    } else if button.status == graphics_utils::ButtonStatus::Pressed {
                        if !button_click.is_playing() {
                            button_click.set_volume(sounds_volume);
                            button_click.play();
                        }
                        graphics_utils::render(
                            &mut canvas,
                            &action_icon_button_pressed_texture,
                            position,
                            sprite_12,
                            1.0,
                            ratio_x,
                            ratio_y,
                        );
                    } else {
                        graphics_utils::render(
                            &mut canvas,
                            &action_icon_button_texture,
                            position,
                            sprite_12,
                            1.0,
                            ratio_x,
                            ratio_y,
                        );
                    }
                }
                if action_icon_buttons[0].status == ButtonStatus::Released {
                    player_action = PlayerAction::Siphon;
                } else if action_icon_buttons[2].status == ButtonStatus::Released {
                    player_action = PlayerAction::Meteoroid;
                } else if action_icon_buttons[1].status == ButtonStatus::Released {
                    player_action = PlayerAction::Raft;
                }
                // raft icon
                let position = Point::new(
                    action_icon_buttons[0].x as i32 + 2,
                    action_icon_buttons[0].y as i32 + 2,
                );
                // meteoroid icon
                graphics_utils::render(
                    &mut canvas,
                    &siphon_icon_texture,
                    position,
                    sprite_8,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
                let position = Point::new(
                    action_icon_buttons[2].x as i32 + 2,
                    action_icon_buttons[2].y as i32 + 2,
                );
                graphics_utils::render(
                    &mut canvas,
                    &meteoroid_icon_texture,
                    position,
                    sprite_8,
                    1.0,
                    ratio_x,
                    ratio_y,
                );
                let position = Point::new(
                    action_icon_buttons[1].x as i32 + 2,
                    action_icon_buttons[1].y as i32 + 2,
                );
                graphics_utils::render(
                    &mut canvas,
                    &raft_icon_texture,
                    position,
                    sprite_8,
                    1.0,
                    ratio_x,
                    ratio_y,
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
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
                    if !button_click.is_playing() {
                        button_click.set_volume(sounds_volume);
                        button_click.play();
                    }
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
            // player movement
            player.mov(graphics_utils::MoveDirection::Nothing, delta_as_millis);
            if w && !up_collision {
                player.mov(graphics_utils::MoveDirection::Up, delta_as_millis);
                if player.get_relative_y(&camera) <= CAMERA_BUFFER_TOP {
                    camera.mov(
                        graphics_utils::MoveDirection::Up,
                        player.speed,
                        delta_as_millis,
                    );
                }
            }
            if a && !left_collision {
                player.mov(graphics_utils::MoveDirection::Left, delta_as_millis);
                if player.get_relative_x(&camera) <= CAMERA_BUFFER_LEFT {
                    camera.mov(
                        graphics_utils::MoveDirection::Left,
                        player.speed,
                        delta_as_millis,
                    );
                }
            }
            if s && !down_collision {
                player.mov(graphics_utils::MoveDirection::Down, delta_as_millis);
                if player.get_relative_y(&camera) >= SCREEN_HEIGHT as f32 - CAMERA_BUFFER_BOTTOM {
                    camera.mov(
                        graphics_utils::MoveDirection::Down,
                        player.speed,
                        delta_as_millis,
                    );
                }
            }
            if d && !right_collision {
                player.mov(graphics_utils::MoveDirection::Right, delta_as_millis);
                if player.get_relative_x(&camera) >= SCREEN_WIDTH as f32 - CAMERA_BUFFER_RIGHT {
                    camera.mov(
                        graphics_utils::MoveDirection::Right,
                        player.speed,
                        delta_as_millis,
                    );
                }
            }
        }
        if normal_button.status == graphics_utils::ButtonStatus::Pressed {
            if !button_click.is_playing() {
                button_click.set_volume(sounds_volume);
                button_click.play();
            }
            map_state = graphics_utils::MapState::Normal;
        } else if political_button.status == graphics_utils::ButtonStatus::Pressed {
            map_state = graphics_utils::MapState::Political;
        } else if religion_button.status == graphics_utils::ButtonStatus::Pressed {
            map_state = graphics_utils::MapState::Religion;
        }

        canvas.present();
        thread::sleep(time::Duration::from_millis(1));
    }

    println!("Socket connection ended.");
    Ok(())
}
pub fn run() {
    main_loop();
}
