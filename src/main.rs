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
    framebuffer.set_pixel(27, 54);
    framebuffer.set_pixel(28, 55);
    framebuffer.set_pixel(29, 56);
    framebuffer.set_pixel(30, 55);
    framebuffer.set_pixel(30, 57);
    framebuffer.set_pixel(31, 54);
    framebuffer.set_pixel(31, 58);

    framebuffer.set_pixel(68, 58);
    framebuffer.set_pixel(68, 54);
    framebuffer.set_pixel(69, 57);
    framebuffer.set_pixel(69, 55);
    framebuffer.set_pixel(70, 56);
    framebuffer.set_pixel(71, 55);
    framebuffer.set_pixel(72, 54);

    //l
    for x in 33..=35 {
        framebuffer.set_pixel(x, 58);
    }

    for x in 64..=66 {
        framebuffer.set_pixel(x, 58);
    }

    for y in 54..=57 {
        framebuffer.set_pixel(35, y);

        framebuffer.set_pixel(64, y);
    }

    //i
    for x in 37..=39 {
        framebuffer.set_pixel(x, 54);
        framebuffer.set_pixel(x, 58);
    }

    for x in 60..=62 {
        framebuffer.set_pixel(x, 54);
        framebuffer.set_pixel(x, 58);
    }

    for y in 55..=57 {
        framebuffer.set_pixel(38, y);

        framebuffer.set_pixel(61, y);
    }

    //m
    for y in 54..=58 {
        framebuffer.set_pixel(41, y);
        framebuffer.set_pixel(45, y);

        framebuffer.set_pixel(54, y);
        framebuffer.set_pixel(58, y);
    }

    framebuffer.set_pixel(42, 55);
    framebuffer.set_pixel(43, 56);
    framebuffer.set_pixel(44, 55);

    framebuffer.set_pixel(55, 55);
    framebuffer.set_pixel(56, 56);
    framebuffer.set_pixel(57, 55);

    //e
    for x in 47..=52 {
        framebuffer.set_pixel(x, 54);
        framebuffer.set_pixel(x, 58);
    }

    for x in 49..=50 {
        framebuffer.set_pixel(x, 55);
        framebuffer.set_pixel(x, 57);
    }
    for x in 48..=51 {
        framebuffer.set_pixel(x, 56);
    }

    //nave
    for x in 47..=52 {
        framebuffer.set_pixel(x, 63);
    }
    for x in 49..=50 {
        framebuffer.set_pixel(x, 62);
    }
    framebuffer.set_pixel(48, 64);
    framebuffer.set_pixel(51, 64);
    framebuffer.set_pixel(48, 61);
    framebuffer.set_pixel(51, 61);

    //corazon
    for x in 45..=47 {
        framebuffer.set_pixel(x, 50);
    }
    for x in 52..=54 {
        framebuffer.set_pixel(x, 50);
    }
    framebuffer.set_pixel(46, 48);
    framebuffer.set_pixel(53, 48);
    framebuffer.set_pixel(47, 49);
    framebuffer.set_pixel(54, 49);
    framebuffer.set_pixel(46, 51);
    framebuffer.set_pixel(53, 51);

    //cara
    for x in 45..=54 {
        framebuffer.set_pixel(x, 45);
    }
    for y in 37..=40 {
        framebuffer.set_pixel(40, y);
        framebuffer.set_pixel(59, y);
    }
    framebuffer.set_pixel(41, 41);
    framebuffer.set_pixel(42, 42);
    framebuffer.set_pixel(43, 43);
    framebuffer.set_pixel(44, 44);
    framebuffer.set_pixel(55, 44);
    framebuffer.set_pixel(56, 43);
    framebuffer.set_pixel(57, 42);
    framebuffer.set_pixel(58, 41);

    for x in 47..=52 {
        framebuffer.set_pixel(x, 42);
    }
    for x in 44..=45 {
        framebuffer.set_pixel(x, 37);
        framebuffer.set_pixel(x, 39);
    }
    for x in 54..=55 {
        framebuffer.set_pixel(x, 37);
        framebuffer.set_pixel(x, 39);
    }

    framebuffer.set_pixel(43, 38);
    framebuffer.set_pixel(46, 38);
    framebuffer.set_pixel(53, 38);
    framebuffer.set_pixel(56, 38);

    //gliders
    for x in 0..=1 {
        framebuffer.set_pixel(x, 2);
    }
    for x in 1..=2 {
        framebuffer.set_pixel(x, 1);
    }
    framebuffer.set_pixel(0, 0);

    for x in 98..=99 {
        framebuffer.set_pixel(x, 2);
    }
    for x in 97..=98 {
        framebuffer.set_pixel(x, 1);
    }
    framebuffer.set_pixel(99, 0);

    for x in 0..=1 {
        framebuffer.set_pixel(x, 97);
    }
    for x in 1..=2 {
        framebuffer.set_pixel(x, 98);
    }
    framebuffer.set_pixel(0, 99);

    for x in 98..=99 {
        framebuffer.set_pixel(x, 97);
    }
    for x in 97..=98 {
        framebuffer.set_pixel(x, 98);
    }
    framebuffer.set_pixel(99, 99);
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
