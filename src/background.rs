use macroquad::prelude::*;


const w: i32 = 3000;
const H: i32 = 1080;

pub fn draw_grid(new_texture: &Texture2D) {
    
    
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