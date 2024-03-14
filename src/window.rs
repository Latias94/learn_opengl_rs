pub trait Application {
    fn init(&mut self, ctx: &GLContext);
    fn update(&mut self, ctx: &GLContext);
    fn handle_event(&mut self, ctx: &GLContext);
    fn exit(&mut self, ctx: &GLContext);
}

pub struct WindowInitInfo {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub major: u8,
    pub minor: u8,
}

pub struct GLContext {
    pub gl: glow::Context,
    pub shader_version: &'static str,
}

pub unsafe fn run<App: Application>(init_info: WindowInitInfo, mut app: App) {
    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        #[cfg(target_arch = "wasm32")]
        let (gl, shader_version) = {
            use wasm_bindgen::JsCast;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            let gl = glow::Context::from_webgl2_context(webgl2_context);
            (gl, "#version 300 es")
        };

        let width = init_info.width;
        let height = init_info.height;
        let title = init_info.title;
        let major = init_info.major;
        let minor = init_info.minor;

        // Create a context from a glutin window on non-wasm32 targets
        #[cfg(feature = "glutin_winit")]
        let (gl, gl_surface, gl_context, shader_version, _window, event_loop) = {
            use glutin::{
                config::{ConfigTemplateBuilder, GlConfig},
                context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext},
                display::{GetGlDisplay, GlDisplay},
                surface::{GlSurface, SwapInterval},
            };
            use glutin_winit::{DisplayBuilder, GlWindow};
            use raw_window_handle::HasRawWindowHandle;
            use std::num::NonZeroU32;

            let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
            let window_builder = winit::window::WindowBuilder::new()
                .with_title(title.as_str())
                .with_inner_size(winit::dpi::LogicalSize::new(width, height));

            let template = ConfigTemplateBuilder::new();

            let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

            let (window, gl_config) = display_builder
                .build(&event_loop, template, |configs| {
                    configs
                        .reduce(|accum, config| {
                            if config.num_samples() > accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                })
                .unwrap();

            let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

            let gl_display = gl_config.display();
            let context_attributes = ContextAttributesBuilder::new()
                .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                    major,
                    minor,
                })))
                .build(raw_window_handle);

            let not_current_gl_context = gl_display
                .create_context(&gl_config, &context_attributes)
                .expect("Cannot create GL context");

            let window = window.unwrap();

            let attrs = window.build_surface_attributes(Default::default());
            let gl_surface = gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap();

            let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

            let gl = glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));

            gl_surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();

            (
                gl,
                gl_surface,
                gl_context,
                "#version 410",
                window,
                event_loop,
            )
        };

        // Create a context from a sdl2 window
        #[cfg(feature = "sdl2")]
        let (gl, shader_version, window, mut events_loop, _context) = {
            let sdl = sdl2::init().unwrap();
            let video = sdl.video().unwrap();
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(major, minor);
            let window = video
                .window(title.as_str(), width, height)
                .opengl()
                .resizable()
                .build()
                .unwrap();
            let gl_context = window.gl_create_context().unwrap();
            let gl =
                glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _);
            let event_loop = sdl.event_pump().unwrap();
            (gl, "#version 130", window, event_loop, gl_context)
        };

        let ctx = GLContext { gl, shader_version };

        app.init(&ctx);

        // We handle events differently between targets
        #[cfg(feature = "glutin_winit")]
        {
            use glutin::prelude::GlSurface;
            use winit::event::{Event, WindowEvent};
            let _ = event_loop.run(move |event, elwt| {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                            app.exit(&ctx);
                        }
                        WindowEvent::RedrawRequested => {
                            app.update(&ctx);
                            gl_surface.swap_buffers(&gl_context).unwrap();
                        }
                        _ => (),
                    }
                }
            });
        }

        #[cfg(feature = "sdl2")]
        {
            let mut running = true;
            while running {
                {
                    for event in events_loop.poll_iter() {
                        match event {
                            sdl2::event::Event::Quit { .. } => running = false,
                            _ => {}
                        }
                    }
                }
                app.update(&ctx);
                window.gl_swap_window();
            }
            app.exit(&ctx);
        }

        #[cfg(target_arch = "wasm32")]
        {
            // This could be called from `requestAnimationFrame`, a winit event
            // loop, etc.
            app.update(&ctx);
            app.exit(&ctx);
        }
    }
}
