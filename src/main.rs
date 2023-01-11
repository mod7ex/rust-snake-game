extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::{ Button, types::Color, PistonWindow, WindowSettings, PressEvent, clear, UpdateEvent };

use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [draw::to_coord_u32(width), draw::to_coord_u32(height)],
    )
    .exit_on_esc(true)
    .resizable(true)
    .build()
    .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |context, graphics, _| {
            clear(BACK_COLOR, graphics);
            game.draw(&context, graphics);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
