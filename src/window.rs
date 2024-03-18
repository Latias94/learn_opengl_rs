use game_loop::TimeTrait;
use std::sync::Arc;
use std::time::Duration;
use typed_builder::TypedBuilder;
use winit_input_helper::WinitInputHelper;

pub const UPDATE_PER_SECOND: usize = 240;
#[allow(dead_code)]
pub const FPS: usize = 60;
#[allow(dead_code)]
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

pub struct Game<A: Application> {
    input: WinitInputHelper,
    app: A,
    ctx: GLContext,
}

pub trait Application {
    fn new(_ctx: &GLContext) -> Self;
    fn init(&mut self, _ctx: &GLContext) {}
    fn render(&mut self, _ctx: &GLContext) {}
    fn update(&mut self, _update_delta_time: f32) {}
    fn resize(&mut self, _ctx: &GLContext, _width: u32, _height: u32) {}
    fn process_input(&mut self, _ctx: &GLContext, _input: &WinitInputHelper) {}
    fn exit(&mut self, _ctx: &GLContext) {}
}

#[derive(TypedBuilder, Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowInitInfo {
    #[builder(default = 800)]
    pub width: u32,
    #[builder(default = 600)]
    pub height: u32,
    #[builder(default = "learn_opengl_rs".to_string())]
    pub title: String,
    #[builder(default = 3)]
    pub major: u8,
    #[builder(default = 3)]
    pub minor: u8,
}

pub struct GLContext {
    pub gl: glow::Context,
    pub suggested_shader_version: &'static str,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
    pub start: chrono::DateTime<chrono::Utc>,
    pub last_update_time: chrono::DateTime<chrono::Utc>,
    pub last_render_time: chrono::DateTime<chrono::Utc>,
    pub update_delta_time: f32,
    pub render_delta_time: f32,
}

pub unsafe fn run<App: Application + 'static>(init_info: WindowInitInfo) {
    unsafe {
        let width = init_info.width;
        let height = init_info.height;
        let title = init_info.title;

        // Create a context from a WebGL2 context on wasm32 targets
        #[cfg(target_arch = "wasm32")]
        let (gl, shader_version, window, event_loop) = {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Cannot init logger");

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
            let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
            let window = winit::window::WindowBuilder::new()
                .with_title(title.as_str())
                .with_inner_size(winit::dpi::LogicalSize::new(width, height))
                .build(&event_loop)
                .unwrap();
            (gl, "#version 300 es", window, event_loop)
        };

        // Create a context from a glutin window on non-wasm32 targets
        #[cfg(not(target_arch = "wasm32"))]
        let (gl, gl_surface, gl_context, shader_version, window, event_loop) = {
            use glutin::{
                config::{ConfigTemplateBuilder, GlConfig},
                context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext},
                display::{GetGlDisplay, GlDisplay},
                surface::{GlSurface, SwapInterval},
            };
            use glutin_winit::{DisplayBuilder, GlWindow};
            use raw_window_handle::HasRawWindowHandle;
            use std::num::NonZeroU32;

            let major = init_info.major;
            let minor = init_info.minor;

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
                "#version 330 core",
                window,
                event_loop,
            )
        };

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let scale_factor = 1.0;
            } else {
                let scale_factor= window.scale_factor();
            }
        }
        let ctx = GLContext {
            gl,
            suggested_shader_version: shader_version,
            width,
            height,
            scale_factor,
            start: chrono::Utc::now(),
            last_update_time: chrono::Utc::now(),
            last_render_time: chrono::Utc::now(),
            update_delta_time: 0.0,
            render_delta_time: 0.0,
        };

        let mut app = App::new(&ctx);
        app.init(&ctx);

        let game = Game {
            input: WinitInputHelper::new(),
            app,
            ctx,
        };

        let window = Arc::new(window);
        game_loop::game_loop(
            event_loop,
            window,
            game,
            UPDATE_PER_SECOND as u32,
            0.1,
            move |g| {
                let ctx = &mut g.game.ctx;
                let now = chrono::Utc::now();
                ctx.update_delta_time =
                    (now - ctx.last_update_time).num_milliseconds() as f32 / 1000.0;
                ctx.last_update_time = chrono::Utc::now();
                g.game.app.update(ctx.update_delta_time);
            },
            move |g| {
                let ctx = &mut g.game.ctx;
                let now = chrono::Utc::now();
                ctx.render_delta_time =
                    (now - ctx.last_render_time).num_milliseconds() as f32 / 1000.0;
                ctx.last_render_time = chrono::Utc::now();
                g.game.app.render(ctx);
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use glutin::surface::GlSurface;
                    gl_surface.swap_buffers(&gl_context).unwrap();

                    let dt =
                        TIME_STEP.as_secs_f64() - game_loop::Time::now().sub(&g.current_instant());
                    if dt > 0.0 {
                        std::thread::sleep(Duration::from_secs_f64(dt));
                    }
                }
            },
            move |g, event| {
                let input = &mut g.game.input;
                let app = &mut g.game.app;
                let ctx = &mut g.game.ctx;

                if input.update(event) {
                    if input.key_pressed(winit::keyboard::KeyCode::Escape)
                        || input.close_requested()
                        || input.destroyed()
                    {
                        log::info!("Exiting");
                        app.exit(ctx);
                        g.exit();
                        return;
                    }
                    if let Some(size) = input.window_resized() {
                        let (width, height) = (size.width, size.height);
                        log::info!("Resizing to {}x{}", width, height);
                        app.resize(ctx, width, height);
                        return;
                    }

                    app.process_input(ctx, input);
                }
            },
        )
        .unwrap();
    }
}
