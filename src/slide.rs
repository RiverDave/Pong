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

    pub fn handle_bounds_col(&mut self) {
        let delta_x = 7;
        let delta_y = 7;

        //the condition below works if delta_x has a value and delta_y has not
        //each boundarie of the screen shall be divided by 2, and if the ball hits one side, it will change direction
        //for example if the ball hits the bottom right side of the screen , it will change direction to the right
        //but the y axis will be inverted obviously as it should.

        let half_screen_x = SCREEN_HEIGHT as i32 / 2;
        let half_screen_y = SCREEN_WIDTH as i32 / 2;

        //upper bound collision
        if self.sy + delta_y + BALL_HEIGHT as i32 > SCREEN_HEIGHT as i32 {
            if self.sx > half_screen_y {
                println!("Not guilty");
                self.dir_y = -1;
                self.dir_x = 1;
            }else {
                println!("Definetly maybe");
                self.dir_y = -1;
                self.dir_x = -1;
            }
        } 

        //lower bound collision
        if self.sy + delta_y < 0 {
            if  self.sx > half_screen_y {

                self.dir_y = 1;
                self.dir_x = 1;
            } else {

                self.dir_y = 1;
                self.dir_x = -1;
            }
        }

        //TODO: Pad collision

        self.sx += delta_x * self.dir_x;
        self.sy += delta_y * self.dir_y;

        self.rect = rect::Rect::new(self.sx, self.sy, self.swidth, self.sheight);
    }

    // pub fn handle_bounds_col(&mut self, delta_x: i32, delta_y: i32) {
    //     // Update the ball's position based on its current speed and direction
    //     let mut new_pos_x = self.sx + delta_x * self.dir;
    //     let mut new_pos_y = self.sy + delta_y * self.dir;
    //
    //     // Check for collisions with horizontal boundaries
    //     if new_pos_x + self.swidth as i32 > SCREEN_WIDTH as i32 || new_pos_x < 0 {
    //         // If the collision is near the center of the screen
    //         let half_screen = SCREEN_WIDTH as i32 / 2;
    //         if (self.sx < half_screen && new_pos_x >= half_screen)
    //             || (self.sx > half_screen && new_pos_x <= half_screen)
    //         {
    //             // Reverse the horizontal direction
    //             self.dir *= -1;
    //         } else {
    //             // Perform regular horizontal collision logic
    //             self.dir *= 1; // Or any other necessary logic based on your game
    //         }
    //
    //         // Calculate the new x position to prevent going beyond boundaries
    //         new_pos_x = if new_pos_x + self.swidth as i32 > SCREEN_WIDTH as i32 {
    //             SCREEN_WIDTH as i32 - self.swidth as i32
    //         } else {
    //             0
    //         };
    //     }
    //
    //     // Check for collisions with vertical boundaries
    //     if new_pos_y + self.sheight as i32 > SCREEN_HEIGHT as i32 || new_pos_y < 0 {
    //         // Reverse the vertical direction upon collision
    //         self.dir *= -1;
    //
    //         // Calculate the new y position to prevent going beyond boundaries
    //         new_pos_y = if new_pos_y + self.sheight as i32 > SCREEN_HEIGHT as i32 {
    //             SCREEN_HEIGHT as i32 - self.sheight as i32
    //         } else {
    //             0
    //         };
    //     }
    //
    //     // Update the ball's position
    //     self.sx = new_pos_x;
    //     self.sy = new_pos_y;
    //
    //     // Update the ball's rectangle
    //     self.rect = rect::Rect::new(self.sx, self.sy, self.swidth, self.sheight);
    // }
}
