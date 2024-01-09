use rand::Rng;
use sdl2::rect;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

// const STANDARD_DIR: i8 = 0;
pub const STANDARD_SPEED: u32 = 30;
pub const SL_HEIGHT: u32 = 180;
pub const SL_WIDTH: u32 = 20;

pub const BALL_HEIGHT: u32 = 20;
pub const BALL_WIDTH: u32 = 20;

//base player structure
pub struct Slide {
    pub sx: i32,
    pub sy: i32,
    pub sheight: u32,
    pub swidth: u32,
    pub dir_y: i32,
    pub dir_x: i32,
    pub speed: u32,
    pub rect: sdl2::rect::Rect,
}

impl Slide {
    pub fn new(x: i32, y: i32, isball: bool) -> Slide {
        //if true returns presets for ball, otherwise returns presets for slide
        match isball {
            true => Slide {
                sx: x,
                sy: y,
                sheight: BALL_HEIGHT,
                swidth: BALL_WIDTH,
                dir_y: 1, //REMEMBER TO CHANGE TO STANDARD_DIR AFTER TESTING
                dir_x: 1,
                speed: STANDARD_SPEED,
                rect: rect::Rect::new(x, y, BALL_HEIGHT, BALL_WIDTH),
            },
            false => Slide {
                sx: x,
                sy: y,
                sheight: SL_HEIGHT,
                swidth: SL_WIDTH,
                dir_y: 0, //REMEMBER TO CHANGE TO STANDARD_DIR AFTER TESTING
                dir_x: 0,
                speed: STANDARD_SPEED,
                rect: rect::Rect::new(x, y, SL_WIDTH, SL_HEIGHT),
            },
        }
    }
    //function may be inncecessary, since the canvas could also perform this in the init func

    //TODO: rework the direction change
    pub fn handle_bounds_col(&mut self, p1: &Slide, p2: &Slide) {
        let delta_x = 7;
        let delta_y = 7;

        //the condition below works if delta_x has a value and delta_y has not
        //each boundarie of the screen shall be divided by 2, and if the ball hits one side, it will change direction
        //for example if the ball hits the bottom right side of the screen , it will change direction to the right
        //but the y axis will be inverted obviously as it should.

        // let half_screen_x = SCREEN_HEIGHT as i32 / 2;
        let half_screen_y = SCREEN_WIDTH as i32 / 2;

        //upper bound collision
        if self.sy + delta_y + BALL_HEIGHT as i32 > SCREEN_HEIGHT as i32 {
            if self.sx > half_screen_y {
                self.dir_y = -1;
                self.dir_x = 1;
            } else {
                self.dir_y = -1;
                self.dir_x = -1;
            }
        }

        //lower bound collision
        if self.sy + delta_y < 0 {
            if self.sx > half_screen_y {
                self.dir_y = 1;
                self.dir_x = 1;
            } else {
                self.dir_y = 1;
                self.dir_x = -1;
            }
        }


        if self.check_collision(&p1) {
            if self.sy < (self.sheight / 2) as i32 {
                println!("UP");
                self.dir_x = -1;
                self.dir_y = 1;
            } else {
                println!("Down");
                self.dir_x = 1;
                self.dir_y = -1;
            }
        }

        if self.check_collision(&p2) {
            if self.sy < (self.sheight / 2) as i32 {

                self.dir_x = -1;
                self.dir_y = -1;
            } else {

                self.dir_x = -1;
                self.dir_y = 1;
            }
        }

        self.sx += delta_x * self.dir_x;
        self.sy += delta_y * self.dir_y;

        self.rect = rect::Rect::new(self.sx, self.sy, self.swidth, self.sheight);
    }

    fn check_collision(&mut self, pad: &Slide) -> bool {
        // Calculate the sides of the ball
        let ball_left = self.sx;
        let ball_right = self.sx + self.swidth as i32;
        let ball_top = self.sy;
        let ball_bottom = self.sy + self.sheight as i32;

        // Calculate the sides of the pad
        let pad_left = pad.sx;
        let pad_right = pad.sx + pad.swidth as i32;
        let pad_top = pad.sy;
        let pad_bottom = pad.sy + pad.sheight as i32;

        // Check for collision
        if ball_right >= pad_left
            && ball_left <= pad_right
            && ball_bottom >= pad_top
            && ball_top <= pad_bottom
        {
            // Collision detected
            return true;
        }

        // No collision
        false
    }

    // brings ball back to the center
    pub fn ball_debug(&mut self) {
        let new_dir_x: i32;
        let new_dir_y: i32;

        let mut rng = rand::thread_rng();
        let rand_bool_x: bool = rng.gen();
        let rand_bool_y: bool = rng.gen();

        if self.sx > SCREEN_WIDTH as i32 || self.sx < 0 {
            self.sx = (SCREEN_WIDTH as i32 - SL_WIDTH as i32) / 2;
            self.sy = (SCREEN_HEIGHT as i32 - SL_HEIGHT as i32) / 2;

            //notice how expressions are sorrounded by curly brackets:  {}
            new_dir_x = if rand_bool_x { -1 } else { 1 };
            new_dir_y = if rand_bool_y { -1 } else { 1 };

            self.dir_x = new_dir_x;
            self.dir_y = new_dir_y;
        }

        self.rect = rect::Rect::new(self.sx, self.sy, self.swidth, self.sheight);
    }
}
