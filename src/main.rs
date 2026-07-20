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

    //y
    framebuffer.set_pixel(28, 54);
    framebuffer.set_pixel(29, 55);
    framebuffer.set_pixel(30, 56);
    framebuffer.set_pixel(31, 55);
    framebuffer.set_pixel(31, 57);
    framebuffer.set_pixel(32, 54);
    framebuffer.set_pixel(32, 58);

    framebuffer.set_pixel(69, 58);
    framebuffer.set_pixel(69, 54);
    framebuffer.set_pixel(70, 57);
    framebuffer.set_pixel(70, 55);
    framebuffer.set_pixel(71, 56);
    framebuffer.set_pixel(72, 55);
    framebuffer.set_pixel(73, 54);

    //l
    for x in 34..=36 {
        framebuffer.set_pixel(x, 58);
    }

    for x in 65..=67 {
        framebuffer.set_pixel(x, 58);
    }

    for y in 54..=57 {
        framebuffer.set_pixel(36, y);

        framebuffer.set_pixel(65, y);
    }

    //i
    for x in 38..=40 {
        framebuffer.set_pixel(x, 54);
        framebuffer.set_pixel(x, 58);
    }

    for x in 61..=63 {
        framebuffer.set_pixel(x, 54);
        framebuffer.set_pixel(x, 58);
    }

    for y in 55..=57 {
        framebuffer.set_pixel(39, y);

        framebuffer.set_pixel(62, y);
    }

    //m
    for y in 54..=58 {
        framebuffer.set_pixel(42, y);
        framebuffer.set_pixel(46, y);

        framebuffer.set_pixel(55, y);
        framebuffer.set_pixel(59, y);
    }

    framebuffer.set_pixel(43, 55);
    framebuffer.set_pixel(44, 56);
    framebuffer.set_pixel(45, 55);

    framebuffer.set_pixel(56, 55);
    framebuffer.set_pixel(57, 56);
    framebuffer.set_pixel(58, 55);

    //e
    for x in 48..=53 {
        framebuffer.set_pixel(x, 54);
        framebuffer.set_pixel(x, 58);
    }

    for x in 50..=51 {
        framebuffer.set_pixel(x, 55);
        framebuffer.set_pixel(x, 57);
    }
    for x in 49..=52 {
        framebuffer.set_pixel(x, 56);
    }

    //nave
    for x in 48..=53 {
        framebuffer.set_pixel(x, 63);
    }
    for x in 50..=51 {
        framebuffer.set_pixel(x, 62);
    }
    framebuffer.set_pixel(49, 64);
    framebuffer.set_pixel(52, 64);
    framebuffer.set_pixel(49, 61);
    framebuffer.set_pixel(52, 61);

    //corazon
    for x in 46..=48 {
        framebuffer.set_pixel(x, 50);
    }
    for x in 53..=55 {
        framebuffer.set_pixel(x, 50);
    }
    framebuffer.set_pixel(47, 48);
    framebuffer.set_pixel(54, 48);
    framebuffer.set_pixel(48, 49);
    framebuffer.set_pixel(55, 49);
    framebuffer.set_pixel(47, 51);
    framebuffer.set_pixel(54, 51);
}

fn main() {
    let window_width = 800;
    let window_height = 800;
    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(
        framebuffer_width,
        framebuffer_height,
        Color::BLACK,
    );

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    initial_pattern(&mut framebuffer);
    framebuffer.swap_buffers(&mut window, &raylib_thread, window_width, window_height);

    while !window.window_should_close() {
        thread::sleep(Duration::from_millis(1000));

        render(&mut framebuffer, framebuffer_width, framebuffer_height);

        framebuffer.swap_buffers(&mut window, &raylib_thread, window_width, window_height);
    }
}
