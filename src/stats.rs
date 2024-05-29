use macroquad::prelude::*;


struct PlayerStats {
    health: i32,
    bullet_count: i32,
    position: Vec2,
    texture: Texture2D,
}

impl PlayerStats {
    fn new(health: i32, bullet_count: i32, position: Vec2, texture: Texture2D) -> PlayerStats {
        PlayerStats {
            health,
            bullet_count,
            position,
            texture,
        }
    }
}
