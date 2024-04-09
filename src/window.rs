use glow::{Context, HasContext};
use std::collections::HashMap;
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

#[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
#[derive(Debug)]
pub enum UserEvent {
    Redraw(Duration),
}

pub trait Application: Sized {
    async unsafe fn new(_ctx: &AppContext) -> Self;
    #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
    fn ui(&mut self, state: &AppState, _gl_ctx: &GLContext, egui_ctx: &egui::Context) {
        // show fps by default
        egui::Window::new("Info").show(egui_ctx, |ui| {
            ui.label(format!("FPS: {:.1}", 1.0 / state.render_delta_time));
        });
    }
    #[cfg(all(not(target_arch = "wasm32"), feature = "imgui-support"))]
    fn ui(&mut self, _ui: &easy_imgui_window::easy_imgui::Ui<EasyImGuiFacade<Self>>) {}
    unsafe fn render(&mut self, _ctx: &AppContext) {}
    unsafe fn update(&mut self, _update_delta_time: f32) {}
    unsafe fn resize(&mut self, ctx: &AppContext, width: u32, height: u32) {
        let gl = ctx.gl();
        gl.viewport(0, 0, width as i32, height as i32);
    }
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
    #[builder(default = 1)]
    pub num_samples: u8,
}

pub struct AppContext {
    #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
    pub egui_glow: egui_glow::EguiGlow,
    #[cfg(feature = "imgui-support")]
    pub imgui_renderer: easy_imgui_window::easy_imgui_renderer::Renderer,
    #[cfg(feature = "imgui-support")]
    pub imgui_status: easy_imgui_window::MainWindowStatus,
    pub gl_context: GLContext,
    pub app_state: AppState,
    pub gl_state: GlState,
}

pub struct GLContext {
    #[cfg(feature = "imgui-support")]
    pub gl: std::rc::Rc<glow::Context>,
    #[cfg(not(feature = "imgui-support"))]
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

#[derive(Debug, Default)]
pub struct GlState {
    pub states: HashMap<u32, bool>,
}

#[allow(dead_code)]
impl AppContext {
    pub fn gl(&self) -> &glow::Context {
        &self.gl_context.gl
    }

    pub fn height(&self) -> u32 {
        self.app_state.height
    }

    pub fn width(&self) -> u32 {
        self.app_state.width
    }

    pub fn scale_factor(&self) -> f64 {
        self.app_state.scale_factor
    }
    pub fn update_delta_time(&self) -> f32 {
        self.app_state.update_delta_time
    }

    pub fn render_delta_time(&self) -> f32 {
        self.app_state.render_delta_time
    }

    pub fn last_update_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.app_state.last_update_time
    }

    pub fn last_render_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.app_state.last_render_time
    }

    pub fn elapsed_time(&self) -> chrono::Duration {
        self.app_state.elapsed_time()
    }

    pub fn elapsed_time_secs(&self) -> f32 {
        self.elapsed_time().num_milliseconds() as f32 / 1000.0
    }

    pub fn suggested_shader_version(&self) -> &'static str {
        self.app_state.suggested_shader_version
    }
}

#[allow(dead_code)]
impl AppState {
    pub fn start(&self) -> chrono::DateTime<chrono::Utc> {
        self.start
    }

    pub fn update_delta_time(&self) -> f32 {
        self.update_delta_time
    }

    pub fn render_delta_time(&self) -> f32 {
        self.render_delta_time
    }

    pub fn last_update_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.last_update_time
    }

    pub fn last_render_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.last_render_time
    }

    pub fn elapsed_time(&self) -> chrono::Duration {
        chrono::Utc::now() - self.start
    }

    pub fn elapsed_time_secs(&self) -> f32 {
        self.elapsed_time().num_milliseconds() as f32 / 1000.0
    }

    pub fn suggested_shader_version(&self) -> &'static str {
        self.suggested_shader_version
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
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

        #[cfg(feature = "egui-support")]
        let event_loop = winit::event_loop::EventLoopBuilder::<UserEvent>::with_user_event()
            .build()
            .unwrap();
        #[cfg(not(feature = "egui-support"))]
        let event_loop = winit::event_loop::EventLoop::new().unwrap();

        let window_builder = winit::window::WindowBuilder::new()
            .with_title(title.as_str())
            .with_inner_size(winit::dpi::LogicalSize::new(width, height));

        let template = ConfigTemplateBuilder::new().with_multisampling(init_info.num_samples);

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
        #[allow(unused_mut)]
        let mut context_attributes = ContextAttributesBuilder::new().with_context_api(
            ContextApi::OpenGl(Some(glutin::context::Version { major, minor })),
        );
        #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
        {
            context_attributes = context_attributes.with_debug(true);
        }
        let context_attributes = context_attributes.build(raw_window_handle);

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

        #[allow(unused_mut)]
        let mut gl =
            unsafe { glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s)) };

        #[cfg(debug_assertions)]
        {
            set_debug_callback(&mut gl);
        }

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
    #[cfg(feature = "imgui-support")]
    #[allow(clippy::arc_with_non_send_sync)]
    let gl = std::rc::Rc::new(gl);

    #[cfg(not(feature = "imgui-support"))]
    let gl = Arc::new(gl);

    #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
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
    #[cfg(feature = "imgui-support")]
    let imgui_renderer = {
        let mut r = easy_imgui_window::easy_imgui_renderer::Renderer::new(gl.clone()).unwrap();
        r.set_background_color(None);
        r
    };
    let ctx = AppContext {
        #[cfg(feature = "imgui-support")]
        imgui_renderer,
        #[cfg(feature = "imgui-support")]
        imgui_status: easy_imgui_window::MainWindowStatus::default(),
        gl_context: GLContext {
            gl,
            #[cfg(not(target_arch = "wasm32"))]
            gl_surface,
        },
        app_state: AppState {
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
        #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
        egui_glow,
        gl_state: GlState::default(),
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
            ctx.app_state.update_delta_time =
                (now - ctx.app_state.last_update_time).num_milliseconds() as f32 / 1000.0;
            ctx.app_state.last_update_time = chrono::Utc::now();
            g.game.app.update(ctx.app_state.update_delta_time);
        },
        move |g| {
            let ctx = &mut g.game.ctx;
            let now = chrono::Utc::now();
            let app = &mut g.game.app;
            ctx.app_state.render_delta_time =
                (now - ctx.app_state.last_render_time).num_milliseconds() as f32 / 1000.0;
            ctx.app_state.last_render_time = chrono::Utc::now();
            #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
            ctx.egui_glow.run(&g.window, |egui_ctx| {
                app.ui(&ctx.app_state, &ctx.gl_context, egui_ctx);
            });

            let gl = &ctx.gl_context.gl;
            restore_gl_states(gl, &ctx.gl_state.states);
            app.render(ctx);

            // we have debug callback already
            // #[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
            // {
            //     let mut error_code = gl.get_error();
            //     while error_code != glow::NO_ERROR {
            //         let prefix = match error_code {
            //             glow::INVALID_ENUM => "INVALID_ENUM",
            //             glow::INVALID_VALUE => "INVALID_VALUE",
            //             glow::INVALID_OPERATION => "INVALID_OPERATION",
            //             glow::STACK_OVERFLOW => "STACK_OVERFLOW",
            //             glow::STACK_UNDERFLOW => "STACK_UNDERFLOW",
            //             glow::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            //             glow::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            //             glow::CONTEXT_LOST => "CONTEXT_LOST",
            //             _ => "UNKNOWN_ERROR",
            //         };
            //         log::error!("OpenGL Error ({}): {:?}", prefix, error_code);
            //         error_code = gl.get_error();
            //     }
            // }

            #[cfg(feature = "imgui-support")]
            ctx.imgui_renderer.do_frame(&mut EasyImGuiFacade(app));

            #[cfg(not(target_arch = "wasm32"))]
            {
                // https://github.com/emilk/egui/issues/93#issuecomment-907745330
                // egui will not recover gl state changes, like gl.enable(DEPTH_TEST)
                record_gl_states(gl, &mut ctx.gl_state.states);

                #[cfg(feature = "egui-support")]
                ctx.egui_glow.paint(&g.window);

                use game_loop::TimeTrait;
                use glutin::surface::GlSurface;

                ctx.gl_context.gl_surface.swap_buffers(&gl_context).unwrap();

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

            #[cfg(feature = "imgui-support")]
            {
                let ui_wants = easy_imgui_window::do_event(
                    &mut &*g.window,
                    &mut ctx.imgui_renderer,
                    &mut ctx.imgui_status,
                    &mut EasyImGuiFacade(app),
                    event,
                    easy_imgui_window::EventFlags::DoNotRender,
                );
                if ui_wants.want_capture_keyboard || ui_wants.want_capture_mouse {
                    //TODO: separate mouse/keyboard capture
                    return;
                }
            }

            if input.update(event) {
                if input.key_pressed(winit::keyboard::KeyCode::Escape)
                    || input.close_requested()
                    || input.destroyed()
                {
                    log::info!("Exiting");
                    app.exit(ctx);
                    #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
                    ctx.egui_glow.destroy();
                    g.exit();
                    return;
                }
                if let Some(size) = input.window_resized() {
                    let (width, height) = (size.width, size.height);
                    log::info!("Resizing to {}x{}", width, height);
                    ctx.app_state.width = width;
                    ctx.app_state.height = height;
                    app.resize(ctx, width, height);
                    return;
                }

                app.process_input(ctx, input);
                #[allow(clippy::needless_return)]
                return;
            }

            #[cfg(all(not(target_arch = "wasm32"), feature = "egui-support"))]
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

#[cfg(feature = "imgui-support")]
pub struct EasyImGuiFacade<'a, A>(&'a mut A);

#[cfg(feature = "imgui-support")]
impl<'a, A: Application> easy_imgui_window::easy_imgui::UiBuilder for EasyImGuiFacade<'a, A> {
    fn do_ui(&mut self, ui: &easy_imgui_window::easy_imgui::Ui<Self>) {
        self.0.ui(ui);
    }
}

#[allow(dead_code)]
unsafe fn set_debug_callback(gl: &mut Context) {
    gl.debug_message_callback(|source, gltype, id, severity, message| {
        let source = match source {
            glow::DEBUG_SOURCE_API => "API",
            glow::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
            glow::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
            glow::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
            glow::DEBUG_SOURCE_APPLICATION => "Application",
            glow::DEBUG_SOURCE_OTHER => "Other",
            _ => "Unknown",
        };
        let gltype = match gltype {
            glow::DEBUG_TYPE_ERROR => "Error",
            glow::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
            glow::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
            glow::DEBUG_TYPE_PORTABILITY => "Portability",
            glow::DEBUG_TYPE_PERFORMANCE => "Performance",
            glow::DEBUG_TYPE_OTHER => "Other",
            glow::DEBUG_TYPE_MARKER => "Marker",
            glow::DEBUG_TYPE_PUSH_GROUP => "Push Group",
            glow::DEBUG_TYPE_POP_GROUP => "Pop Group",
            _ => "Unknown",
        };

        // match severity use different log level
        match severity {
            glow::DEBUG_SEVERITY_HIGH => log::error!(
                "OpenGL Debug High Severity: {} {} {} {}",
                source,
                gltype,
                id,
                message
            ),
            glow::DEBUG_SEVERITY_MEDIUM => log::warn!(
                "OpenGL Debug Medium Severity: {} {} {} {}",
                source,
                gltype,
                id,
                message
            ),
            glow::DEBUG_SEVERITY_LOW => log::debug!(
                "OpenGL Debug Low Severity: {} {} {} {}",
                source,
                gltype,
                id,
                message
            ),
            glow::DEBUG_SEVERITY_NOTIFICATION => log::trace!(
                "OpenGL Debug Notification: {} {} {} {}",
                source,
                gltype,
                id,
                message
            ),
            _ => log::warn!(
                "OpenGL Debug Unknown Severity: {} {} {} {}",
                source,
                gltype,
                id,
                message
            ),
        }
    });
}

#[allow(dead_code)]
fn record_gl_states(gl: &glow::Context, states: &mut HashMap<u32, bool>) {
    unsafe {
        states.insert(glow::BLEND, gl.is_enabled(glow::BLEND));
        states.insert(glow::CULL_FACE, gl.is_enabled(glow::CULL_FACE));
        states.insert(glow::DEPTH_TEST, gl.is_enabled(glow::DEPTH_TEST));
        states.insert(glow::DITHER, gl.is_enabled(glow::DITHER));
        states.insert(
            glow::POLYGON_OFFSET_FILL,
            gl.is_enabled(glow::POLYGON_OFFSET_FILL),
        );
        states.insert(
            glow::SAMPLE_ALPHA_TO_COVERAGE,
            gl.is_enabled(glow::SAMPLE_ALPHA_TO_COVERAGE),
        );
        states.insert(glow::SAMPLE_COVERAGE, gl.is_enabled(glow::SAMPLE_COVERAGE));
        states.insert(glow::SCISSOR_TEST, gl.is_enabled(glow::SCISSOR_TEST));
        states.insert(glow::STENCIL_TEST, gl.is_enabled(glow::STENCIL_TEST));
        states.insert(
            glow::FRAMEBUFFER_SRGB,
            gl.is_enabled(glow::FRAMEBUFFER_SRGB),
        );
    }
}

fn restore_gl_states(gl: &glow::Context, states: &HashMap<u32, bool>) {
    unsafe {
        for (state, enabled) in states {
            if *enabled {
                gl.enable(*state);
            } else {
                gl.disable(*state);
            }
        }
    }
}
