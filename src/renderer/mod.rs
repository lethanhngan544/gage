use std::collections::HashMap;


use crate::engine::types::{CVarValue, EngineCvar, Message, MessageQueue};

pub mod static_model;

pub enum DrawCommands {
    StaticModel { id: uuid::Uuid },
}

pub struct Renderer<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    draw_commands: Vec<DrawCommands>,

    static_model_renderer: static_model::StaticModelRenderer,
    static_model_paths: HashMap<String, uuid::Uuid>,
    static_models: HashMap<uuid::Uuid, static_model::StaticModel>,
}

impl<'a> Renderer<'a> {
    pub fn new(window: &'a glfw::PWindow, cvar: &mut EngineCvar) -> Self {

        let width = cvar.get("window_width").unwrap().as_int();
        let height = cvar.get("window_height").unwrap().as_int();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12,
            ..Default::default()
        });
        let surface = instance.create_surface(window).unwrap();
        let adapter = pollster::block_on(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })).unwrap();

        let (device, queue) = pollster::block_on(adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::PUSH_CONSTANTS,
                required_limits: wgpu::Limits {
                    max_push_constant_size: 128,
                    ..wgpu::Limits::defaults()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })).unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: width as u32,
            height: height as u32,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let static_model_renderer = static_model::StaticModelRenderer::new(&device, &surface_format);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
            draw_commands: vec![],
            static_model_renderer,
            static_model_paths: HashMap::new(),
            static_models: HashMap::new(),
        }
    }

    // fn allocate_buffer(&self, label: &str, data: &'a [u8], usage: wgpu::BufferUsages) {
    //     let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //         label: Some(label),
    //         contents: data,
    //         usage: usage
    //     });
    //     let uuid = uuid::Uuid::new_v4();
        
    // }

    pub fn on_message(&mut self, message: Message, messages: &mut MessageQueue, cvar: &EngineCvar) {
        match message {
            Message::WindowResized(width, height) => {
                self.config.width = width;
                self.config.height = height;
                self.surface.configure(&self.device, &self.config);
            }
            Message::LoadStaticModel { path } => {
                //Check if model is already loaded
                if self.static_model_paths.contains_key(&path) {
                    log::info!("Static model: {} already loaded loading from cache !", path);
                    messages.push_back(Message::StaticModelReady { id: self.static_model_paths.get(&path).unwrap().clone() });
                }
                else {
                    //TODO: Asynchronously load model
                    let model = static_model::StaticModel::new(&self.device);
                    let uuid = uuid::Uuid::new_v4();
                    self.static_model_paths.insert(path.clone(), uuid.clone());
                    self.static_models.insert( uuid.clone(), model);   
                    messages.push_back(Message::StaticModelReady { id: uuid });
                }

               
                
            }
            _ => {}
        }
    }
    pub fn render(&self, cvar: &EngineCvar) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });



        //Main pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            render_pass.set_pipeline(self.static_model_renderer.get_pipeline());
            for cmd in &self.draw_commands {
                match cmd {
                    DrawCommands::StaticModel { id } => {
                        if let Some(model) = self.static_models.get(id) {
                            for raw_mesh in &model.raw_meshes {
                                render_pass.set_vertex_buffer(0, raw_mesh.vertex_buffer.slice(..));
                                render_pass.set_index_buffer(raw_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                                render_pass.draw_indexed(0..raw_mesh.num_elements, 0, 0..1);
                            }
                        }
                    }
                }
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
    pub fn add_draw_command(&mut self, cmd: DrawCommands) {
        self.draw_commands.push(cmd);
    }
    pub fn clear_draw_commands(&mut self) {
        self.draw_commands.clear();
    }
}