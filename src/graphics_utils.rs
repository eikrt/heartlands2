use crate::world_structs::{EntityType, ReligionType, TileType};
use bincode;
use rand::Rng;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(PartialEq)]
pub enum MapState {
    Normal,
    Political,
    Religion,
}
#[derive(PartialEq)]
pub enum MoveDirection {
    Up,
    Left,
    Down,
    Right,
    Nothing,
    Zoomin,
    Zoomout,
}
#[derive(PartialEq, Clone, Debug)]
pub enum ButtonStatus {
    Neutral,
    Hovered,
    Pressed,
    Released,
}
#[derive(Serialize, Deserialize, Clone)]

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub zoom_speed: f32,
    pub move_speed: f32,
}
impl Camera {
    pub fn zoom(&mut self, dir: MoveDirection, delta: u128) {
        if dir == MoveDirection::Zoomin {
            self.zoom += self.zoom_speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Zoomout {
            self.zoom -= self.zoom_speed * delta as f32 / 100.0;
        }
    }
    pub fn mov(&mut self, dir: MoveDirection, speed: f32, delta: u128) {
        if dir == MoveDirection::Up {
            self.y -= speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Left {
            self.x -= speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Down {
            self.y += speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Right {
            self.x += speed * delta as f32 / 100.0;
        }
    }
}

pub struct Button {
    pub status: ButtonStatus,
    pub previous_status: ButtonStatus,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl Button {
    pub fn hover(&mut self) {
        self.status = ButtonStatus::Hovered;
    }
    pub fn press(&mut self) {
        self.status = ButtonStatus::Pressed;
    }
    pub fn release(&mut self) {
        self.status = ButtonStatus::Released;
    }
    pub fn neutralize(&mut self) {
        self.status = ButtonStatus::Neutral;
    }
    pub fn check_if_hovered(&mut self, m_x: f32, m_y: f32, ratio_x: f32, ratio_y: f32) {
        let m_x2 = m_x;
        let m_y2 = m_y;
        if m_x2 > self.x as f32
            && m_x2 < self.x + self.width
            && m_y2 > self.y
            && m_y2 < self.y + self.height
        {
            self.hover();
        } else {
            self.neutralize();
        }
    }
    pub fn check_if_pressed(&mut self, _m_x: f32, _m_y: f32, pressed: bool) {
        if pressed && self.status == ButtonStatus::Hovered {
            self.press();
        } else if !pressed && self.previous_status == ButtonStatus::Pressed {
            self.release();
        }
        self.previous_status = self.status.clone();
    }
}
pub struct TileGraphics {
    pub sc: Color,
    pub tc: Color,
}

pub struct EntityGraphics {
    pub src: String,
    pub color: Color,
    pub position: Point,
    pub sprite: Rect,
}

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
    zoom: f32,
    ratio_x: f32,
    ratio_y: f32,
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let screen_rect = Rect::new(
        (position.x as f32 / ratio_x) as i32,
        (position.y as f32 / ratio_y) as i32,
        (sprite.width() as f32 * zoom / ratio_x) as u32,
        (sprite.height() as f32 * zoom / ratio_y) as u32,
    );
    canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
}
pub fn render_transparent(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
    zoom: f32,
    ratio_x: f32,
    ratio_y: f32,
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let screen_rect = Rect::new(
        (position.x as f32 / ratio_x) as i32,
        (position.y as f32 / ratio_y) as i32,
        (sprite.width() as f32 * zoom / ratio_x) as u32,
        (sprite.height() as f32 * zoom / ratio_y) as u32,
    );
    canvas.copy(texture, sprite, screen_rect)?;
    Ok(())
}

pub fn render_tile_with_color(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
    color: Color,
    zoom: f32,
    ratio_x: f32,
    ratio_y: f32,
) -> Result<(), String> {
    let (_width, _height) = canvas.output_size()?;
    let screen_rect = Rect::new(
        (position.x as f32 / ratio_x - 1.0) as i32,
        (position.y as f32 / ratio_y - 1.0) as i32,
        (sprite.width() as f32 * zoom / ratio_x + 1.0) as u32,
        (sprite.height() as f32 * zoom / ratio_y + 1.0) as u32,
    );
    canvas.copy(texture, sprite, screen_rect)?;
    canvas.set_draw_color(color);
    match canvas.fill_rect(Rect::new(
        (position.x as f32 / ratio_x - 1.0) as i32,
        (position.y as f32 / ratio_y - 1.0) as i32,
        (sprite.width() as f32 * zoom / ratio_x + 1.0) as u32,
        (sprite.height() as f32 * zoom / ratio_y + 1.0) as u32,
    )) {
        Ok(_v) => (),
        Err(_v) => (),
    }
    Ok(())
}
pub fn render_rect(
    canvas: &mut WindowCanvas,
    position: Point,
    sprite: Rect,
    color: Color,
    zoom: f32,
) -> Result<(), String> {
    let (_width, _height) = canvas.output_size()?;
    canvas.set_draw_color(color);
    match canvas.fill_rect(Rect::new(
        position.x as i32,
        position.y as i32,
        (sprite.width() as f32 * zoom) as u32,
        (sprite.height() as f32 * zoom) as u32,
    )) {
        Ok(_v) => (),
        Err(_v) => (),
    }
    Ok(())
}
pub fn render_text(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
    ratio_x: f32,
    ratio_y: f32,
) {
    let screen_rect = Rect::new(
        (position.x as f32 / ratio_x) as i32,
        (position.y as f32 / ratio_y) as i32,
        (sprite.width() as f32 / ratio_x) as u32,
        (sprite.height() as f32 / ratio_y) as u32,
    );
    canvas.copy(texture, None, screen_rect);
}
pub fn tile_graphics() -> HashMap<TileType, TileGraphics> {
    return HashMap::from([
        (
            TileType::Grass,
            TileGraphics {
                sc: Color::RGB(58, 132, 56),
                tc: Color::RGB(5, 85, 5),
            },
        ),
        (
            TileType::ColdLand,
            TileGraphics {
                sc: Color::RGB(58, 132, 56),
                tc: Color::RGB(5, 85, 5),
            },
        ),
        (
            TileType::Water,
            TileGraphics {
                sc: Color::RGB(65, 65, 195),
                tc: Color::RGB(17, 17, 87),
            },
        ),
        (
            TileType::Ice,
            TileGraphics {
                sc: Color::RGB(255, 255, 255),
                tc: Color::RGB(200, 200, 250),
            },
        ),
        (
            TileType::PermaFrost,
            TileGraphics {
                sc: Color::RGB(58, 125, 50),
                tc: Color::RGB(95, 110, 95),
            },
        ),
        (
            TileType::CoarseLand,
            TileGraphics {
                sc: Color::RGB(150, 145, 105),
                tc: Color::RGB(90, 85, 45),
            },
        ),
        (
            TileType::SavannahLand,
            TileGraphics {
                sc: Color::RGB(186, 165, 80),
                tc: Color::RGB(150, 150, 105),
            },
        ),
        (
            TileType::Sand,
            TileGraphics {
                sc: Color::RGB(255, 247, 56),
                tc: Color::RGB(170, 165, 90),
            },
        ),
        (
            TileType::RedSand,
            TileGraphics {
                sc: Color::RGB(190, 130, 80),
                tc: Color::RGB(150, 90, 35),
            },
        ),
        (
            TileType::MudHiveWall,
            TileGraphics {
                sc: Color::RGB(90, 90, 60),
                tc: Color::RGB(90, 90, 60),
            },
        ),
        (
            TileType::MudHiveFloor,
            TileGraphics {
                sc: Color::RGB(120, 120, 75),
                tc: Color::RGB(120, 120, 75),
            },
        ),
        (
            TileType::StoneHiveWall,
            TileGraphics {
                sc: Color::RGB(110, 110, 110),
                tc: Color::RGB(110, 110, 110),
            },
        ),
        (
            TileType::StoneHiveFloor,
            TileGraphics {
                sc: Color::RGB(50, 50, 50),
                tc: Color::RGB(50, 50, 50),
            },
        ),
    ]);
}
pub struct SkillDesc {
    pub title: String,
    pub text: String,
}
pub struct Text<'a> {
    pub text_surface: Surface<'a>,
    pub text_texture: Texture<'a>,
    pub text_sprite: Rect,
}

pub fn get_text<'a, T>(
    msg: String,
    color: Color,
    font_size: u16,
    font: &Font,
    texture_creator: &'a TextureCreator<T>,
) -> Option<Text<'a>> {
    let text_surface = font
        .render(&msg)
        .blended(color)
        .map_err(|e| e.to_string())
        .ok()?;
    let text_texture = texture_creator
        .create_texture_from_surface(&text_surface)
        .map_err(|e| e.to_string())
        .ok()?;
    let text_sprite = Rect::new(
        0,
        0,
        (font_size as f32 / 2.0) as u32 * msg.len() as u32,
        (font_size as f32) as u32,
    );

    let text = Text {
        text_surface: text_surface,
        text_texture: text_texture,
        text_sprite: text_sprite,
    };
    return Some(text);
}
pub fn get_descriptions_for_entities() -> HashMap<EntityType, String> {
    let entity_descriptions = HashMap::from([
        (EntityType::Oak, "Oak".to_string()),
        (EntityType::Birch, "Birch".to_string()),
        (EntityType::AppleTree, "Apple tree".to_string()),
        (EntityType::Pine, "Pine".to_string()),
        (EntityType::Spruce, "Spruce".to_string()),
        (EntityType::Cactus, "Cactus".to_string()),
        (EntityType::WorkerAnt, "ant worker".to_string()),
        (EntityType::QueenAnt, "ant queen".to_string()),
        (EntityType::DroneAnt, "ant drone".to_string()),
        (EntityType::SoldierAnt, "ant soldier".to_string()),
        (EntityType::Mechant, "mechant".to_string()),
        (EntityType::FoodStorage, "Food storage".to_string()),
        (EntityType::Snail, "Snail".to_string()),
        (EntityType::FoodStorage, "Food storage".to_string()),
        (EntityType::AntEgg, "Ant egg".to_string()),
        (EntityType::AntEgg, "Ant egg".to_string()),
    ]);
    return entity_descriptions;
}
pub fn get_descriptions_for_tiles() -> HashMap<TileType, String> {
    let tile_descriptions = HashMap::from([
        (TileType::Grass, "Grass".to_string()),
        (TileType::ColdLand, "Grass".to_string()),
        (TileType::Ice, "Ice".to_string()),
        (TileType::Water, "Water".to_string()),
        (TileType::CoarseLand, "Coarse grass".to_string()),
        (TileType::SavannahLand, "Savannah grass".to_string()),
        (TileType::Sand, "Sand".to_string()),
        (TileType::RedSand, "Red sand".to_string()),
        (TileType::PermaFrost, "Frozen ground".to_string()),
        (TileType::MudHiveWall, "Mud wall".to_string()),
        (TileType::MudHiveFloor, "Mud floor".to_string()),
    ]);
    return tile_descriptions;
}
pub fn get_descriptions_for_religions() -> HashMap<ReligionType, String> {
    let religion_descriptions = HashMap::from([
        (ReligionType::Plasma, "Plasma".to_string()),
        (ReligionType::Moon, "Moon".to_string()),
        (ReligionType::Technology, "Technology".to_string()),
        (ReligionType::Giants, "Giants".to_string()),
        (ReligionType::Element, "Element".to_string()),
        (ReligionType::Spiral, "Spiral".to_string()),
        (ReligionType::Infinity, "Infinity".to_string()),
        (ReligionType::Sacrifice, "Sacrifice".to_string()),
        (ReligionType::Nothing, "No religion".to_string()),
    ]);
    return religion_descriptions;
}
pub fn get_dialogue_for_criteria(relation: i32, chunk_relations: HashMap<String, i32>) -> String {
    let mut rng = rand::thread_rng();
    let dialogue_select = rng.gen_range(0..1);
    if dialogue_select == 0 {
        if relation < 0 {
            return "Prepare to be dissected!".to_string();
        } else if relation > 0 && relation <= 25 {
            return "Get out of my sight, lunatic...".to_string();
        } else if relation > 25 && relation <= 50 {
            return "It's a good day in Terrant...".to_string();
        } else if relation > 50 && relation <= 75 {
            return "I'm always happy to help you!".to_string();
        } else if relation > 75 && relation <= 100 {
            return "We should erect a monument for you!".to_string();
        }
    } else if dialogue_select == 1 {
    }
    "Klack klack...".to_string()
}
pub fn get_skill_descriptions() -> HashMap<String, SkillDesc> {
    HashMap::from([
        (
            "meteoroid".to_string(),
            SkillDesc {
                title: "Plasma Drop".to_string(),
                text: "Drop meteors".to_string(),
            },
        ),
        (
            "hibernate".to_string(),
            SkillDesc {
                title: "Soothe".to_string(),
                text: "Sing a song to sleep enemies".to_string(),
            },
        ),
        (
            "slow".to_string(),
            SkillDesc {
                title: "Unholy grab".to_string(),
                text: "Slow".to_string(),
            },
        ),
    ])
}
