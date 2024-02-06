mod slide;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use slide::{SL_HEIGHT, SL_WIDTH};
use std::time::Duration;

use crate::slide::STANDARD_SPEED;

//mac values, in a 13' inch screen
// const SCREEN_WIDTH: u32 = 800;
// const SCREEN_HEIGHT: u32 = 600;

//linux values, in a 27' inch screen
const SCREEN_WIDTH: u32 = 900;
const SCREEN_HEIGHT: u32 = 1400;
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
    let cb: Color = Color::RGB(0, 0, 0); //BLACK
    let cw: Color = Color::RGB(255, 255, 255); // WHITE

    //entity initialization
    let mut p1 = slide::Slide::new(0, (SCREEN_HEIGHT as i32 - SL_HEIGHT as i32) / 2, false);
    let mut p2 = slide::Slide::new(
        SCREEN_WIDTH as i32 - SL_WIDTH as i32,
        (SCREEN_HEIGHT as i32 - SL_HEIGHT as i32) / 2,
        false,
    );
    let mut ball = slide::Slide::new(
        (SCREEN_WIDTH as i32 - SL_WIDTH as i32) / 2,
        (SCREEN_HEIGHT as i32 - SL_HEIGHT as i32) / 2,
        true,
    );

    canvas.set_draw_color(cb);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(cb);
        canvas.clear();

        //Paint entities
        draw_line(&mut canvas, cw);
        color_rect(p1.rect, &mut canvas, cw);
        color_rect(p2.rect, &mut canvas, cw);
        color_rect(ball.rect, &mut canvas, cw);

        // THIS HANDLES BALL PHYSICS
        ball.handle_bounds_col(&p1, &p2);
        // THIS HANDLES BALL REPOSITION UPON SCORING
        ball.ball_score();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } //escape sequences below
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    .. //default params
                } | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    break 'running;
                }

                _ => {

                    // movement logic
                    handle_slide_mov_y_1(&mut p1, &event);
                    handle_slide_mov_y_2(&mut p2, &event);

                }
            }
        }

        // Update the canvas
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_line(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: sdl2::pixels::Color) {
    canvas.set_draw_color(color);

    for i in (1..SCREEN_HEIGHT).step_by(3) {
        canvas
            .draw_point(((SCREEN_WIDTH / 2) as i32, i as i32))
            .unwrap();
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

fn handle_slide_mov_y_1(slide: &mut slide::Slide, event: &Event) {
    // println!("p1 {}", slide.speed);
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => {
            slide.dir_y = -1; // Move up when 'W' is pressed
        }
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => {
            slide.dir_y = 1; // Move down when 'S' is pressed
        }
        Event::KeyUp {
            keycode: Some(Keycode::W) | Some(Keycode::S),
            ..
        } => {
            slide.dir_y = 0; // Stop movement when 'W' or 'S' is released
        }
        _ => {}
    }

    // Calculate new position based on direction and boundaries
    let new_pos = slide.sy as i32 + slide.speed as i32 * slide.dir_y as i32;

    if new_pos < 0 {
        slide.sy = 0;
    } else if new_pos + SL_HEIGHT as i32 > SCREEN_HEIGHT as i32 {
        slide.sy = SCREEN_HEIGHT as i32 - SL_HEIGHT as i32;
    } else {
        slide.sy = new_pos;
    }

    slide.rect = sdl2::rect::Rect::new(slide.sx, slide.sy, slide.swidth, slide.sheight);
}

fn handle_slide_mov_y_2(slide: &mut slide::Slide, event: &Event) {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::K),
            ..
        } => {
            slide.dir_y = -1; // Move up when 'W' is pressed
        }
        Event::KeyDown {
            keycode: Some(Keycode::J),
            ..
        } => {
            slide.dir_y = 1; // Move down when 'S' is pressed
        }
        Event::KeyUp {
            keycode: Some(Keycode::J) | Some(Keycode::K),
            ..
        } => {
            slide.dir_y = 0; // Stop movement when 'W' or 'S' is released
        }
        _ => {}
    }

    // Calculate new position based on direction and boundaries
    let new_pos = slide.sy as i32 + STANDARD_SPEED as i32 * slide.dir_y as i32;

    if new_pos < 0 {
        slide.sy = 0;
    } else if new_pos + SL_HEIGHT as i32 > SCREEN_HEIGHT as i32 {
        slide.sy = SCREEN_HEIGHT as i32 - SL_HEIGHT as i32;
    } else {
        slide.sy = new_pos;
    }

    slide.rect = sdl2::rect::Rect::new(slide.sx, slide.sy, slide.swidth, slide.sheight);
}

fn main() {
    game_init();
}
