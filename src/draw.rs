use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, context: &Context, graphics: &mut G2d) {
    rectangle(
        color,
        [
            to_coord(x),
            to_coord(y),
            to_coord(width),
            to_coord(height),
        ],
        context.transform,
        graphics,
    );
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, graphics: &mut G2d) {
    draw_rectangle(color, x, y,  1, 1, context, graphics);
}

