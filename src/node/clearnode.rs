use std::{
    cell::{Cell, RefCell},
    error::Error,
    rc::Rc,
};

use sdl3::{
    gpu::{BlitInfo, ColorTargetInfo, LoadOp, Texture, TextureCreateInfo, TextureUsage},
    pixels::Color,
};

use crate::{node::Node, renderer::Renderer};

pub struct ClearNode {
    position: (u32, u32),
    size: (u32, u32),
    clear_colour: Color,

    // We know that the texture backing this will be tied to the node itself
    backing_texture: RefCell<Option<Texture<'static>>>,
}
impl ClearNode {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            size: (0, 0),
            clear_colour: Color::RGB(0, 0, 0),
            backing_texture: RefCell::new(None),
        }
    }

    pub fn with_position(&self, position: (u32, u32)) -> Self {
        Self {
            position,
            backing_texture: RefCell::new(None),
            ..*self
        }
    }

    pub fn with_size(&self, size: (u32, u32)) -> Self {
        Self {
            size,
            backing_texture: RefCell::new(None),
            ..*self
        }
    }

    pub fn with_clear_colour(&self, clear_colour: Color) -> Self {
        Self {
            clear_colour,
            backing_texture: RefCell::new(None),
            ..*self
        }
    }
}

impl Default for ClearNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for ClearNode {
    fn draw(&self, renderer: &Renderer) -> Result<(), Box<dyn Error>> {
        // First check if we have a backing texture
        let mut backing_texture = self.backing_texture.borrow();

        if backing_texture.is_none() {
            drop(backing_texture);

            // Try creating a texture
            let texture = renderer.create_texture(
                TextureCreateInfo::new()
                    .with_width(1)
                    .with_height(1)
                    .with_layer_count_or_depth(1)
                    .with_num_levels(1)
                    .with_sample_count(sdl3::gpu::SampleCount::NoMultiSampling)
                    .with_format(sdl3::gpu::TextureFormat::R8g8b8a8UnormSrgb)
                    .with_usage(TextureUsage::COLOR_TARGET)
                    .with_type(sdl3::gpu::TextureType::_2D),
            )?;

            self.backing_texture.replace(Some(texture));

            backing_texture = self.backing_texture.borrow();
        }

        let backing_texture = backing_texture
            .as_ref()
            .expect("We definitely have a texture at this point.");

        let cb = renderer.create_command_buffer()?;

        let colour_target = ColorTargetInfo::default()
            .with_texture(backing_texture)
            .with_load_op(LoadOp::CLEAR)
            .with_clear_color(self.clear_colour);

        let rp = renderer.begin_render_pass(&cb, &[colour_target], None)?;

        renderer.end_render_pass(rp);

        let swapchain = renderer.get_backbuffer();

        cb.blit_texture(
            BlitInfo::default()
                .with_source_texture(backing_texture)
                .with_source_region(0, 0, 0, 1, 1)
                .with_destination_texture(swapchain.0)
                .with_destination_region(
                    0,
                    self.position.0,
                    self.position.1,
                    self.size.0,
                    self.size.1,
                )
                .with_load_op(LoadOp::DONT_CARE),
        );

        cb.submit()?;

        Ok(())
    }
}
