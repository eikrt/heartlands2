use crate::graphics_utils::{Camera, MoveDirection};
use crate::world_structs::{ActionType, CategoryType, EntityType, ItemType, TaskType};
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub hp: i32,
    pub speed: f32,
    pub dir: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub stopped: bool,
    pub id: i32,
    pub entity_type: EntityType,
    pub category_type: CategoryType,
    pub faction: String,
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
        } else if dir == MoveDirection::Down {
            self.y += self.speed * delta as f32 / 100.0;
        } else if dir == MoveDirection::Right {
            self.x += self.speed * delta as f32 / 100.0;
        }
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
