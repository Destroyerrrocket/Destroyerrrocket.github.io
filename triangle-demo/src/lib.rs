use log::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) {
    web_sys::console::log_1(&format!("Hello, {}!", name).into());
}

async fn request_animation_frame(window: &web_sys::Window) -> Result<(), String> {
    let (s, r) = oneshot::channel();

    let closure = web_sys::wasm_bindgen::prelude::Closure::once(move || s.send(()).unwrap());

    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .map_err(|err| format!("{:?}", err.as_string()))?;

    r.await.map_err(|err| format!("{err}"))?;
    Ok(())
}

#[wasm_bindgen]
pub async fn run(id: String) -> Result<JsValue, JsValue> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });

    let window = web_sys::window().ok_or("Failed to get window")?;

    let html_canvas: web_sys::HtmlCanvasElement = window
        .document()
        .ok_or("Failed to get document")?
        .get_element_by_id(&id)
        .ok_or("Failed to get canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let surface_target = wgpu::SurfaceTarget::Canvas(html_canvas.clone());
    let surface = instance.create_surface(surface_target).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .ok_or("Failed to find an appropriate adapter")?;

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .map_err(|err| format!("{err}"))?;

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let mut width: i32 = -1;
    let mut height: i32 = -1;

    let mut i = 0;
    loop {
        i += 1;
        let curr_width = html_canvas.client_width() as u32;
        let curr_height = html_canvas.client_height() as u32;

        if curr_width as i64 != width as i64 || curr_height as i64 != height as i64 {
            width = curr_width as i32;
            height = curr_height as i32;
            /*info!(
                "Resizing canvas with width: {} and height: {}",
                width, height
            )*/
            let config = surface
                .get_default_config(&adapter, width as u32, height as u32)
                .ok_or("Failed to get default config")?;
            surface.configure(&device, &config);

            html_canvas.set_width(curr_width);
            html_canvas.set_height(curr_height);
        }

        if (i % 60) == 0 {
            info!("Frame: {}", i);
        }
        let frame = surface
            .get_current_texture()
            .map_err(|err| format!("{err}"))?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();

        request_animation_frame(&window).await?;
    }
    Ok("".into())
}
