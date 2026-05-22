use std::{cell::RefCell, collections::VecDeque, error::Error};

use sdl3::{
    gpu::{
        BlitInfo, BufferUsageFlags, ColorTargetInfo, CommandBuffer, DepthStencilTargetInfo, LoadOp,
        RenderPass, ShaderFormat, Texture, TextureCreateInfo, TextureUsage,
    },
    pixels::Color,
};

use crate::{error::InvalidOperationError, renderer::Renderer};

pub struct SdlRenderer<'a> {
    gpu: sdl3::gpu::Device,
    window: sdl3::video::Window,

    backbuffers: VecDeque<Texture<'a>>,
    depthbuffers: VecDeque<Texture<'a>>,
    current_backbuffer_index: usize,

    current_command_buffer: RefCell<Option<CommandBuffer>>,
    current_render_pass: RefCell<Option<RenderPass>>,
}

impl<'a> Renderer<'a> for SdlRenderer<'a> {
    fn create_texture(
        &self,
        creation_info: TextureCreateInfo,
    ) -> Result<Texture<'static>, Box<dyn Error>> {
        Ok(self.gpu.create_texture(creation_info)?)
    }

    fn get_render_target(&self) -> (&Texture<'a>, &Texture<'a>) {
        (
            &self.backbuffers[self.current_backbuffer_index],
            &self.depthbuffers[self.current_backbuffer_index],
        )
    }

    fn create_buffer(
        &self,
        size: u32,
        usage: BufferUsageFlags,
    ) -> Result<sdl3::gpu::Buffer, Box<dyn Error>> {
        Ok(self
            .gpu
            .create_buffer()
            .with_size(size)
            .with_usage(usage)
            .build()?)
    }

    fn begin_render_pass(
        &self,
        colour_targets: &[ColorTargetInfo],
        depth_target: Option<&DepthStencilTargetInfo>,
    ) -> Result<(), Box<dyn Error>> {
        match self.current_command_buffer.borrow().as_ref() {
            Some(cb) => {
                let old_render_pass = self.current_render_pass.borrow();

                if old_render_pass.is_some() {
                    drop(old_render_pass);
                    self.end_render_pass()?;
                } else {
                    drop(old_render_pass);
                }

                self.current_render_pass
                    .replace(Some(self.gpu.begin_render_pass(
                        cb,
                        colour_targets,
                        depth_target,
                    )?));
            }

            None => {
                panic!("Cannot begin a render pass before frame begins.");
            }
        };

        Ok(())
    }

    fn end_render_pass(&self) -> Result<(), Box<dyn Error>> {
        match self.current_render_pass.take() {
            Some(rp) => {
                self.gpu.end_render_pass(rp);

                Ok(())
            }
            None => Err(Box::new(InvalidOperationError::new(
                "Cannot end a non-existant render pass.",
            ))),
        }
    }

    fn begin_frame(&self) -> Result<(), Box<dyn Error>> {
        let command_buffer = self.gpu.acquire_command_buffer()?;
        let old_buffer = self.current_command_buffer.replace(Some(command_buffer));

        debug_assert!(old_buffer.is_none());

        Ok(())
    }

    fn end_frame(&self) -> Result<(), Box<dyn Error>> {
        let mut command_buffer = match self.current_command_buffer.take() {
            Some(cb) => cb,
            None => panic!("Can't end a frame before beginning it."),
        };

        let swapchain = match command_buffer.acquire_swapchain_texture(&self.window)? {
            Some(s) => s,
            None => {
                return Ok(());
            }
        };

        let (current_render_target, _) = self.get_render_target();

        let blit_info = BlitInfo::default()
            .with_source_texture(current_render_target)
            .with_source_region(
                0,
                0,
                0,
                current_render_target.width(),
                current_render_target.height(),
            )
            .with_destination_texture(&swapchain)
            .with_destination_region(0, 0, 0, swapchain.width(), swapchain.height())
            .with_filter(sdl3::gpu::Filter::Linear)
            .with_load_op(LoadOp::CLEAR)
            .with_clear_color(Color::RGB(0, 0, 0));

        command_buffer.blit_texture(blit_info);

        command_buffer.submit()?;

        Ok(())
    }

    fn blit_texture(&self, blit_info: BlitInfo) -> Result<(), Box<dyn Error>> {
        if let Some(cb) = self.current_command_buffer.borrow().as_ref() {
            cb.blit_texture(blit_info);
            Ok(())
        } else {
            Err(Box::new(InvalidOperationError::new(
                "Can't blit texture without beginning a frame.",
            )))
        }
    }
}

impl<'a> SdlRenderer<'a> {
    pub fn new(window: sdl3::video::Window) -> Result<Self, Box<dyn Error>> {
        let gpu = sdl3::gpu::Device::new(ShaderFormat::SPIRV, false)?.with_window(&window)?;

        let mut renderer = Self {
            gpu,
            window,
            backbuffers: [].into(),
            depthbuffers: [].into(),
            current_backbuffer_index: 0,

            current_command_buffer: RefCell::new(None),
            current_render_pass: RefCell::new(None),
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
                .with_num_levels(1)
                .with_layer_count_or_depth(1);

            let texture = self.gpu.create_texture(depthbuffer_creation_info)?;

            self.depthbuffers.push_back(texture);
        }

        Ok(())
    }
}
