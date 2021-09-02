// (c) anaya 2021
// for गाबीन <3
extern crate sdl2;

use std::convert::TryInto;

// general
use sdl2::event::Event;
use sdl2::ttf::Sdl2TtfContext;

// video
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use sdl2::ttf::Font;

// input
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;

// program resources class
struct ProgramResources {
    atlas: Surface<'static>,
    font: Font<'static, 'static>,
}

impl ProgramResources {
    pub fn new(ttf_context: sdl2::ttf::Sdl2TtfContext) -> ProgramResources {
        let atlas_bytes = include_bytes!("bin/atlas.bmp"); // im in love wtff its so easy to embed files
        let mut atlas_rwops = RWops::from_bytes(atlas_bytes).unwrap();

        let font_bytes = include_bytes!("bin/internal.ttf");
        let mut font_rwops = RWops::from_bytes(font_bytes).unwrap();

        ProgramResources {
            atlas: Surface::load_bmp_rw(&mut atlas_rwops).unwrap(),
            font: ttf_context.load_font_from_rwops(font_rwops, 24).unwrap(),
        }
    }
}

// player class
struct Player {
    x: i32,
    y: i32,
}

impl Player {
    pub fn new() -> Player {
        Player { x: 0, y: 0 }
    }
}

// program core
struct Core {
    sdl_context: sdl2::Sdl,
    ttf_context: Sdl2TtfContext,
    resources: ProgramResources,
}

impl Core {
    pub fn new() -> Core {
        let _ttf_context = sdl2::ttf::init().unwrap();

        let core = Core { 
            sdl_context: sdl2::init().unwrap(),
            ttf_context: _ttf_context,
            resources: ProgramResources::new(_ttf_context);
        };

        return core;
    }

    pub fn run(&self) {
        let video_subsystem = self.sdl_context.video().unwrap();
        let window = video_subsystem
            .window("demo", 640, 480)
            .position_centered()
            .build()
            .unwrap();

        // init hardware renderer
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        // load player + map
        let mut player = Player::new();
        player.x = 5;
        player.y = 5;

        let atlas_minimap = texture_creator
            .create_texture_from_surface(&self.resources.atlas)
            .unwrap();

        // for events :p
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
            let keyboard = KeyboardState::new(&event_pump); // input :3

            canvas.clear();
            canvas.copy(&atlas_minimap, None, None).unwrap();
            canvas.present();
        }
    }
}

// easy startup :D
fn main() {
    Core::new().run();
}
