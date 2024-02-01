mod game_state;
mod renderer;
mod consts;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;
use crate::consts::{DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};
use crate::game_state::GameContext;
use crate::renderer::Renderer;



pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Snake Game",
                GRID_X_SIZE * DOT_SIZE_IN_PXS,
                GRID_Y_SIZE * DOT_SIZE_IN_PXS)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut context = GameContext::new();
    let mut renderer = Renderer::new(window).expect("test");

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

       renderer.draw(&context);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}