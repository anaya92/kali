// (c) anaya 2021
// for गाबीन <3

use raylib::prelude::*;
use serde::*;

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

#[derive(Deserialize)]
struct Sprite<'a> {
    pub name: &'a str,
    animation_index: usize,
    frame_index: usize,
    ticks: i32,
    animations: Vec<Animation<'a>>
}

impl Sprite<'_> {
    pub fn from_json_str(data: &'_ str) -> Sprite<'_> {
        let value: Sprite = serde_json::from_str(data).unwrap();
        return value;
    }

    pub fn from_bytes(bytes: &[u8]) -> Sprite<'_> {
        let string = std::str::from_utf8(bytes).unwrap();
        return Sprite::from_json_str(string);
    }

    pub fn width(&self) -> i32 {
        return self.animations[self.animation_index].array[self.frame_index].width as i32;
    }

    pub fn height(&self) -> i32 {
        return self.animations[self.animation_index].array[self.frame_index].height as i32;
    }

    fn get_current_animation(&self) -> &Animation {
        return &self.animations[self.animation_index];
    }

    pub fn update(&mut self) // called by parent class or whatever
    {
        self.ticks += 1;

        if (self.ticks % self.get_current_animation().speed) == 0 { 
            self.frame_index += 1;
        }

        if self.frame_index > self.get_current_animation().array.len() - 1 {
            self.frame_index = 0;
        }
    }

    pub fn draw(&self, x: i32, y: i32, draw_call: &mut RaylibMode2D<RaylibDrawHandle>, atlas: &Texture2D) {
        draw_call.draw_texture_pro(
            atlas,
            self.get_current_animation().array[self.frame_index].to_rl_rect(),
            Rectangle { x: x as f32, y: y as f32, width: self.width() as f32, height: self.height() as f32 },
            Vector2 { x: (self.width() / 2) as f32, y: (self.height() / 2) as f32 },
            0.0,
            Color::WHITE
        )
    }
}

// entity trait for ecs
trait Entity {
    fn update(&mut self, rl: &RaylibHandle, core: *mut Core);
    fn draw(&self, core: *mut Core, draw_call: &mut RaylibMode2D<RaylibDrawHandle>);
}

// player struct :3
struct Player<'a> {
    x: i32,
    y: i32,
    sprite: Sprite<'a>
}

impl Player<'_> {
    pub fn new(x: i32, y: i32) -> Player<'static> {
        Player { x: x, y: y, sprite: Sprite::from_bytes(include_bytes!("data/gabin.json")) }
    }
}

impl Entity for Player<'_> {
    fn update(&mut self, rl: &RaylibHandle, core: *mut Core) {
        // get input, move, etc.
        use raylib::consts::KeyboardKey::*;
        let movement_vel = 2;

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

        self.sprite.update();

        unsafe {
            let core_deref = &mut *core;
            core_deref.camera.target = Vector2::new(self.x as f32, self.y as f32);
        }
    }

    fn draw(&self, core: *mut Core, draw_call: &mut RaylibMode2D<RaylibDrawHandle>) {
        draw_call.draw_text("The Player", self.x, self.y, 24, Color::BLACK);

        unsafe {
            let core_deref = &*core;
            self.sprite.draw(self.x, self.y, draw_call, &core_deref.atlas);
        }
    }
}

// core struct holds stuff we need to keep (camera, etc.)
// kinda acts like map struct
struct Core {
    camera: Camera2D,
    atlas: Texture2D
}

impl Core {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Core {
        let atlas_texture: Texture2D;

        unsafe {
            let atlas_filetype: [char; 5] = ['.', 'p', 'n', 'g', '\0'];
            let atlas_bytes = include_bytes!("data/atlas.png");
    
            let atlas_ffi_image = raylib::ffi::LoadImageFromMemory(
                atlas_filetype.as_ptr() as *const i8,
                atlas_bytes as *const u8, 
                atlas_bytes.len() as i32
            );

            let texture_temp = raylib::ffi::LoadTextureFromImage(atlas_ffi_image);

            atlas_texture = Texture2D::from_raw(texture_temp);
        }

        Core { 
            camera: Camera2D { 
                offset: Vector2 { x: (rl.get_screen_height() / 2) as f32, y: (rl.get_screen_height() / 2) as f32 },
                target: Vector2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                zoom: 1.0
            },
            atlas: atlas_texture
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
    let mut core = Core::new(&mut rl, &thread);
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
                entity.as_mut().draw(&mut core, &mut camera_draw_call);
            }
        }
        
        draw_call.draw_fps(0, 0);
    }
}