// (c) anaya 2021
// for गाबीन <3
extern crate sdl2;
extern crate sdl2_unifont;

use sdl2::event::Event;

// video
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::rwops::RWops;

// input
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;

use sdl2_unifont::renderer::SurfaceRenderer;

// program resources class
struct ProgramResources {
    atlas: Surface<'static>
}

impl ProgramResources {
    pub fn new() -> ProgramResources {
        let atlas_bytes = include_bytes!("img/atlas.bmp"); // im in love wtff its so easy to embed files
        let mut rwops = RWops::from_bytes(atlas_bytes).unwrap();
        
        ProgramResources { atlas: Surface::load_bmp_rw(&mut rwops).unwrap() }
    }
}

// player struct
struct Player {
    x: i32,
    y: i32
}

impl Player {
    pub fn new() -> Player {
        Player { x: 0, y: 0 }
    }
}

// program core
struct Core {
    context: sdl2::Sdl,
    resources: ProgramResources
}

impl Core {
    pub fn new() -> Core {
        Core { context: sdl2::init().unwrap(), resources: ProgramResources::new() }
    }

    pub fn run(&self) {
        // load font first
        let mut text_renderer = SurfaceRenderer::new(Color::RGB(0, 0, 0), Color::RGB(255, 255, 255));
        text_renderer.scale = 2;

        let video_subsystem = self.context.video().unwrap();
        let window = video_subsystem.window("demo", 640, 480)
            .position_centered()
            .build()
            .unwrap();
        
        // init hardware renderer
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let text_surface = text_renderer.draw("this is what it looks like maiya").unwrap();
        let text_w = text_surface.width();
        let text_h = text_surface.height();

        // load stuff into the gpu
        let text = texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap();

        let atlas_minimap = texture_creator
            .create_texture_from_surface(&self.resources.atlas)
            .unwrap();

        // for events :p
        let mut event_pump = self.context.event_pump().unwrap();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running;
                    },
                    _ => {}
                }
            }
            let keyboard = KeyboardState::new(&event_pump); // input :3

            canvas.clear();
            canvas.copy(&atlas_minimap, None, None).unwrap();
            canvas.copy(&text, None, Rect::new(10, 10, text_w, text_h)).unwrap();
            canvas.present();
        }
    }
}

// easy startup :D
fn main() {
    Core::new().run();
}
