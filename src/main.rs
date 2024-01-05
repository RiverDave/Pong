mod slide;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::sys::KeyCode;
use slide::{SL_HEIGHT, SL_WIDTH};
use std::time::Duration;

const SCREEN_WIDTH: u32 = 1400;
const SCREEN_HEIGHT: u32 = 1200;

fn game_init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Pong", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    //colors used:
    let cb: Color = Color::RGB(0, 0, 0);
    let cw: Color = Color::RGB(255, 255, 255);

    //entity initialization
    let mut p1 = slide::Slide::new(0, (SCREEN_HEIGHT as i32  - SL_HEIGHT as i32) / 2, false);
    let mut p2 = slide::Slide::new(
        SCREEN_WIDTH as i32 - SL_WIDTH as i32,
        (SCREEN_HEIGHT as i32 - SL_HEIGHT as i32) / 2,
        false
    );
    let mut ball = slide::Slide::new(
        (SCREEN_WIDTH as i32 - SL_WIDTH as i32) / 2,
        (SCREEN_HEIGHT as i32 - SL_HEIGHT as i32) / 2,
        true
    );

    canvas.set_draw_color(cb);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        color_rect(p1.rect, &mut canvas, cw);
        color_rect(p2.rect, &mut canvas, cw);
        color_rect(ball.rect, &mut canvas, cw);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        canvas.present();
    }
}

fn color_rect(
    prect: sdl2::rect::Rect,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    color: sdl2::pixels::Color,
) {
    canvas.set_draw_color(color);
    canvas.fill_rect(prect).unwrap();
}

//will handle slide movement based on keypresses?
fn handle_slide_mov(slide: slide::Slide, event: Event) {
    todo!();
}

fn main() {
    game_init();
}
