// (c) anaya 2021
// for गाबीन <3

use raylib::prelude::*;

// for ecs
trait Entity {
    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&mut self, draw_call: &mut RaylibDrawHandle);
}

// player struct :3
struct Player {
    x: i32,
    y: i32
}

impl Player {
    pub fn new() -> Player {
        Player { x: 0, y: 0 }
    }
}

impl Entity for Player {
    fn update(&mut self, rl: &RaylibHandle) {
        // get input, move, etc.
        use raylib::consts::KeyboardKey::*;
        let delta = rl.get_frame_time();

        if rl.is_key_down(KEY_W) {
            self.y -= (256.0* delta) as i32;
        }

        if rl.is_key_down(KEY_S) {
            self.y += (256.0  * delta) as i32;
        }

        if rl.is_key_down(KEY_A) {
            self.x -= (256.0 * delta) as i32;
        }

        if rl.is_key_down(KEY_D) {
            self.x += (256.0 * delta) as i32;
        }
    }

    fn draw(&mut self, draw_call: &mut RaylibDrawHandle) {
        // println!("{}, {}", self.x, self.y);
        draw_call.draw_text("The Player", self.x, self.y, 24, Color::BLACK);
    }
}

fn main() {
    raylib::set_trace_log(TraceLogType::LOG_WARNING);
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("")
        .build();

    // define stuff we need to keep ahold of
    let player = Box::new(Player::new()); // box entities we keep cuz we need a reference

    // rudimentary ecs
    let mut entities: Vec::<Box<dyn Entity>> = Vec::new();
    entities.push(player);

    while !rl.window_should_close() {
        // update entities
        for entity in &mut entities {
            entity.as_mut().update(&rl);
        }

        // draw world
        let mut draw_call = rl.begin_drawing(&thread);
        draw_call.clear_background(Color::WHITE);

        for entity in &mut entities {
            entity.as_mut().draw(&mut draw_call);
        }

        draw_call.draw_fps(0, 0);
    }
}