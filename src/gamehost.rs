use std::error::Error;

use sdl3::{
    event::WindowEvent,
    gpu::{ColorTargetInfo, LoadOp},
    keyboard::Keycode,
    pixels::Color,
};

use crate::renderer::Renderer;

pub struct GameHost<'a> {
    sdl_context: sdl3::Sdl,
    video_subsystem: sdl3::VideoSubsystem,
    events: sdl3::EventPump,
    renderer: Renderer<'a>,

    // Testing only
    i: u8,
}

struct ExitRequest;

impl<'a> GameHost<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("guacraft", 800, 600)
            .position_centered()
            .resizable()
            .vulkan()
            .build()?;

        let events = sdl_context.event_pump()?;
        let renderer = Renderer::new(window)?;

        Ok(GameHost {
            sdl_context,
            video_subsystem,
            events,
            renderer,
            i: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if let Some(ExitRequest) = &self.handle_event()? {
                break;
            }
            self.update();
            self.draw()?;
        }

        Ok(())
    }

    fn handle_event(&mut self) -> Result<Option<ExitRequest>, Box<dyn Error>> {
        for event in self.events.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. }
                | sdl3::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(Some(ExitRequest)),

                sdl3::event::Event::Window {
                    win_event: WindowEvent::Resized(..),
                    ..
                } => self.renderer.generate_offscreen_buffers()?,
                _ => {}
            }
        }

        Ok(None)
    }

    fn update(&mut self) {
        self.i = (self.i + 1) % 255;
    }

    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        let (backbuffer, _) = self.renderer.get_backbuffer();
        let command_buffer = self.renderer.create_command_buffer()?;

        let colour_target_info = ColorTargetInfo::default()
            .with_texture(backbuffer)
            .with_load_op(LoadOp::CLEAR)
            .with_clear_color(Color::RGB(self.i, 100, 255 - self.i))
            .with_store_op(sdl3::sys::gpu::SDL_GPUStoreOp::STORE);

        let render_pass =
            self.renderer
                .begin_render_pass(&command_buffer, &[colour_target_info], None)?;

        self.renderer.end_render_pass(render_pass);
        command_buffer.submit()?;

        self.renderer.end_frame()?;

        Ok(())
    }
}
