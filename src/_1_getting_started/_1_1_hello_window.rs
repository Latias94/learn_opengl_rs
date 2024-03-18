const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// window functions will move to `window.rs`
pub fn main_1_1_1() {
    // Create a context from a WebGL2 context on wasm32 targets
    #[cfg(target_arch = "wasm32")]
    let (_gl, _shader_version, _window, _event_loop) = {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        use winit::platform::web::WindowExtWebSys;

        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        let window = winit::window::WindowBuilder::new()
            .with_title("Hello window")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
            .with_append(true)
            .build(&event_loop)
            .unwrap();
        let canvas = window.canvas().unwrap();
        let factor = window.scale_factor();
        canvas.set_height((HEIGHT as f32 / factor as f32) as u32);
        canvas.set_width((WIDTH as f32 / factor as f32) as u32);
        canvas.style().set_css_text(
            "position: absolute; top: 0; left: 0; background-color: black; display: block;",
        );

        let webgl2_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        let gl = glow::Context::from_webgl2_context(webgl2_context);

        (gl, "#version 300 es", window, event_loop)
    };

    // Create a context from a glutin window on non-wasm32 targets
    #[cfg(not(target_arch = "wasm32"))]
    let (_gl, _gl_surface, _gl_context, _shader_version, _window, event_loop) = {
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
            .with_title("Hello window")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT));

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
                major: 3,
                minor: 3,
            })))
            .build(raw_window_handle);

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .expect("Cannot create GL context 3.3")
        };

        let window = window.unwrap();

        let attrs = window.build_surface_attributes(Default::default());
        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        let gl =
            unsafe { glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s)) };

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

    // We handle events differently between targets
    #[cfg(not(target_arch = "wasm32"))]
    {
        use winit::event::{Event, WindowEvent};
        let _ = event_loop.run(move |event, elwt| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        // draw
                    }
                    _ => (),
                }
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    {
        // This could be called from `requestAnimationFrame`, a winit event loop, etc.
        // or use game-loop crate. see `src/window.rs`
        // draw
    }
}
