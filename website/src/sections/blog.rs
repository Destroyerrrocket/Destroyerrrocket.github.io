use crate::sections;
use dioxus::logger::tracing::*;
use dioxus::prelude::*;
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::JsCast;

#[cfg(feature = "web")]
use dioxus_web::WebEventExt;

mod entries;

#[cfg(feature = "web")]
async fn run(extern_module: String, id: String) -> dioxus::Result<JsValue> {
    use web_sys::js_sys;
    info!("id: {}", id);
    // import init, {{ run }} from '{}';
    // await module.init();
    let result = js_sys::eval(
        format!(
            "
        async function run_async() {{
            const {{
                default: init,
                run,
            }} = await import('{}');
            await init();
            return await run('{}');
        }}
        run_async",
            extern_module, id
        )
        .as_str(),
    )
    .map_err(|err| dioxus::CapturedError::from_display(format!("{:#?}", err)))?
    .dyn_into::<js_sys::Function>()
    .map_err(|err| dioxus::CapturedError::from_display(format!("{:#?}", err)))?;

    let result = result
        .call0(&JsValue::null())
        .map_err(|err| dioxus::CapturedError::from_display(format!("{:#?}", err)))?
        .dyn_into::<js_sys::Promise>()
        .map(wasm_bindgen_futures::JsFuture::from)
        .map_err(|err| dioxus::CapturedError::from_display(format!("{:#?}", err)))?
        .await
        .map_err(|err| dioxus::CapturedError::from_display(format!("{:#?}", err)))?;

    info!("Result: {:?}", result);
    Ok(result)
}

#[component]
fn BlogMarkdown(blog_date: sections::BlogDate) -> Element {
    #[cfg(feature = "web")]
    let markdown: Resource<Result<String, JsValue>> = use_resource(move || async move {
        let opts = web_sys::RequestInit::new();

        opts.set_method("GET");
        opts.set_mode(web_sys::RequestMode::SameOrigin);

        let html = entries::get_blog(blog_date).html;
        let url = html.to_string();

        let request = web_sys::Request::new_with_str_and_init(&url, &opts)?;
        let window = web_sys::window().unwrap();
        let resp_value =
            wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<web_sys::Response>());
        let resp: web_sys::Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let text = wasm_bindgen_futures::JsFuture::from(resp.text()?).await?;

        // Send the JSON response back to JS.
        Ok(text.as_string().unwrap())
    });

    #[cfg(not(feature = "web"))]
    let markdown: Resource<Result<String, String>> = use_resource(move || async move {
        info!("Returning an empty blog");
        Ok("".to_string())
    });

    match &*markdown.read_unchecked() {
        Some(Ok(response)) => rsx! {
            div { class: "container min-w-full px-0 mx-0 py-10",
                div { class: "w-full py-10 px-10 md:mx-auto md:py-12 md:px-12 md:max-w-4xl md:rounded-lg bg-secondary",
                    div { class: "w-full *:xl:gap-8",
                        div {
                            class: "prose prose-invert max-w-full",
                            dangerous_inner_html: "{response}",
                        }
                    }
                }
            
            }
        },
        Some(Err(_)) => rsx! {
            div { "Loading blog failed" }
        },
        None => rsx! {
            div { "Loading blog..." }
        },
    }
}

#[component]
fn BlogEntry(blog_date: sections::BlogDate) -> Element {
    let blog_entry = entries::get_blog(blog_date);

    #[allow(unused_mut)]
    let mut errorMessage = use_signal(|| Option::<String>::None);

    #[allow(unused_variables)]
    let mountedFn = move |event: dioxus::prelude::Event<MountedData>| async move {
        info!("Canvas mounted");
        #[cfg(feature = "web")]
        {
            let element = event.as_web_event().clone().id();
            let result = run(
                "/assets/wasm/triangle-demo/triangle_demo.js".into(),
                element,
            )
            .await
            .err();
            if let Some(result) = &result {
                warn!("Error: {}", result);
                errorMessage.set(Some(format!("{}", result)));
            }
        }
    };

    rsx! {
        div { class: "bg-primary min-h-screen", id: "blog-post",
            div {
                class: "group relative py-16 h-72 bg-cover bg-center bg-no-repeat sm:h-84 lg:h-64 xl:h-72 sm:py-20",
                style: "background-image: url({blog_entry.image_file_blog}); background-size: contain",
                span { class: "absolute inset-0 block bg-gradient-to-b from-blog-gradient-from to-blog-gradient-to bg-cover bg-center bg-no-repeat opacity-50" }
                span { class: "absolute top-12 left-0 w-full h-full block bg-opacity-100",
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
                p { class: "text-center text-white-text",
                    "I'm currently running an experiment with WebGPU. This may not work on all browsers."
                }
            } else {
                canvas {
                    onmounted: mountedFn,
                    id: "render-canvas",
                    class: "w-full h-full",
                    width: 1000,
                    height: 1000,
                }
            }
            BlogMarkdown { blog_date }
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
