// (c) anaya 2021
// for गाबीन <3

use raylib::prelude::*;
use serde::*;
use serde_json::*;

// for ecs
trait Entity {
    fn update(&mut self, rl: &RaylibHandle, core: *mut Core);
    fn draw(&self, draw_call: &mut RaylibMode2D<RaylibDrawHandle>);
}

// animation/sprite struct, kinda like the one from dvi/halcyon but in json
// NOTE: these structs hold only data. something else has to provide a 
// texture to be sourced from.

// define new rectangle so it can be serializable
// its identical to raylib's data wise.
#[derive(Deserialize)]
struct GabinRectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl GabinRectangle {
    pub fn to_rl_rect(&self) -> Rectangle {
        Rectangle { x: self.x, y: self.y, width: self.width, height: self.height }
    }
}

#[derive(Deserialize)]
struct Animation<'a> {
    name: &'a str,
    speed: i32,
    array: Vec<GabinRectangle>
}

impl Animation<'_> {
    pub fn new(name: &str, speed: i32, array: Vec<GabinRectangle>) -> Animation<'_> {
        Animation { name: name, speed: speed, array: array }
    }
}

#[derive(Deserialize)]
struct Sprite<'a> {
    name: &'a str,
    index: i32,
    animations: Vec<Animation<'a>>
}

impl Sprite<'_> {
    pub fn from_json_str(data: &'_ str) -> Sprite<'_> {
        let value: Sprite = serde_json::from_str(data).unwrap();
        return value;
    }

    pub fn width() -> i32 {
        
    }

    pub fn height() -> i32 {
        
    }
}

// player struct :3
struct Player {
    x: i32,
    y: i32
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player { x: x, y: y }
    }
}

impl Entity for Player {
    fn update(&mut self, rl: &RaylibHandle, core: *mut Core) {
        // get input, move, etc.
        use raylib::consts::KeyboardKey::*;
        let movement_vel = 3;

        if rl.is_key_down(KEY_UP) {
            self.y -= movement_vel;
        }

        if rl.is_key_down(KEY_DOWN) {
            self.y += movement_vel;
        }

        if rl.is_key_down(KEY_LEFT) {
            self.x -= movement_vel;
        }

        if rl.is_key_down(KEY_RIGHT) {
            self.x += movement_vel;
        }

        unsafe {
            let core_deref = &mut *core;
            core_deref.camera.target = Vector2::new(self.x as f32, self.y as f32);
        }
    }

    fn draw(&self, draw_call: &mut RaylibMode2D<RaylibDrawHandle>) {
        // println!("{}, {}", self.x, self.y);
        draw_call.draw_text("The Player", self.x, self.y, 24, Color::BLACK);
    }
}

// core struct holds stuff we need to keep (camera, etc.)
// kinda acts like map struct
struct Core {
    camera: Camera2D
}

impl Core {
    pub fn new(rl: &mut RaylibHandle) -> Core {
        Core { 
            camera: Camera2D { 
                offset: Vector2 { x: (rl.get_screen_height() / 2) as f32, y: (rl.get_screen_height() / 2) as f32 },
                target: Vector2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                zoom: 1.0
            }
        }
    }
}

fn main() {
    raylib::set_trace_log(TraceLogType::LOG_WARNING);
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("gabin's adventure :3")
        .build();
    rl.set_target_fps(60);

    // define stuff we need to keep ahold of in core struct
    let mut core = Core::new(&mut rl);
    let player = Box::new(Player::new(0, 0)); // except boxed stuff cuz pointer rules smh

    // rudimentary ecs
    let mut entities: Vec::<Box<dyn Entity>> = Vec::new();
    entities.push(player);

    while !rl.window_should_close() {
        // update entities
        for entity in &mut entities {
            entity.as_mut().update(&rl, &mut core);
        }

        let mut draw_call = rl.begin_drawing(&thread);
        draw_call.clear_background(Color::WHITE);

        // draw world
        // camera draw call scope
        {
            let mut camera_draw_call = draw_call.begin_mode2D(core.camera);

            camera_draw_call.draw_text("asdf", 0, 0, 64, Color::BLUE);

            for entity in &mut entities {
                entity.as_mut().draw(&mut camera_draw_call);
            }
        }
        
        draw_call.draw_fps(0, 0);
    }
}