// Window
pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 700.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;

pub const TILE_WIDTH: u32 = 32;
pub const TILE_HEIGHT: u32 = 32;

pub const SPRITE_SHEET_WIDTH: usize = 4;
pub const SPRITE_SHEET_HEIGHT: usize = 4;

// World
pub const WORLD_WIDTH: f32 = 4000.0;
pub const WORLD_HEIGHT: f32 = 4000.0;
pub const NUM_WORLD_DECORATION: u16 = 1000;

// Player
pub const PLAYER_SPEED: f32 = 2.0;

// Gun
pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;
pub const BULLET_SPEED: f32 = 4.0;
pub const BULLET_LIFETIME: f32 = 5.0; 

// Colors
pub const BG_COLOR: (f32, f32, f32) = (0.196, 0.329, 0.192);

//ENEMIES
pub const NUM_ENEMIES: usize = 1000;
pub const ENEMIES_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.3;