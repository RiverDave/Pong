use sdl2::rect;

const STANDARD_DIR: i8 = 0;
const STANDARD_SPEED: u32 = 10;
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
    pub dir: i8,
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
                dir: 0, //REMEMBER TO CHANGE TO STANDARD_DIR AFTER TESTING
                speed: STANDARD_SPEED,
                rect: rect::Rect::new(x, y, BALL_HEIGHT, BALL_WIDTH),
            },
            false =>

        Slide {
            sx: x,
            sy: y,
            sheight: SL_HEIGHT,
            swidth: SL_WIDTH,
            dir: 0, //REMEMBER TO CHANGE TO STANDARD_DIR AFTER TESTING
            speed: STANDARD_SPEED,
            rect: rect::Rect::new(x, y, SL_WIDTH, SL_HEIGHT),
        }
    }
    }
    //function may be inncecessary, since the canvas could also perform this in the init func
}
