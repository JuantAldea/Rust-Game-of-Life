use self::piston_window::*;
use std::io::{self, Write};
use std::time::Instant;
use termion::{clear, cursor};

extern crate image as im;
extern crate piston_window;

use crate::world::World;
pub mod cell;
pub mod world;

const UPS: u32 = 120;
const CELL_PIXEL_SIDE: u32 = 5;
const X_SIZE: u32 = CELL_PIXEL_SIDE * 100;
const Y_SIZE: u32 = CELL_PIXEL_SIDE * 100;

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let (cols, rows) = (X_SIZE / CELL_PIXEL_SIDE, Y_SIZE / CELL_PIXEL_SIDE);
    let mut world = World::new_random(cols as usize, rows as usize);

    let mut window: PistonWindow = WindowSettings::new("Game of Life", [X_SIZE, Y_SIZE])
        .resizable(false)
        .exit_on_esc(true)
        .graphics_api(OpenGL::V3_2)
        .fullscreen(false)
        .build()
        .unwrap();

    window.events = Events::new(EventSettings {
        max_fps: 240,
        ups: UPS as u64,
        ups_reset: 2,
        swap_buffers: true,
        lazy: false,
        bench_mode: false,
    });

    let mut canvas = im::ImageBuffer::new(X_SIZE as u32, Y_SIZE as u32);
    let mut ctx = window.create_texture_context();

    let mut texture = Texture::from_image(&mut ctx, &canvas, &TextureSettings::new()).unwrap();

    let mut running = true;
    let mut cursor = [0, 0];
    while let Some(e) = window.next() {
        e.mouse_cursor(|[x, y]| {
            cursor = [x as u32, y as u32];
            //println!("Cursor event: {:?}", cursor);
        });

        if let Some(btn) = e.press_args() {
            match btn {
                Button::Mouse(MouseButton::Left) => {
                    let (_x, _y) = (cursor[0] / CELL_PIXEL_SIDE, cursor[1] / CELL_PIXEL_SIDE);
                    //world = world.toggle(x as usize, y as usize);
                }
                Button::Keyboard(Key::Space) => running = !running,
                Button::Keyboard(Key::C) => world = world.clear(),
                Button::Keyboard(Key::R) => world = world.random(),
                Button::Keyboard(Key::T) => world = world.tick(),
                _ => {}
            };
        }

        if e.render_args().is_some() {
            let _pixel_to_point = |sx: u32, sy: u32| -> (usize, usize) {
                (
                    ((sx / CELL_PIXEL_SIDE) as usize),
                    (sy / CELL_PIXEL_SIDE) as usize,
                )
            };

            let _now = Instant::now();
            world.cells.iter().enumerate().for_each(|cell| {
                let x = cell.0 % world.width;
                let y = cell.0 / world.width;
                let color = if cell.1.alive { [255; 4] } else { [128; 4] };

                for j in 0..CELL_PIXEL_SIDE {
                    let sy = y as u32 * CELL_PIXEL_SIDE + j;
                    for i in 0..CELL_PIXEL_SIDE {
                        let sx = x as u32 * CELL_PIXEL_SIDE + i;
                        canvas.put_pixel(sx, sy, im::Rgba(color));
                    }
                }
            });

            texture.update(&mut ctx, &canvas).unwrap();
            window.draw_2d(&e, |c, g, d| {
                ctx.encoder.flush(d);
                clear([0., 0., 0., 1.], g);
                image(&texture, c.transform, g);
            });

            write!(stdout, "{}", clear::All).unwrap();
            write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
            println!("{}", world);
            println!("Render: {}ms", _now.elapsed().as_millis());
            println!("World Tick: {}ms", world.tick_time_ms);
        }

        if e.update_args().is_some() && running {
            world = world.tick();
        }
    }
}

/*
fn main() {
    let mut world = world::World::new_random(5, 5);
    loop {
        print!("{}", world);
        world.tick_mutable();

    }

    print!("{}", world);
    print!("{}", world.tick());
    print!("{}", world.tick().tick());
    print!("{}", world.tick().tick().tick());
    print!("--------------------------------");
    print!("--------------------------------");
    print!("--------------------------------");
    print!("{}", world);
    world.tick_mutable();
    print!("{}", world);
    world.tick_mutable();
    print!("{}", world);
    print!("{}", world.tick_mutable());
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}
*/
