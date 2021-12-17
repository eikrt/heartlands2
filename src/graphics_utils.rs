use std::collections::HashMap;
use sdl2::pixels::Color;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub speed: f32,
}
pub struct TileGraphics {
    pub sc: Color,
    pub tc: Color
}
pub fn tile_graphics() -> HashMap<String, TileGraphics>{
return HashMap::from([
    ("grass".to_string(),
    TileGraphics {

       sc: Color::RGB(128,64,55),
       tc: Color::RGB(128,128,55)
    }),

    ("water".to_string(),
    TileGraphics {
       sc: Color::RGB(0,20,55),
       tc: Color::RGB(0,20,255)
    }),

    ("ice".to_string(),
    TileGraphics {
       sc: Color::RGB(128,64,55),
       tc: Color::RGB(128,128,55)
    }),

    ("permafrost".to_string(),
    TileGraphics {
       sc: Color::RGB(128,64,55),
       tc: Color::RGB(128,128,55)
    }),

    ("coarse_land".to_string(),
    TileGraphics {
       sc: Color::RGB(128,64,55),
       tc: Color::RGB(128,128,55)
    }),
    ("savannah_land".to_string(),
    TileGraphics {
       sc: Color::RGB(128,64,55),
       tc: Color::RGB(128,128,55)
    }),

    ("sand".to_string(),
    TileGraphics {
       sc: Color::RGB(128,64,55),
       tc: Color::RGB(128,128,55)
    }),
    ("red_sand".to_string(),
    TileGraphics {
       sc: Color::RGB(128,64,55),
       tc: Color::RGB(128,128,55)
    })]);
}
