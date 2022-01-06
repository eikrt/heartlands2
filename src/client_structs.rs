use crate::graphics_utils::{Camera, MoveDirection};
use crate::world_structs::{
    ActionType, CategoryType, EntityType, ItemType, ReligionType, TaskType,
};
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub energy: i32,
    pub speed: f32,
    pub dir: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub stopped: bool,
    pub id: i32,
    pub entity_type: EntityType,
    pub category_type: CategoryType,
    pub faction: String,
    pub religion_type: ReligionType,
    pub faction_id: i32,
    pub current_action: ActionType,
    pub task_type: TaskType,
    pub wielding_item: ItemType,
    pub backpack_item: ItemType,
    pub wearable_item: ItemType,
    pub backpack_amount: u8,
    pub time: u128,
}
impl Player {
    pub fn mov(&mut self, dir: MoveDirection, delta: u128) {
        if dir == MoveDirection::Up {
            self.y -= self.speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Left {
            self.x -= self.speed * delta as f32 / 100.0;
            self.dir = std::f64::consts::PI as f32 * (2.0);
        } else if dir == MoveDirection::Down {
            self.y += self.speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Right {
            self.x += self.speed * delta as f32 / 100.0;
            self.dir = std::f64::consts::PI as f32 / 2.0;
        }
        if dir == MoveDirection::Nothing {
            self.stopped = true;
        } else {
            self.stopped = false;
        }
    }
    pub fn tick(&mut self, delta: u128) {
        self.time += 10;
    }
    pub fn get_relative_x(&self, camera: &Camera) -> f32 {
        return self.x - camera.x;
    }
    pub fn get_relative_y(&self, camera: &Camera) -> f32 {
        return self.y - camera.y;
    }
}
pub struct ClientPacket {
    camera: Camera,
    player: Player,
}
