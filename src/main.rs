

use macroquad::prelude::*;
mod collision;
use collision::{collision_left, collision_right};


use macroquad::audio::{load_sound, play_sound_once, Sound};

const w: i32 = 1920;
const H: i32 = 1080;


fn collision_top(player_position: &mut Vec2, object_position: Vec2, size: Vec2, player_size: f32) -> bool {
    let player_top_edge = player_position.y;
    let object_bottom_edge = object_position.y + size.y;

    let player_left = player_position.x;
    let player_right = player_position.x + player_size;
    let object_left = object_position.x;
    let object_right = object_position.x + size.x;

    if player_top_edge < object_bottom_edge && player_top_edge > object_position.y &&
        player_right > object_left && player_left < object_right {
        // Collision detected, player is colliding with the bottom side of the object
        true
    } else {
        // No collision
        false
    }
}

fn collision_bottom(player_position: &mut Vec2, object_position: Vec2, size: Vec2, player_size: Vec2) -> bool {
    let player_pos_y = player_position.y + player_size.y;
    let palayer_pos_x = player_position.x;
    let object_pos_y = object_position.y;
    let object_size_x = size.x;
    let object_pos_x = object_position.x;


    if player_pos_y >= object_pos_y && !{palayer_pos_x < object_pos_x} && !{palayer_pos_x > object_pos_x + object_size_x}{
        // Collision detected, player is colliding with the top side of the object
        true
    } else {
        // No collision
        false
    }
}
 

fn handle_jump(jump_state: &mut bool, jump_sec: &mut f32, player_position: &mut Vec2, delay_jump: &mut f32) {
    if is_key_pressed(KeyCode::Space) && player_position.y > -1.0 && *delay_jump > 47.0 {
        *jump_state = true;
        *delay_jump = 0.0;
    }

    if *jump_state {
        *jump_sec += 1.0;
        player_position.y -= 20.0;
    }

    if *jump_sec > 10.0 {
        *jump_state = false;
        *jump_sec = 0.0;
    }
}






fn draw_grid(new_texture: &Texture2D) {
    
    
    let block_size = 30.0;
    let num_blocks_x = (w as f32 / block_size) as i32;
    let num_blocks_y = (H as f32 / block_size) as i32;

    for i in 0..num_blocks_x {
        for j in 0..num_blocks_y {
            let x = i as f32 * block_size;
            let y = j as f32 * block_size;
            
            
            draw_texture(&new_texture, x, y,  WHITE);
        }
    }
}

fn draw_player(player_position: Vec2, player_texture: &Texture2D) {
    let width = player_texture.width();
    let height = player_texture.height();

    // Adjust the position to center the texture
    let x = player_position.x - width / 2.0;
    let y = player_position.y - height / 2.0;

    draw_texture(player_texture, x, y, WHITE);
}


fn conf() -> Conf {
    Conf {
        window_title: String::from("peer game"),
        window_width: w,
        window_height: H,
        fullscreen: false,
        ..Default::default()
    }
}

enum GameState {
    SettingsScreen,
    StartScreen,
    Gameplay,
}


struct Bullet {
    pos: Vec2,
    velocity: Vec2,
}
impl Bullet {
    fn new(pos: Vec2, velocity: Vec2) -> Bullet {
        Bullet { pos, velocity }
    }
    fn update(&mut self) {
        self.pos += self.velocity;
    }
    fn draw(&self,texture: &Texture2D) {
        
        draw_texture_ex(&texture, self.pos[0], self.pos[1], WHITE, DrawTextureParams {
            dest_size: Some(vec2( 15., 5.0)),
            rotation: (self.velocity.y / self.velocity.x).atan(),
            ..Default::default()
        });
        

    }
}


#[macroquad::main(conf)]
async fn main() {


    


    
    let mut player_position = vec2(500., 2.);
    let mut jump_sec: f32 = 0.;
    let mut jump_state: bool = false;
    let mut delay_jump: f32 = 0.;
    let mut texture: Texture2D = load_texture("/home/peer/world/code/assets/princess-side.png").await.unwrap();
    let mut texture2: Texture2D = load_texture("/home/peer/world/code/assets/princess-side1.png").await.unwrap();
    let mut dir: bool = false; 
    let bullet_texture: Texture2D = load_texture("/home/peer/world/code/assets/arrow.png").await.unwrap();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut game_state = GameState::StartScreen;
    let block_texture: Texture2D = load_texture("/home/peer/world/code/assets/block-up.png").await.unwrap();
    let shoot_sound: Sound = load_sound("/home/peer/world/code/assets/gun-gunshot-02.wav").await.unwrap();
    let mut last_shot_time = get_time();
    
  
    let player_size: Vec2 = vec2(64., 96.);
    let object2: Vec2 = vec2(500., 1000.);
    let object_size2: Vec2 = vec2(300., 400.); 

    loop {
        
            match game_state {
                GameState::SettingsScreen =>{
                    draw_text("Settings", screen_width() / 2. - 100., screen_height() / 2., 40., BLACK);
                    if is_key_pressed(KeyCode::Escape) {
                        game_state = GameState::StartScreen;
                    }
                }
                GameState::StartScreen => {
                    clear_background(WHITE);
                    draw_text("Press SPACE to start", screen_width() / 2. - 100., screen_height() / 2., 40., BLACK);
        
                    if is_key_pressed(KeyCode::Space) {
                        game_state = GameState::Gameplay;
                    }
                }
                GameState::Gameplay => {
        draw_grid(&bullet_texture);
        let mut mouse_position:Vec2 = mouse_position().into();


        

    



        

     


        

        if(dir == false){
            draw_texture(&texture, player_position[0], player_position[1], WHITE);
        
        }else if(dir == true){
            draw_texture(&texture2, player_position[0], player_position[1], WHITE);

        }







        if(player_position[1] < 1030. && !collision_bottom(&mut player_position, object2, object_size2, player_size)){
              player_position[1] += 4.;
        }
    



        if is_key_down(KeyCode::D)&& !collision_right(&mut player_position, object2, object_size2, player_size){
            player_position.x += 5.0;
            dir = true;
        }
        if is_key_down(KeyCode::A) && !collision_left(&mut player_position, object2, object_size2, player_size){
            player_position.x -= 5.0;
            dir = false;
        }
        draw_rectangle(object2[0], object2[1], object_size2[0], object_size2[1], BLUE);

        

        let current_time = get_time();
        if is_mouse_button_down(MouseButton::Left) && current_time - last_shot_time >= 0.2 {
            let player_pos = vec2(player_position[0], player_position[1]); // Define player position
            let mouse_pos: Vec2 = mouse_position.into();
            let bullet = Bullet::new(player_pos, (mouse_pos - player_pos).normalize() * 5.0);
            bullets.push(bullet);
            play_sound_once(&shoot_sound);

            // Update the time of the last shot
            last_shot_time = current_time;
        }
        
        
        for bullet in bullets.iter_mut() {
            if bullet.pos[0] < 0. || bullet.pos[0] > 1920. || bullet.pos[1] < 0. || bullet.pos[1] > 1080. {
                bullet.pos = vec2(-100., -100.);
            }
        }
        
        for bullet in bullets.iter_mut() {
            bullet.update();
            bullet.draw(&bullet_texture);
        }

     

        delay_jump += 1.;
        
        
  
        handle_jump(&mut jump_state, &mut jump_sec, &mut player_position, &mut delay_jump);
       

  
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 30., 32., WHITE);
    }
   

}
        next_frame().await
    }

}   
