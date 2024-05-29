

use macroquad::prelude::*;
mod collision;
mod background;
use background::draw_grid;
use collision::{collision_left, collision_right, collision_bottom, collision_top};


use macroquad::audio::{load_sound, play_sound_once, Sound};

const w: i32 = 3000;
const H: i32 = 1080;



 

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






fn draw_player(player_position: Vec2, player_texture: &Texture2D) {
    let width = player_texture.width();
    let height = player_texture.height() ;

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
    let mut texture: Texture2D = load_texture("assets/princess-side.png").await.unwrap();
    let mut texture2: Texture2D = load_texture("assets/princess-side1.png").await.unwrap();
    let mut dir: bool = false; 
    let bullet_texture: Texture2D = load_texture("assets/arrow.png").await.unwrap();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut game_state = GameState::StartScreen;
    let block_texture: Texture2D = load_texture("assets/block-up.png").await.unwrap();
    let shoot_sound: Sound = load_sound("assets/gun-gunshot-02.wav").await.unwrap();
    let mut last_shot_time = get_time();
    let gun: Texture2D = load_texture("assets/gun.png").await.unwrap();
    let gun2: Texture2D = load_texture("assets/gun-flipped.png").await.unwrap();

    let player_size: Vec2 = vec2(64., 96.);
    let object: Vec2 = vec2(screen_width() * 0.9, screen_height() * 0.);
    let object_size: Vec2 = vec2(screen_width() / 5., screen_height());
    let object2: Vec2 = vec2(screen_width() * 0., screen_height() * 0.8);
    let object_size2: Vec2 = vec2(screen_width(), screen_height() / 5.); 
    let mut mirror = false;

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
                    let mut mouse_position:Vec2 = mouse_position().into();
                    draw_grid(&bullet_texture);

        if(dir == false){
            draw_player(player_position, &texture);
                   

            let radius = 50.0; // The radius of the circle around which the texture rotates

            // Calculate the angle to the mouse as before
            let diff = mouse_position - vec2(player_position.x, player_position.y);
            let angle = diff.y.atan2(diff.x);

            // Calculate the position of the texture
            let texture_position = player_position - vec2(player_size.x, 0.) + vec2(radius * angle.cos(), radius * angle.sin());

            // Draw the texture at the calculated position, rotated by the angle
            draw_texture_ex(
                &gun,
                texture_position.x,
                texture_position.y,
                WHITE,
                DrawTextureParams {
                    
                    flip_y : mirror,
                    
                    rotation: angle,
                    ..Default::default()
                },
            );
            

        }else if(dir == true){
            draw_player(player_position, &texture2);        

            let radius = 50.0; // The radius of the circle around which the texture rotates

            // Calculate the angle to the mouse as before
            let diff = mouse_position - vec2(player_position.x, player_position.y);
            let angle = diff.y.atan2(diff.x);

            // Calculate the position of the texture
            let texture_position = player_position - vec2(20., 0.) + vec2(radius * angle.cos(), radius * angle.sin());

            // Draw the texture at the calculated position, rotated by the angle
            draw_texture_ex(
                &gun,
                texture_position.x,
                texture_position.y,
                WHITE,
                DrawTextureParams {
                    rotation: angle,
                    ..Default::default()
                },
            );
            

        }
        

   










      

        if(player_position[1] < 1030. && !collision_bottom(&mut player_position, object2, object_size2, player_size)){
              player_position[1] += 4.;
        }
    

        if player_position.x < mouse_position.x{
            dir = true;
            mirror = true;
            
        }else{
            dir = false;
        }

        if is_key_down(KeyCode::D)&& !collision_right(&mut player_position, object2, object_size2, player_size) && !collision_right(&mut player_position, object, object_size, player_size){
            player_position.x += 5.0;
            
        }
        if is_key_down(KeyCode::A) && !collision_left(&mut player_position, object2, object_size2, player_size){
            player_position.x -= 5.0;
            
        }
        draw_rectangle(object2[0], object2[1], object_size2[0], object_size2[1], BLUE);
        draw_rectangle(object.x, object.y, object_size.x, object_size.y, BLUE);

        

        let current_time = get_time();
        if is_mouse_button_down(MouseButton::Left) && current_time - last_shot_time >= 0.2 {
            let diff = mouse_position - vec2(player_position.x, player_position.y);
            let angle = diff.y.atan2(diff.x);
            let gun_length = 100.0; // Replace with the actual length of your gun texture
            let gun_angle = angle; // The angle you calculated earlier

            let bullet_start_position = player_position + vec2(gun_length * gun_angle.cos(), gun_length * gun_angle.sin());
            
            let mouse_pos: Vec2 = mouse_position.into();
            let bullet = Bullet::new(bullet_start_position, (mouse_pos - bullet_start_position).normalize() * 10.0);
            bullets.push(bullet);
            play_sound_once(&shoot_sound);

            // Update the time of the last shot
            last_shot_time = current_time;
        }
        
        
        for bullet in bullets.iter_mut() {
            if bullet.pos[0] < 0. || bullet.pos[0] > screen_width() || bullet.pos[1] < 0. || bullet.pos[1] > screen_height() {
                bullet.pos = vec2(-100., -100.);
                
            }
        }
        
        for bullet in bullets.iter_mut() {
            bullet.update();
            bullet.draw(&bullet_texture);
            println!("{:?}", bullet.pos);
        }
   

     

        delay_jump += 1.;
        
        
  
        handle_jump(&mut jump_state, &mut jump_sec, &mut player_position, &mut delay_jump);
       

  
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 30., 32., WHITE);
    }
   

}
        next_frame().await
    }

}   
