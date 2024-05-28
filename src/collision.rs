use macroquad::prelude::*;   
pub fn collision_left(player_position: &mut Vec2, object_position: Vec2, size: Vec2, player_size: Vec2) -> bool {
        
    let player_pos_x = player_position.x;
    let player_pos_y = player_position.y;
    
    let object_size_x = size.x;
    
    let object_pos_y = object_position.y;
    let object_pos_x = object_position.x + object_size_x ;

    

    if(player_pos_x < object_pos_x && !{player_pos_y < object_pos_y} && !{player_pos_x < object_position.x}){
        return true
    }else{
        return false
    }

}


pub fn collision_right(player_position: &mut Vec2, object_position: Vec2, size: Vec2, player_size: Vec2) -> bool {
    let player_pos_x = player_position.x + player_size.x;
    let player_pos_y = player_position.y;
    let object_pos_x = object_position.x;
    let object_pos_y = object_position.y;
    if player_pos_x > object_pos_x && !{player_pos_y < object_pos_y} && !{player_pos_x > object_pos_x + size.x} {
        return true


    }else{
        return false
    }
}