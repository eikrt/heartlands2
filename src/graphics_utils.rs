use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::{Point,Rect};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::image::{LoadTexture, InitFlag};
use crate::world_structs;
#[derive(PartialEq)]

pub enum MoveDirection {
    UP,
    LEFT,
    DOWN,
    RIGHT,
    ZOOMIN,
    ZOOMOUT
}
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub zoom_speed: f32,
    pub move_speed: f32,
}
impl Camera {
    pub fn zoom(&mut self, dir: MoveDirection) { // + is zoom, - is negative zoom
        if dir == MoveDirection::ZOOMIN {
            self.zoom += self.zoom_speed;
        }
        else if dir == MoveDirection::ZOOMOUT {
            self.zoom -= self.zoom_speed;
        }
    }
    pub fn mov(&mut self, dir: MoveDirection) { // 0 = up, 1 = left, 2 = down, 3 = right
        if dir == MoveDirection::UP {
            self.y -= self.move_speed;
        }
        else if dir == MoveDirection::LEFT {
            self.x -= self.move_speed;
        }

        else if dir == MoveDirection::DOWN {
            self.y += self.move_speed;
        }
        else if dir == MoveDirection::RIGHT {
            self.x += self.move_speed;
        }
    }
}
pub struct TileGraphics {
    pub sc: Color,
    pub tc: Color
}

pub struct EntityGraphics {
    pub src: String,
    pub color: Color,
    pub position: Point,
    pub sprite: Rect,
}

    pub fn render(canvas: &mut WindowCanvas, texture: &Texture, position: Point, sprite: Rect) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let screen_rect = Rect::from_center(position, sprite.width(), sprite.height());
        canvas.copy(texture, sprite, screen_rect)?;

        
        Ok(())
    }
pub fn tile_graphics() -> HashMap<world_structs::TileType, TileGraphics>{
return HashMap::from([
    (world_structs::TileType::GRASS ,
    TileGraphics {

       sc: Color::RGB(58,132,56),
       tc: Color::RGB(5,85,5)
    }),

    (world_structs::TileType::COLD_LAND ,
    TileGraphics {

       sc: Color::RGB(58,132,56),
       tc: Color::RGB(5,85,5)
    }),
    (world_structs::TileType::WATER ,
    TileGraphics {
       sc: Color::RGB(65,65,195),
       tc: Color::RGB(17,17,87)
    }),

    (world_structs::TileType::ICE ,
    TileGraphics {
       sc: Color::RGB(255,255,255),
       tc: Color::RGB(200,200,250)
    }),

    (world_structs::TileType::PERMAFROST,
    TileGraphics {
       sc: Color::RGB(58,125,50),
       tc: Color::RGB(95,110,95)
    }),

    (world_structs::TileType::COARSE_LAND,
    TileGraphics {
       sc: Color::RGB(150,145,105),
       tc: Color::RGB(90,85,45)
    }),
    (world_structs::TileType::SAVANNAH_LAND,
    TileGraphics {
       sc: Color::RGB(186,165,80),
       tc: Color::RGB(150,150,105)
    }),

    (world_structs::TileType::SAND,
    TileGraphics {
       sc: Color::RGB(255,247,56),
       tc: Color::RGB(170,165,90)
    }),
    (world_structs::TileType::RED_SAND,
    TileGraphics {
       sc: Color::RGB(190,130,80),
       tc: Color::RGB(150,90,35)
    }),
    (world_structs::TileType::MUD_HIVE_WALL,
    TileGraphics {
       sc: Color::RGB(90,90,60),
       tc: Color::RGB(90,90,60)
    }),
    (world_structs::TileType::MUD_HIVE_FLOOR,
    TileGraphics {
       sc: Color::RGB(120,120,75),
       tc: Color::RGB(120,120,75)
    }),

    (world_structs::TileType::STONE_HIVE_WALL,
    TileGraphics {
       sc: Color::RGB(110,110,110),
       tc: Color::RGB(110,110,110)
    }),
    (world_structs::TileType::STONE_HIVE_FLOOR,
    TileGraphics {
       sc: Color::RGB(50,50,50),
       tc: Color::RGB(50,50,50)
    })
]);
}
