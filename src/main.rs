use std::io::{self, Write};
use std::time::Instant;
use termion::{clear, cursor};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::world::World;
pub mod cell;
pub mod world;

const CELL_PIXEL_SIDE: u32 = 5;
const X_SIZE: u32 = CELL_PIXEL_SIDE * 480;
const Y_SIZE: u32 = CELL_PIXEL_SIDE * 200;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let (cols, rows) = (X_SIZE / CELL_PIXEL_SIDE, Y_SIZE / CELL_PIXEL_SIDE);
    let mut world = World::new_random(cols as usize, rows as usize);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game of Life", X_SIZE, Y_SIZE)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    'event_loop: loop {
        let now = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Space => running = !running,
                    Keycode::C => world = world.clear(),
                    Keycode::R => world = world.random(),
                    Keycode::T => world = world.tick(),
                    _ => {}
                },
                _ => {}
            }
        }

        world.cells.iter().enumerate().for_each(|(index, cell)| {
            let x = index % world.width;
            let y = index / world.width;
            let px = x as u32 * CELL_PIXEL_SIDE;
            let py = y as u32 * CELL_PIXEL_SIDE;
            let color = if cell.alive {
                Color::RGB(255, 255, 255)
            } else {
                Color::RGB(0, 0, 0)
            };
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(
                    px as i32,
                    py as i32,
                    CELL_PIXEL_SIDE,
                    CELL_PIXEL_SIDE,
                ))
                .unwrap();
        });

        canvas.present();

        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        println!("{}", world);
        println!("Render: {}ms", now.elapsed().as_millis());
        println!("World Tick: {}ms", world.tick_time_ms);

        if running {
            world = world.tick();
        }
    }
}
