use crate::sections;
use dioxus::logger::tracing::*;
use dioxus::prelude::*;
#[cfg(feature = "web")]
use std::borrow::Cow;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::JsCast;

#[cfg(feature = "web")]
use dioxus_web::WebEventExt;

mod entries;

#[cfg(feature = "web")]
async fn request_animation_frame(window: &web_sys::Window) -> Result<(), String> {
    let (s, r) = oneshot::channel();

    let closure = web_sys::wasm_bindgen::prelude::Closure::once(move || s.send(()).unwrap());

    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .map_err(|err| format!("{:?}", err.as_string()))?;

    r.await.map_err(|err| format!("{err}"))?;
    Ok(())
}

#[cfg(feature = "web")]
#[wasm_bindgen]
pub fn hello_world() {
    web_sys::window()
        .unwrap()
        .alert_with_message("Hello world!")
        .unwrap();
}

#[cfg(feature = "web")]
async fn run(extern_module: String, html_canvas: web_sys::HtmlCanvasElement) -> Result<(), String> {
    let window = web_sys::window().ok_or("Failed to get window")?;
    let module =
        wasm_bindgen_futures::JsFuture::from(web_sys::js_sys::WebAssembly::instantiate_streaming(
            &window.fetch_with_str(&extern_module),
            &web_sys::js_sys::Object::new(),
        ))
        .await
        .map_err(|err| format!("{:?}", err.as_string()))?;
    info!("Module: {:?}", module);
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
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
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("blog/shader.wgsl"))),
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
            info!(
                "Resizing canvas with width: {} and height: {}",
                width, height
            );
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
}

#[component]
fn BlogEntry(blog_date: sections::BlogDate) -> Element {
    let blog_entry = entries::get_blog(blog_date);

    #[allow(unused_mut)]
    let mut errorMessage = use_signal(|| Option::<String>::None);

    #[allow(unused_variables)]
    let mountedFn = move |event: dioxus::prelude::Event<MountedData>| async move {
        #[cfg(feature = "web")]
        {
            let element = event
                .as_web_event()
                .clone()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let result = run(
                "/docs/assets/dioxus/personal-webpage_bg.wasm".into(),
                element,
            )
            .await
            .err();
            if let Some(result) = &result {
                warn!("Error: {:?}", result);
            }
            errorMessage.set(result);
        }
    };

    rsx! {
        div { class: "bg-primary min-h-screen", id: "blog-post",
            div {
                class: "group relative py-16 h-72 bg-cover bg-center bg-no-repeat sm:h-84 lg:h-64 xl:h-72 sm:py-20",
                style: "background-image: url({blog_entry.image_file_blog}); background-size: contain",
                span { class: "absolute inset-0 block bg-gradient-to-b from-blog-gradient-from to-blog-gradient-to bg-cover bg-center bg-no-repeat opacity-50" }
                span { class: "absolute top-0 left-0 w-full h-full block bg-opacity-100",
                    h2 { class: "text-center font-header text-4xl font-semibold uppercase text-white sm:text-5xl lg:text-6xl",
                        {blog_entry.title}
                    }
                    h4 { class: "text-center font-header text-4xl font-thin uppercase text-white sm:text-5xl lg:text-6xl",
                        {blog_entry.description}
                    }
                }
            }
            if let Some(msg) = errorMessage() {
                p { class: "text-center text-error", {msg} }
            } else {
                canvas {
                    onmounted: mountedFn,
                    id: "render-canvas",
                    class: "w-full h-full",
                    width: 1000,
                    height: 1000,
                }
            }
        }
    }
}

fn change_section(section: sections::ActiveSection) {
    let nav = navigator();
    nav.push(crate::Route::NavBar {
        route: section.into(),
    });
}

#[component]
fn BlogCard(blog_entry: &'static entries::BlogEntry) -> Element {
    let href = format!("/blog/{}/index.html", blog_entry.blog_date);
    let image_thumbnail = &blog_entry.image_file_thumbnail;
    let title = &blog_entry.title;
    let description = &blog_entry.description;
    rsx! {
        a {
            href,
            class: "shadow",
            onclick: move |event| {
                event.prevent_default();
                change_section(sections::ActiveSection::Blog(Some(blog_entry.blog_date)));
            },
            div {
                style: "background-image: url({image_thumbnail})",
                class: "group relative h-72 bg-cover bg-center bg-no-repeat sm:h-84 lg:h-64 xl:h-72",
                span { class: "absolute inset-0 block bg-gradient-to-b from-blog-gradient-from to-blog-gradient-to bg-cover bg-center bg-no-repeat opacity-10 transition-opacity group-hover:opacity-50" }
                span { class: "absolute right-0 bottom-0 mr-4 mb-4 block rounded-full border-2 border-secondary px-6 py-2 text-center font-body text-sm font-bold uppercase text-secondary md:text-base",
                    "Read More"
                }
            }
            div { class: "bg-white py-6 px-5 xl:py-8",
                span { class: "block font-body text-lg font-semibold text-black", {title} }
                span { class: "block pt-2 font-body text-grey-20", {description} }
            }
        }
    }
}

#[component]
fn BlogMenu() -> Element {
    let posts_entries = entries::get_blogs();

    let posts = posts_entries.iter().map(|(_, blog_entry)| {
        rsx! {
            BlogCard { blog_entry }
        }
    });

    rsx! {
        div { class: "bg-primary min-h-screen", id: "blog",
            div { class: "container py-16 md:py-20",
                h2 { class: "text-center font-header text-4xl font-semibold uppercase text-white sm:text-5xl lg:text-6xl",
                    "Some of my ramblings"
                }
                div { class: "mx-auto grid w-full grid-cols-1 gap-6 pt-12 sm:w-3/4 lg:w-full lg:grid-cols-3 xl:gap-10",
                    {posts}
                }
            }
        }
    }
}

#[component]
pub fn Blog(blog_date: Option<sections::BlogDate>) -> Element {
    info!("Creating blog");

    if let Some(blog_date) = blog_date {
        rsx! {
            BlogEntry { blog_date }
        }
    } else {
        rsx! {
            BlogMenu {}
        }
    }
}
