use rand::RngCore;
use wgpu::{util::DeviceExt, BindGroupLayoutEntry};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::NamedKey,
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Params {
    damping: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MergeSortParams {
    order_of_pass: u32,
    direction: u32,
    order_of_stage: u32,
}

pub struct MergeSort {
    compute_pipeline: wgpu::ComputePipeline,
}
impl MergeSort {
    pub fn new(device: &wgpu::Device) -> Self {
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("merge sort pipeline"),
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&device.create_bind_group_layout(
                        &wgpu::BindGroupLayoutDescriptor {
                            label: Some("Merge Sort Bind Group Layout"),
                            entries: &[
                                BindGroupLayoutEntry {
                                    binding: 0,
                                    count: None,
                                    ty: wgpu::BindingType::Buffer {
                                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                                        has_dynamic_offset: false,
                                        min_binding_size: None,
                                    },
                                    visibility: wgpu::ShaderStages::COMPUTE,
                                },
                                BindGroupLayoutEntry {
                                    binding: 1,
                                    count: None,
                                    ty: wgpu::BindingType::Buffer {
                                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                                        has_dynamic_offset: false,
                                        min_binding_size: None,
                                    },
                                    visibility: wgpu::ShaderStages::COMPUTE,
                                },
                                BindGroupLayoutEntry {
                                    binding: 2,
                                    count: None,
                                    ty: wgpu::BindingType::Buffer {
                                        ty: wgpu::BufferBindingType::Uniform,
                                        has_dynamic_offset: false,
                                        min_binding_size: None,
                                    },
                                    visibility: wgpu::ShaderStages::COMPUTE,
                                },
                            ],
                        },
                    )],
                    push_constant_ranges: &[],
                }),
            ),
            module: &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("bitonic sort shader module"),
                source: wgpu::ShaderSource::Wgsl(include_str!("bitonic_merge_sort.wgsl").into()),
            }),
            entry_point: "start",
        });
        Self { compute_pipeline }
    }
    pub fn sort(
        self: &Self,
        list: wgpu::Buffer,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::Buffer {
        let second = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 2048 * 2 * 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let bind_group_layout = self.compute_pipeline.get_bind_group_layout(0);

        let mut direction = 0;
        for i in 1..=11 {
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            for j in 0..i {
                let param_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("sort param buffer"),
                    contents: bytemuck::cast_slice(&[MergeSortParams {
                        order_of_pass: i - j,
                        direction,
                        order_of_stage: i,
                    }]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });
                let bind_group_entry_list_1 = wgpu::BindGroupEntry {
                    binding: 0,
                    resource: list.as_entire_binding(),
                };
                let bind_group_entry_list_2 = wgpu::BindGroupEntry {
                    binding: 1,
                    resource: second.as_entire_binding(),
                };
                let bind_group_entry_param = wgpu::BindGroupEntry {
                    binding: 2,
                    resource: param_buffer.as_entire_binding(),
                };
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &bind_group_layout,
                    entries: &[
                        bind_group_entry_list_1,
                        bind_group_entry_list_2,
                        bind_group_entry_param,
                    ],
                });
                let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Compute Pass"),
                    timestamp_writes: None,
                });
                compute_pass.set_pipeline(&self.compute_pipeline);
                compute_pass.set_bind_group(0, &bind_group, &[]);
                compute_pass.dispatch_workgroups(32, 1, 1);
                direction = 1 - direction;
            }
            queue.submit(std::iter::once(encoder.finish()));
        }
        match direction {
            0 => list,
            _ => second,
        }
    }
}
//doesn't work yet
struct BallSim {
    compute_pipeline: wgpu::ComputePipeline,
}
impl BallSim {
    fn new(device: &wgpu::Device) -> Self {
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&device.create_bind_group_layout(
                        &wgpu::BindGroupLayoutDescriptor {
                            label: None,
                            entries: &[
                                wgpu::BindGroupLayoutEntry {
                                    binding: 0,
                                    visibility: wgpu::ShaderStages::COMPUTE,
                                    ty: wgpu::BindingType::Texture {
                                        sample_type: wgpu::TextureSampleType::Float {
                                            filterable: false,
                                        },
                                        multisampled: false,
                                        view_dimension: wgpu::TextureViewDimension::D2,
                                    },
                                    count: None,
                                },
                                wgpu::BindGroupLayoutEntry {
                                    binding: 1,
                                    visibility: wgpu::ShaderStages::COMPUTE,
                                    ty: wgpu::BindingType::StorageTexture {
                                        access: wgpu::StorageTextureAccess::WriteOnly,
                                        format: wgpu::TextureFormat::Rgba8Unorm,
                                        view_dimension: wgpu::TextureViewDimension::D2,
                                    },
                                    count: None,
                                },
                                wgpu::BindGroupLayoutEntry {
                                    binding: 2,
                                    visibility: wgpu::ShaderStages::COMPUTE,
                                    ty: wgpu::BindingType::StorageTexture {
                                        access: wgpu::StorageTextureAccess::WriteOnly,
                                        format: wgpu::TextureFormat::Rgba8Unorm,
                                        view_dimension: wgpu::TextureViewDimension::D2,
                                    },
                                    count: None,
                                },
                                wgpu::BindGroupLayoutEntry {
                                    binding: 3,
                                    visibility: wgpu::ShaderStages::COMPUTE,
                                    ty: wgpu::BindingType::Buffer {
                                        ty: wgpu::BufferBindingType::Uniform,
                                        has_dynamic_offset: false,
                                        min_binding_size: None,
                                    },
                                    count: None,
                                },
                            ],
                        },
                    )],
                    push_constant_ranges: &[],
                }),
            ),
            module: &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Perlin Noise"),
                source: wgpu::ShaderSource::Wgsl(include_str!("ball_sim.wgsl").into()),
            }),
            entry_point: "start",
        });
        Self { compute_pipeline }
    }
}
struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
    merge_sort: MergeSort,
    rand_buffer: wgpu::Buffer,
}
impl<'a> State<'a> {
    async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_limits: wgpu::Limits::default(),
                    required_features: wgpu::Features::empty(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let mut noisy_data = [0; 4096 * 4];
        rand::thread_rng().fill_bytes(&mut noisy_data);
        let rand_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("to sort"),
            contents: &noisy_data,
            usage: wgpu::BufferUsages::STORAGE,
        });
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::STORAGE_BINDING,
            format: wgpu::TextureFormat::Rgba8Unorm,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };
        surface.configure(&device, &config);
        let merge_sort = MergeSort::new(&device);
        Self {
            surface,
            window,
            device,
            queue,
            config,
            size,
            merge_sort,
            rand_buffer,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }
    fn update(&mut self) {
        let mut noisy_data = [0; 4096 * 4];
        rand::thread_rng().fill_bytes(&mut noisy_data);
        self.rand_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("to sort"),
                contents: &noisy_data,
                usage: wgpu::BufferUsages::STORAGE,
            });
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        output.present();
        Ok(())
    }
}
pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    event_loop
        .run(|event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    logical_key: winit::keyboard::Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            state.resize(window.inner_size());
                        }
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if lost
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    control_flow.exit();
                                }
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }

                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                // RedrawRequested will only trigger once unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
