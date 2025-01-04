use bevy::color::Color;

pub const INITIAL_WIDTH: f32 = 1280.;
pub const INITIAL_HEIGHT: f32 = 720.;

pub const WORLD_COLOR: Color = Color::linear_rgb(0.03, 0.03, 0.1);
pub const PLAYER_HEAD_COLOR: Color = Color::linear_rgb(0.3, 0.3, 0.9);
pub const PLAYER_BODY_COLOR: Color = Color::linear_rgb(0.2, 0.2, 0.7);
pub const ENEMY_HEAD_COLOR: Color = Color::linear_rgb(0.7, 0.7, 0.7);
pub const ENEMY_BODY_COLOR: Color = Color::linear_rgb(0.3, 0.3, 0.3);
pub const FOOD_COLOR: Color = Color::linear_rgb(1.0, 0.0, 1.0);

pub const HEAD_SIZE: f32 = 0.8;
pub const BODY_SIZE: f32 = 0.6;
pub const FOOD_SIZE: f32 = 0.8;

pub const ARENA_WIDTH: u32 = 56;
pub const ARENA_HEIGHT: u32 = 28;

pub const SNAKE_SPEED: u64 = 10;

// [上, 右, 下, 左]
pub const DIRECTION_WEIGHT: [i32; 4] = [60, 20, 0, 20];

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn in_arena(x: i32, y: i32) -> bool {
    0 <= x && x < ARENA_WIDTH as i32 && 0 <= y && y < ARENA_HEIGHT as i32
}

pub fn in_enemy_arena(x: i32, y: i32) -> bool {
    -(ARENA_WIDTH as i32 / 2) <= x
        && x < 3 * ARENA_WIDTH as i32 / 2
        && -(ARENA_HEIGHT as i32 / 2) <= y
        && y < 3 * ARENA_HEIGHT as i32 / 2
}
