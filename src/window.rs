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
    ctx: AppContext,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
pub enum UserEvent {
    Redraw(Duration),
}

pub trait Application: Sized {
    async unsafe fn new(_ctx: &AppContext) -> Self;
    fn ui(&mut self, _state: &AppState, _egui_ctx: &egui::Context) {}
    unsafe fn render(&mut self, _ctx: &AppContext) {}
    unsafe fn update(&mut self, _update_delta_time: f32) {}
    unsafe fn resize(&mut self, _ctx: &AppContext, _width: u32, _height: u32) {}
    unsafe fn process_input(&mut self, _ctx: &AppContext, _input: &WinitInputHelper) {}
    unsafe fn exit(&mut self, _ctx: &AppContext) {}
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

pub struct AppContext {
    #[cfg(not(target_arch = "wasm32"))]
    pub egui_glow: egui_glow::EguiGlow,
    pub gl_context: GLContext,
    pub state: AppState,
}

pub struct GLContext {
    pub gl: Arc<glow::Context>,

    #[cfg(not(target_arch = "wasm32"))]
    pub gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
}

pub struct AppState {
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

impl AppContext {
    pub fn gl(&self) -> &glow::Context {
        &self.gl_context.gl
    }

    pub fn height(&self) -> u32 {
        self.state.height
    }

    pub fn width(&self) -> u32 {
        self.state.width
    }

    #[allow(dead_code)]
    pub fn scale_factor(&self) -> f64 {
        self.state.scale_factor
    }
    pub fn update_delta_time(&self) -> f32 {
        self.state.update_delta_time
    }

    pub fn render_delta_time(&self) -> f32 {
        self.state.render_delta_time
    }

    pub fn suggested_shader_version(&self) -> &'static str {
        self.state.suggested_shader_version
    }
}

pub async unsafe fn run<App: Application + 'static>(init_info: WindowInitInfo) {
    let width = init_info.width;
    let height = init_info.height;
    let title = init_info.title;

    // Create a context from a WebGL2 context on wasm32 targets
    #[cfg(target_arch = "wasm32")]
    let (gl, shader_version, window, event_loop) = {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("Cannot init logger");

        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        use winit::platform::web::WindowExtWebSys;

        let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
        let window = winit::window::WindowBuilder::new()
            .with_title(title.as_str())
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .with_append(true)
            .build(&event_loop)
            .unwrap();
        let web_width = width as f32;
        let web_height = height as f32;
        let canvas = window.canvas().unwrap();
        canvas
            .style()
            .set_css_text("background-color: black; display: block; margin: 20px auto;");
        // From winit 0.29, canvas size can't be set by request_inner_size or canvas.set_width
        let scale_factor = window.scale_factor() as f32;
        canvas.set_width((web_width * scale_factor) as u32);
        canvas.set_height((web_height * scale_factor) as u32);
        canvas.style().set_css_text(
            &(canvas.style().css_text()
                + &format!("width: {}px; height: {}px", web_width, web_height)),
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

        let event_loop = winit::event_loop::EventLoopBuilder::<UserEvent>::with_user_event()
            .build()
            .unwrap();
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

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .expect("Cannot create GL context")
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
    #[allow(clippy::arc_with_non_send_sync)]
    let gl = Arc::new(gl);

    #[cfg(not(target_arch = "wasm32"))]
    let egui_glow = {
        let egui_glow = egui_glow::EguiGlow::new(&event_loop, gl.clone(), None, None);
        let event_loop_proxy = egui::mutex::Mutex::new(event_loop.create_proxy());
        egui_glow
            .egui_ctx
            .set_request_repaint_callback(move |info| {
                event_loop_proxy
                    .lock()
                    .send_event(UserEvent::Redraw(info.delay))
                    .expect("Cannot send event");
            });
        egui_glow
    };

    let ctx = AppContext {
        gl_context: GLContext {
            gl,
            #[cfg(not(target_arch = "wasm32"))]
            gl_surface,
        },
        state: AppState {
            suggested_shader_version: shader_version,
            width,
            height,
            scale_factor,
            start: chrono::Utc::now(),
            last_update_time: chrono::Utc::now(),
            last_render_time: chrono::Utc::now(),
            update_delta_time: 0.0,
            render_delta_time: 0.0,
        },
        #[cfg(not(target_arch = "wasm32"))]
        egui_glow,
    };

    let app = App::new(&ctx).await;

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
            ctx.state.update_delta_time =
                (now - ctx.state.last_update_time).num_milliseconds() as f32 / 1000.0;
            ctx.state.last_update_time = chrono::Utc::now();
            g.game.app.update(ctx.state.update_delta_time);
        },
        move |g| {
            let ctx = &mut g.game.ctx;
            let now = chrono::Utc::now();
            let app = &mut g.game.app;
            ctx.state.render_delta_time =
                (now - ctx.state.last_render_time).num_milliseconds() as f32 / 1000.0;
            ctx.state.last_render_time = chrono::Utc::now();
            #[cfg(not(target_arch = "wasm32"))]
            ctx.egui_glow.run(&g.window, |egui_ctx| {
                app.ui(&ctx.state, egui_ctx);
            });
            app.render(ctx);
            #[cfg(not(target_arch = "wasm32"))]
            {
                ctx.egui_glow.paint(&g.window);
                use game_loop::TimeTrait;
                use glutin::surface::GlSurface;

                g.game
                    .ctx
                    .gl_context
                    .gl_surface
                    .swap_buffers(&gl_context)
                    .unwrap();

                let dt = TIME_STEP.as_secs_f64() - game_loop::Time::now().sub(&g.current_instant());
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
                    #[cfg(not(target_arch = "wasm32"))]
                    ctx.egui_glow.destroy();
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
                return;
            }

            #[cfg(not(target_arch = "wasm32"))]
            if let winit::event::Event::WindowEvent { event, .. } = &event {
                let event_response = ctx.egui_glow.on_window_event(&g.window, event);
                if event_response.repaint {
                    g.window.request_redraw();
                }
            }
        },
    )
    .unwrap();
}
