use iced::advanced::graphics::core::Element;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use cosmic_text::{FontSystem, SwashCache, Buffer as TextBuffer};
use iced::{
    widget::{column, container, text, scrollable},
    Alignment, Length,
};
use crate::config::theme::WarpTheme;
use crate::block::Block; // Import Block

pub struct GpuRenderer {
    device: Device,
    queue: Queue,
    surface: Surface,
    config: SurfaceConfiguration,
    font_system: FontSystem,
    swash_cache: SwashCache,
    renderer: Renderer,
    
    // Performance metrics
    frame_time: std::time::Duration,
    fps: f32,
}

impl GpuRenderer {
    pub async fn new(window: &winit::window::Window) -> Result<Self, Box<dyn std::error::Error>> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(window)? };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find adapter")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let renderer = Renderer::new();

        Ok(GpuRenderer {
            device,
            queue,
            surface,
            config,
            font_system,
            swash_cache,
            renderer,
            frame_time: std::time::Duration::from_millis(16),
            fps: 60.0,
        })
    }

    pub fn render_frame(&mut self, content: &str) -> Result<(), wgpu::SurfaceError> {
        let start_time = std::time::Instant::now();

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.02,
                            g: 0.02,
                            b: 0.02,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Call the new render_text method
            self.renderer.render_frame(content);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // Update performance metrics
        self.frame_time = start_time.elapsed();
        self.fps = 1.0 / self.frame_time.as_secs_f32();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn get_fps(&self) -> f32 {
        self.fps
    }

    pub fn get_frame_time(&self) -> std::time::Duration {
        self.frame_time
    }
}

pub struct Renderer {
    // Renderer might hold state related to rendering performance,
    // or cached rendering artifacts.
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    /// Renders a list of `Block`s into an Iced `Element`.
    pub fn view_blocks<'a>(blocks: &'a [Block], theme: &WarpTheme) -> Element<'a, crate::terminal::Message> {
        let content = blocks.iter().fold(column![], |col, block| {
            col.push(block.view(theme))
        })
        .spacing(10)
        .width(Length::Fill)
        .align_items(Alignment::Start);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Renders a simple text message.
    pub fn view_message<'a>(message: &str, theme: &WarpTheme) -> Element<'a, crate::terminal::Message> {
        container(
            text(message)
                .size(18)
                .color(theme.get_foreground_color())
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    pub fn render_frame(&self, content: &str) {
        println!("Rendering frame with content:\n{}", content);
    }
}

// Shader for smooth scrolling and blur effects
pub const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 tex_coords;

layout(location = 0) out vec2 v_tex_coords;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 transform;
    float time;
    float scroll_offset;
};

void main() {
    v_tex_coords = tex_coords;
    
    // Apply smooth scrolling transformation
    vec2 scroll_pos = position + vec2(0.0, scroll_offset);
    
    gl_Position = transform * vec4(scroll_pos, 0.0, 1.0);
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) in vec2 v_tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 1) uniform texture2D t_diffuse;
layout(set = 0, binding = 2) uniform sampler s_diffuse;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 transform;
    float time;
    float scroll_offset;
};

void main() {
    vec4 tex_color = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    
    // Apply subtle blur effect for smooth rendering
    vec2 blur_offset = vec2(0.001, 0.001);
    vec4 blur_color = (
        texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords + blur_offset) +
        texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords - blur_offset) +
        texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords + vec2(blur_offset.x, -blur_offset.y)) +
        texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords + vec2(-blur_offset.x, blur_offset.y))
    ) * 0.25;
    
    f_color = mix(tex_color, blur_color, 0.1);
}
"#;
