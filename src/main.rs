// main.rs

mod framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use std::{thread, time::Duration};

fn is_alive(framebuffer: &mut Framebuffer, x: i32, y: i32) -> bool {
    let color = framebuffer.get_color(x, y);
    color == Color::BLUE || color == Color::GREEN
}

fn render(
    framebuffer: &mut Framebuffer,
    width: u32,
    height: u32,
) {
    let mut next_generation = Vec::new();

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut neighbors = 0;

            for offset_y in -1..=1 {
                for offset_x in -1..=1 {
                    if (offset_x != 0 || offset_y != 0)
                        && is_alive(framebuffer, x + offset_x, y + offset_y)
                    {
                        neighbors += 1;
                    }
                }
            }

            let alive = is_alive(framebuffer, x, y);

            if !alive && neighbors == 3 {
                next_generation.push(Color::GREEN);
            } else if alive && (neighbors == 2 || neighbors == 3) {
                next_generation.push(Color::BLUE);
            } else {
                next_generation.push(Color::BLACK);
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            framebuffer.set_current_color(next_generation[(y * width + x) as usize]);
            framebuffer.set_pixel(x, y);
        }
    }
}

fn initial_pattern(framebuffer: &mut Framebuffer) {
    framebuffer.set_current_color(Color::GREEN);

    // Agrega tus células iniciales aquí con: framebuffer.set_pixel(x, y);
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        window_width as u32,
        window_height as u32,
        Color::BLACK,
    );

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    initial_pattern(&mut framebuffer);

    while !window.window_should_close() {
        render(&mut framebuffer, window_width as u32, window_height as u32);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
