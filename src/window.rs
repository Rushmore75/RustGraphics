// TODO continue at https://sotrh.github.io/learn-wgpu/beginner/tutorial4-buffer/#the-index-buffer

use wgpu::{include_wgsl, util::DeviceExt};
use winit::{window::Window, event::WindowEvent, event_loop::ControlFlow, dpi::PhysicalSize};
use crate::{bounding_box::Boxes, peripherals::Peripheral, vertex::Vertex};

// TODO name this is a better fashion
pub struct StatusState {
    // These are all public because you can't really
    // mess them up that bad. They will be corrected on the next
    // frame from the event methods.
        pub is_focused: bool,
        pub button_widget: Boxes,
        pub peripheral: Peripheral, // State machine for peripherals
}

pub struct WindowState {
    // mission critical
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    bkgnd_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,
    pub status_state: StatusState,

}

impl WindowState {
    pub async fn new(window: &Window, button_widget: Boxes) -> Self {
        
        let size = window.inner_size();
        
        //===================================================
        //                      Canvas
        //===================================================

        // The instance is a handle to the GPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        // Creates a surface from a raw window handle.
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase { 
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
             },
            
        ).await.unwrap();
        // Requests a connection to a physical device, creating a logical device.
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                // https://docs.rs/wgpu/latest/wgpu/struct.Features.html
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // format: surface.get_preferred_format(&adapter).unwrap(),
            format: surface.get_supported_formats(&adapter)[0], // the first item is the preferred
            width: size.width,
            height: size.height,
            // https://docs.rs/wgpu/latest/wgpu/enum.PresentMode.html
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        //===================================================
        //               Pipelines & Coloring
        //===================================================

        let clear_color = wgpu::Color::WHITE;
        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),

            // vertex shader
            vertex: wgpu::VertexState { 
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                ]
            },
            // fragment shader
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                // TODO change the struct in the shader file to allow for values to be passed in. 
                // these values can be then changed to change the values of the vec4 in the fragment shader.
                // This will change the color of the triangle that gets rendered.
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
       
        //===================================================
        //                      Buffers
        //===================================================
        
        // something to "render" on setup, it will quickly get overridden in the main loop
        const DUMMY_VERTICES: &[Vertex] = &[Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.0, 0.0] }];
        const DUMMY_INDICES: &[u16] = &[0];
        
        let num_vertices = DUMMY_VERTICES.len() as u32;
        let num_indices = DUMMY_INDICES.len() as u32;
        
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(DUMMY_VERTICES),
                usage: wgpu::BufferUsages::VERTEX, 
            }
        );
    
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(DUMMY_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let extra_state = StatusState {
            is_focused: true,
            button_widget,
            peripheral: Peripheral::new(),
        };

        Self {
            surface,
            device,
            queue,
            config,
            bkgnd_color: clear_color,
            size,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            num_vertices,
            status_state: extra_state,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        //https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#resize
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
    
    /** This function only handles ***Window*** events */
    pub fn window_events(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {

        match event {
            // ====================================================================
            //                          Input related 
            // ====================================================================
            // !! Everything that isn't updating state is in the tick() function !!
            //
            WindowEvent::CursorMoved { position, .. } => {
                // NOTE: this function get's called ***A LOT*** don't do
                // anything dumb in here.
                self.status_state.peripheral.update_pointer([position.x, position.y]);
            },

            WindowEvent::MouseInput { state, button, .. } => {
                self.status_state.peripheral.update_mouse(button, state);
            },

            WindowEvent::MouseWheel { device_id, delta, phase, .. } => {
                // TODO dafuq?
                (phase, delta, device_id);
            },

            WindowEvent::KeyboardInput { input, .. } => {
                // virtual_keycode is None if it is an unrecognized key
                match input.virtual_keycode {
                    Some(keycode) => {
                        // update the state of the key
                        self.status_state.peripheral.update_key(&keycode, &input.state);
                    },
                    /* Keys that result in None:
                        * number pad 5 (num lock off)
                        * meta key
                        * print screen
                        * caps lock
                        * scroll lock 
                        * extra keyboard-specific keys
                    */
                    None => invalid_keypress(),
                }

                fn invalid_keypress() {
                    println!("Invalid keypress! (You can probably ignore this message)")
                }
            },

            //=======================================================
            //      WindowEvents that are non-input related 
            //=======================================================
            WindowEvent::Focused(focused) => {
                self.status_state.is_focused = *focused;
                println!("Focus is now: {}", focused);
            },

            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            },
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
            }
                
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize(**new_inner_size);
            }
                
            _ => {}
        };

    }

    /// Add points to the next buffer
    pub fn queue_buffer(&mut self, vertices: &[Vertex], indices: &[u16]) {

        // TODO this would benefit from State being set up with a builder
        // pattern. It would allow this to be a generalized function for easier use later.
        self.vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX, 
            }
        );
        self.num_vertices = vertices.len() as u32;

        self.index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        self.num_indices = indices.len() as u32;

    }

    /// Pushes the buffers to the gpu for rendering
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                // This is what @location(0) in the fragment shader targets    
                // TODO somehow pass more values into the fragment shader
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.bkgnd_color),
                            store: true,
                        },
                    })],
                depth_stencil_attachment: None,
            });
            /*  
            these brackets drop render_pass because now it goes out of scope
            `drop(render_pass)`
            would do the same thing, we need encoder back
            */ 
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Get size of window
    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }


}

