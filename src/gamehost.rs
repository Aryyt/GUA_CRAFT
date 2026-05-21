use std::error::Error;

use sdl3::{
    gpu::{self, ColorTargetInfo, CommandBuffer, LoadOp, ShaderFormat},
    pixels::Color,
};

pub struct GameHost {
    sdl_context: sdl3::Sdl,
    video_subsystem: sdl3::VideoSubsystem,
    window: sdl3::video::Window,
    events: sdl3::EventPump,
    gpu: sdl3::gpu::Device,
}

impl GameHost {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("guacraft", 800, 600)
            .position_centered()
            .vulkan()
            .build()?;

        let events = sdl_context.event_pump()?;
        let gpu = sdl3::gpu::Device::new(ShaderFormat::SPIRV, false)?.with_window(&window)?;

        Ok(GameHost {
            sdl_context,
            video_subsystem,
            window,
            events,
            gpu,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            self.update();
            self.draw()?;
        }

        Ok(())
    }

    pub fn update(&self) {}

    pub fn draw(&self) -> Result<(), Box<dyn Error>> {
        let mut command_buffer = self.gpu.acquire_command_buffer()?;

        let swapchain = match command_buffer.acquire_swapchain_texture(&self.window)? {
            Some(s) => s,
            None => return Ok(()),
        };

        let colour_info = ColorTargetInfo::default()
            .with_texture(&swapchain)
            .with_clear_color(Color::RGB(255, 0, 100))
            .with_load_op(LoadOp::CLEAR);

        let render_pass = self
            .gpu
            .begin_render_pass(&command_buffer, &[colour_info], None)?;

        self.gpu.end_render_pass(render_pass);

        command_buffer.submit()?;

        Ok(())
    }
}
