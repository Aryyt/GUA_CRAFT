use std::time::Duration;

use crate::renderer::Renderer;

pub trait Node {
    fn handle_input(&self, input: sdl3::event::Event) {}
    fn update(&self, dt: Duration) {}
    fn draw(&self, renderer: &Renderer) {}
}
