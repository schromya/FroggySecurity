use bracket_lib::prelude::*;

const SCREEN_HEIGHT_FLOAT: f32 = 25.0;
// const FROG_JUMPING: &str = "         (•)___(•)\n      /\\/         \\/\\\n      | \\         / |\n     _|  ---------  |_";
// const FROG_DUCKING: &str = "         (•)___(•)\n      /\\/         \\/\\\n      | \\         / |\n     _|  ---------  |_";

const FROG_DUCKING: [&str; 4] = [
    "     (•)___(•)",     
    "  /\\/         \\/\\",
    "  | \\         / |",
    " _|  ---------  |_",         
];

const FROG_JUMPING: [&str; 6] = [
    "      (•)___(•)",  
    "     /         \\",
    "    |\\         /\\",  
    "    |  -------  |",   
    "    |           |",
    "    /           \\",
];

pub struct Frog {
    pub x: i32,
    pub y: f32,
    pub velocity: f32,
}

impl Frog {
    pub fn new() -> Self {
        Frog {
            x: 5,
            y: SCREEN_HEIGHT_FLOAT - 7.0,
            velocity: 0.0,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        if let Some(VirtualKeyCode::Down) = ctx.key {
            for (i, line) in FROG_DUCKING.iter().enumerate() {
                ctx.print(5, self.y as i32 + i as i32, line);
            }
        } else {
            for (i, line) in FROG_JUMPING.iter().enumerate() {
                ctx.print(5, self.y as i32 + i as i32, line);
            }
        }
    }

    // pub fn render(&mut self, ctx: &mut BTerm) {
    //     if let Some(VirtualKeyCode::Down) = ctx.key {
    //         ctx.print(5, self.y as i32, FROG_DUCKING);
    //     } else {
    //         ctx.print(5, self.y as i32, FROG_JUMPING);
    //     }
    // }

    pub fn gravity_and_move(&mut self) {
        if self.velocity < 3.0 {
            self.velocity += 0.5;
        }
        self.y += self.velocity as f32;
        self.x += 1;
        if self.y < 0.0 {
            self.y = 0.0;
        }
        if self.y > SCREEN_HEIGHT_FLOAT - 7.0 {
            self.y = SCREEN_HEIGHT_FLOAT - 7.0;
        }
    }

    pub fn jump(&mut self) {
        if self.y == SCREEN_HEIGHT_FLOAT - 7.0 {
            self.velocity = -3.0;
        }
    }

    pub fn in_range(&mut self) -> bool {
        let hit_cactus = (self.y == SCREEN_HEIGHT_FLOAT - 8.0) || (SCREEN_HEIGHT_FLOAT - 7.0 == self.y);
        hit_cactus
    }
}
