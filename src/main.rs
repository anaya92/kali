// (c) anaya 2021
// for गाबीन <3
extern crate sdl2;

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
struct ProgramResources<'a> {
    atlas: Surface<'a>,
    font: Font<'a, 'a>,
}

impl ProgramResources<'static> {
    pub fn new(ttf_context: &'_ sdl2::ttf::Sdl2TtfContext) -> ProgramResources {
        let atlas_bytes = include_bytes!("bin/atlas.bmp"); // im in love wtff its so easy to embed files
        let mut atlas_rwops = RWops::from_bytes(atlas_bytes).unwrap();

        let font_bytes = include_bytes!("bin/internal.ttf");
        let font_rwops = RWops::from_bytes(font_bytes).unwrap();

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
}

impl Core {
    pub fn new() -> Core {
        Core { sdl_context: sdl2::init().unwrap(), ttf_context: sdl2::ttf::init().unwrap() }
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

        // resources
        let resources = ProgramResources::new(&self.ttf_context);

        // load player + map if needed
        let mut player = Player::new();
        player.x = 5;
        player.y = 5;

        // prerender text
        let text_surface = resources.font.render("<3")
            .solid(Color::BLACK).unwrap();

        // load stuff into gpu
        let atlas_minimap = texture_creator
            .create_texture_from_surface(&resources.atlas)
            .unwrap();

        let text_texture = texture_creator
            .create_texture_from_surface(&text_surface)
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
            if keyboard.is_scancode_pressed(Scancode::Escape) { break 'running; }

            canvas.clear();
            canvas.copy(&atlas_minimap, None, None).unwrap();
            canvas.copy(&text_texture, None, Rect::new(10, 10, text_surface.width(), text_surface.height())).unwrap();
            canvas.present();
        }
    }
}

// easy startup :D
fn main() {
    Core::new().run();
}
