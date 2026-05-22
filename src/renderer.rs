use std::{cell::RefCell, collections::VecDeque, error::Error};

use sdl3::{
    gpu::{
        BlitInfo, BufferUsageFlags, ColorTargetInfo, CommandBuffer, DepthStencilTargetInfo, LoadOp,
        RenderPass, ShaderFormat, Texture, TextureCreateInfo, TextureUsage,
    },
    pixels::Color,
};

use crate::error::InvalidOperationError;

// For now this is still locked into sdl3 constructs
// We may want to provide a more agnostic alternative at some point if we value switching rendering methods
pub trait Renderer<'a> {
    fn create_texture(
        &self,
        creation_info: TextureCreateInfo,
    ) -> Result<Texture<'static>, Box<dyn Error>>;

    fn get_render_target(&self) -> (&Texture<'a>, &Texture<'a>);

    fn create_buffer(
        &self,
        size: u32,
        usage: BufferUsageFlags,
    ) -> Result<sdl3::gpu::Buffer, Box<dyn Error>>;

    fn begin_frame(&self) -> Result<(), Box<dyn Error>>;

    fn begin_render_pass(
        &self,
        colour_targets: &[ColorTargetInfo],
        depth_target: Option<&DepthStencilTargetInfo>,
    ) -> Result<(), Box<dyn Error>>;

    fn end_render_pass(&self) -> Result<(), Box<dyn Error>>;

    fn end_frame(&self) -> Result<(), Box<dyn Error>>;

    fn blit_texture(&self, blit_info: BlitInfo) -> Result<(), Box<dyn Error>>;
}

pub mod sdl;
