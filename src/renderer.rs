use std::{collections::VecDeque, error::Error};

use sdl3::gpu::{
    BlitInfo, BufferUsageFlags, ColorTargetInfo, CommandBuffer, DepthStencilTargetInfo, LoadOp,
    RenderPass, ShaderFormat, Texture, TextureCreateInfo, TextureUsage,
};

pub struct Renderer<'a> {
    gpu: sdl3::gpu::Device,
    window: sdl3::video::Window,

    backbuffers: VecDeque<Texture<'a>>,
    depthbuffers: VecDeque<Texture<'a>>,
    current_backbuffer_index: u8,
}

impl<'a> Renderer<'a> {
    pub fn new(window: sdl3::video::Window) -> Result<Self, Box<dyn Error>> {
        let gpu = sdl3::gpu::Device::new(ShaderFormat::SPIRV, false)?.with_window(&window)?;

        let mut renderer = Renderer {
            gpu,
            window,
            backbuffers: [].into(),
            depthbuffers: [].into(),
            current_backbuffer_index: 0,
        };

        renderer.generate_offscreen_buffers()?;

        Ok(renderer)
    }

    pub fn generate_offscreen_buffers(&mut self) -> Result<(), Box<dyn Error>> {
        let size = self.window.size_in_pixels();

        self.current_backbuffer_index = 0;

        self.backbuffers.clear();
        self.depthbuffers.clear();

        for _ in 0..3 {
            let backbuffer_creation_info = TextureCreateInfo::new()
                .with_width(size.0)
                .with_height(size.1)
                .with_usage(TextureUsage::COLOR_TARGET)
                .with_type(sdl3::gpu::TextureType::_2D)
                .with_format(sdl3::gpu::TextureFormat::R8g8b8a8UnormSrgb)
                .with_num_levels(1)
                .with_layer_count_or_depth(1);

            let texture = self.gpu.create_texture(backbuffer_creation_info)?;

            self.backbuffers.push_back(texture);
        }

        for _ in 0..3 {
            let depthbuffer_creation_info = TextureCreateInfo::new()
                .with_width(size.0)
                .with_height(size.1)
                .with_usage(TextureUsage::DEPTH_STENCIL_TARGET)
                .with_type(sdl3::gpu::TextureType::_2D)
                .with_format(sdl3::gpu::TextureFormat::D32FloatS8Uint)
                .with_num_levels(4)
                .with_layer_count_or_depth(1);

            let texture = self.gpu.create_texture(depthbuffer_creation_info)?;

            self.depthbuffers.push_back(texture);
        }

        Ok(())
    }

    pub fn begin_render_pass(
        &self,
        command_buffer: &CommandBuffer,
        colour_targets: &[ColorTargetInfo],
        depth_target: Option<&DepthStencilTargetInfo>,
    ) -> Result<sdl3::gpu::RenderPass, sdl3::Error> {
        self.gpu
            .begin_render_pass(command_buffer, colour_targets, depth_target)
    }

    pub fn end_render_pass(&self, render_pass: RenderPass) {
        self.gpu.end_render_pass(render_pass);
    }

    pub fn get_backbuffer(&self) -> (&Texture<'_>, &Texture<'_>) {
        (
            &self.backbuffers[self.current_backbuffer_index.into()],
            &self.depthbuffers[self.current_backbuffer_index.into()],
        )
    }

    pub fn create_command_buffer(&self) -> Result<CommandBuffer, sdl3::Error> {
        self.gpu.acquire_command_buffer()
    }

    pub fn create_texture(
        &self,
        texture_creation_info: TextureCreateInfo,
    ) -> Result<Texture<'_>, sdl3::Error> {
        self.gpu.create_texture(texture_creation_info)
    }

    pub fn create_buffer(
        &self,
        size: u32,
        usage: BufferUsageFlags,
    ) -> Result<sdl3::gpu::Buffer, sdl3::Error> {
        self.gpu
            .create_buffer()
            .with_size(size)
            .with_usage(usage)
            .build()
    }

    pub fn end_frame(&mut self) -> Result<(), sdl3::Error> {
        let mut cb = self.create_command_buffer()?;

        let swapchain = match cb.acquire_swapchain_texture(&self.window)? {
            None => return Ok(()),
            Some(s) => s,
        };

        let (backbuffer, _) = self.get_backbuffer();

        let blit_info = BlitInfo::default()
            .with_source_texture(backbuffer)
            .with_destination_texture(&swapchain)
            .with_load_op(LoadOp::CLEAR)
            .with_source_region(0, 0, 0, backbuffer.width(), backbuffer.height())
            .with_destination_region(0, 0, 0, swapchain.width(), swapchain.height())
            .with_cycle(false);

        cb.blit_texture(blit_info);

        cb.submit()?;

        self.current_backbuffer_index = (self.current_backbuffer_index + 1) % 3;

        Ok(())
    }
}
