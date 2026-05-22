use std::{error::Error, time::Instant};

use sdl3::{event::WindowEvent, keyboard::Keycode};

use crate::{
    node::Node,
    renderer::{Renderer, sdl::SdlRenderer},
};

pub struct GameHost<'a> {
    sdl_context: sdl3::Sdl,
    video_subsystem: sdl3::VideoSubsystem,
    events: sdl3::EventPump,
    renderer: SdlRenderer<'a>,

    root_node: Option<Box<dyn Node>>,
}

struct ExitRequest;

impl<'a> GameHost<'a> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("guacraft", 800, 600)
            .position_centered()
            //.resizable()
            .vulkan()
            .build()?;

        let events = sdl_context.event_pump()?;
        let renderer = SdlRenderer::new(window)?;

        Ok(GameHost {
            sdl_context,
            video_subsystem,
            events,
            renderer,
            root_node: None,
        })
    }

    pub fn set_root_node<T>(&mut self, node: T)
    where
        T: Node,
    {
        self.root_node = Some(Box::new(node));
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut previous_time = Instant::now();
        loop {
            let time = Instant::now();
            let dt = time - previous_time;

            if let Some(ExitRequest) = &self.handle_event()? {
                break;
            }

            self.update(dt);
            self.draw()?;

            previous_time = time;
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

    fn update(&mut self, dt: std::time::Duration) {
        match &self.root_node {
            None => {}
            Some(n) => n.update(dt),
        }
    }

    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        self.renderer.begin_frame()?;

        match &self.root_node {
            None => {}
            Some(n) => {
                n.draw(&self.renderer)?;
            }
        };

        self.renderer.end_frame()?;

        Ok(())
    }
}
