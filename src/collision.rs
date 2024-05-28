use macroquad::prelude::*;   
pub fn collision_left(player_position: &mut Vec2, object_position: Vec2, size: Vec2, player_size: Vec2) -> bool {
        
    let player_pos_x = player_position.x;
    let player_pos_y = player_position.y;
    
    let object_size_x = size.x;
    
    let object_pos_y = object_position.y;
    let object_pos_x = object_position.x + object_size_x + player_size.x / 2.;

    

    if(player_pos_x < object_pos_x && !{player_pos_y < object_pos_y} && !{player_pos_x < object_position.x}){
        return true
    }else{
        return false
    }

}


pub fn collision_right(player_position: &mut Vec2, object_position: Vec2, size: Vec2, player_size: Vec2) -> bool {
    let player_pos_x = player_position.x + player_size.x / 2.;
    let player_pos_y = player_position.y;

    
    let object_pos_y = object_position.y;
    let object_pos_x = object_position.x ;

    

    if(player_pos_x > object_pos_x && !{player_pos_y < object_pos_y} && !{player_pos_x > object_position.x + size.x}){
        return true
    }else{
        return false
    }
}
pub fn collision_top(player_position: &mut Vec2, object_position: Vec2, size: Vec2, player_size: f32) -> bool {
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

pub fn collision_bottom(player_position: &mut Vec2, object_position: Vec2, size: Vec2, player_size: Vec2) -> bool {
    let player_pos_y = player_position.y + player_size.y / 2.;
    let palayer_pos_x = player_position.x;
    let object_pos_y = object_position.y;
    let object_size_x = size.x;
    let object_pos_x = object_position.x;


    if player_pos_y > object_pos_y && !{palayer_pos_x < object_pos_x} && !{palayer_pos_x > object_pos_x + object_size_x}{
        // Collision detected, player is colliding with the top side of the object
        true
    } else {
        // No collision
        false
    }
}