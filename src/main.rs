mod slide;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::sys::KeyCode;
use slide::{SL_HEIGHT, SL_WIDTH};
use std::time::Duration;

use crate::slide::STANDARD_SPEED;

//mac values
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

//linux values
// const SCREEN_WIDTH: u32 = 1400;
// const SCREEN_HEIGHT: u32 = 1200;
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

    //ball thingy

    // let ball_speed_x = 7;
    // let ball_speed_y = 0;

    'running: loop {
        canvas.set_draw_color(cb);
        canvas.clear();


        color_rect(p1.rect, &mut canvas, cw);
        color_rect(p2.rect, &mut canvas, cw);
        color_rect(ball.rect, &mut canvas, cw);

        // handle_ball_mov(&mut ball);
        ball.handle_bounds_col();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }

                _ => {


                    handle_slide_mov_y_1(&mut p1, &event, 1);
                    handle_slide_mov_y_2(&mut p2, &event, 2);

                }
            }
        }

        // Update the canvas
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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

fn handle_slide_mov_y_1(slide: &mut slide::Slide, event: &Event, player_num: u8) {
    println!("p1 {}", slide.speed);
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

fn handle_slide_mov_y_2(slide: &mut slide::Slide, event: &Event, player_num: u8) {
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

fn handle_ball_mov(ball: &mut slide::Slide){

    let new_pos_x = ball.sx + ball.sy as i32 + STANDARD_SPEED as i32 * ball.dir_y as i32;

    // if new_pos_x < 0 {
    //     ball.dir = 1;
    // } else if new_pos_x + SL_WIDTH as i32 > SCREEN_WIDTH as i32 {
    //     ball.dir = -1;
    // } else {
    //     ball.sx = new_pos_x;
    // }

    if new_pos_x < 0 {
        ball.dir_y = 1;
    } else if new_pos_x + SL_HEIGHT as i32 > SCREEN_HEIGHT as i32 {
        ball.dir_y = -1;
    } else {
        ball.sx = new_pos_x;
    }

    ball.rect = sdl2::rect::Rect::new(ball.sx, ball.sy, ball.swidth, ball.sheight);
}

fn main() {
    game_init();
}
