use color_eyre::eyre::eyre;
use glutin::config::{ColorBufferType, Config, ConfigTemplateBuilder, GetGlConfig, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version,
};
use glutin::display::GetGlDisplay;
use glutin::prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext};
use glutin::surface::{GlSurface, Surface, SwapInterval, WindowSurface};
use glutin_winit::DisplayBuilder;
use glutin_winit::GlWindow;
use skia_safe::gpu::direct_contexts::make_gl;
use skia_safe::gpu::ganesh::gl::backend_render_targets;
use skia_safe::gpu::gl::Format;
use skia_safe::gpu::{self, DirectContext, gl};
use skia_safe::{Color, Color4f, ColorType, Paint, Rect};
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::{REQUEST_REDRAW};
impl ApplicationHandler for WinitApp {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let (window, gl_config) = match DisplayBuilder::new()
            .with_window_attributes(Some(self.window_options.clone()))
            .build(event_loop, self.template.clone(), gl_config_picker)
        {
            Ok((window, gl_config)) => (window.unwrap(), gl_config),
            Err(err) => {
                self.exit_state = Err(eyre!("{:#?}", err));
                event_loop.exit();
                return;
            }
        };
        log::trace!("Picked a config with {} samples", gl_config.num_samples());
        self.post_opengl_init(window, gl_config);
    }
    fn resumed(&mut self, event_loop: &dyn ActiveEventLoop) {
        log::trace!("Recreating window in `resumed`");
        // Pick the config which we already use for the context.
        let gl_config = self.gl_context.as_ref().unwrap().config();
        let window = match glutin_winit::finalize_window(
            event_loop,
            self.window_options.clone(),
            &gl_config,
        ) {
            Ok(window) => window,
            Err(err) => {
                self.exit_state = Err(err.into());
                event_loop.exit();
                return;
            }
        };

        self.post_opengl_init(window, gl_config);
    }

    fn suspended(&mut self, _event_loop: &dyn ActiveEventLoop) {
        log::trace!("Android window removed");
        self.window = None;

        // Make context not current.
        self.gl_context = Some(
            self.gl_context
                .take()
                .unwrap()
                .make_not_current()
                .unwrap()
                .treat_as_possibly_current(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::SurfaceResized(size) if size.width != 0 && size.height != 0 => {
                let Some(SurfaceAndWindow {
                    gl_surface,
                    window,
                    mut skia_context,
                    mut skia_surface,
                }) = self.window.take()
                else {
                    return;
                };

                let gl_context = self.gl_context.take().unwrap();
                let skia_surface = self.make_skia_surface(
                    &gl_surface,
                    &gl_context.config(),
                    &mut skia_context,
                    size.width,
                    size.height,
                );
                gl_surface.resize(
                    &gl_context,
                    NonZeroU32::new(size.width).unwrap(),
                    NonZeroU32::new(size.height).unwrap(),
                );
                self.gl_context = gl_context.into();
                self.window = SurfaceAndWindow {
                    gl_surface,
                    skia_surface,
                    skia_context,
                    window,
                }
                .into();
            }
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let Some(SurfaceAndWindow {
                    skia_surface,
                    skia_context,
                    gl_surface,
                    ..
                }) = self.window.as_mut()
                else {
                    return;
                };
                skia_surface
                    .canvas()
                    .clear(Color4f::new(1.0, 1.0, 1.0, 1.0))
                    .draw_rect(
                        Rect::from_wh(100., 100.),
                        Paint::default().set_color(Color::BLACK),
                    );

                skia_context.flush_and_submit();
                gl_surface
                    .swap_buffers(self.gl_context.as_ref().unwrap())
                    .unwrap();

                log::debug!("Render");
            }
            _ => {
                let Some(SurfaceAndWindow { window, .. }) = self.window.as_mut() else {
                    return;
                };
                window.request_redraw();
            }
        }
    }

    fn destroy_surfaces(&mut self, _event_loop: &dyn ActiveEventLoop) {
        let _gl_display = self.gl_context.take().unwrap().display();

        self.window = None;
        if let glutin::display::Display::Egl(display) = _gl_display {
            unsafe {
                display.terminate();
            }
        }
    }
}

fn create_gl_context(window: &dyn Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    let gl_display = gl_config.display();

    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    }
}

pub(super) struct WinitApp {
    template: ConfigTemplateBuilder,
    gl_context: Option<PossiblyCurrentContext>,
    exit_state: color_eyre::Result<()>,
    window_options: WindowAttributes,
    window: Option<SurfaceAndWindow>,
}

impl WinitApp {
    pub(super) fn new(options: impl Into<WindowAttributes>) -> Self {
        let options = options.into();
        Self {
            template: ConfigTemplateBuilder::new()
                .with_alpha_size(8)
                .with_transparency(true),
            window_options: options.clone(),
            exit_state: Ok(()),
            gl_context: None,
            window: None,
        }
    }
    fn post_opengl_init(&mut self, window: Box<dyn Window>, gl_config: Config) {
        // Create gl context.
        self.gl_context =
            Some(create_gl_context(window.as_ref(), &gl_config).treat_as_possibly_current());

        let attrs = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        // The context needs to be current for the Renderer to set up shaders and
        // buffers. It also performs function loading, which needs a current context on
        // WGL.
        let gl_context = self.gl_context.as_ref().unwrap();
        gl_context.make_current(&gl_surface).unwrap();
        // Try setting vsync.
        if let Err(res) = gl_surface
            .set_swap_interval(gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            log::error!("Error setting vsync: {res:?}");
        }
        let window: Rc<dyn Window> = window.into();
        REQUEST_REDRAW.set({
            let window = Rc::downgrade(&window);
            Box::new(move || {
                let Some(window) = window.upgrade() else {
                    return;
                };
                window.request_redraw();
            })
        });
        let (skia_surface, skia_context) = self.initialize_skia(&gl_config, &gl_surface);
        self.window = Some(SurfaceAndWindow {
            gl_surface,
            window,
            skia_surface,
            skia_context,
        });
    }
    pub(super) fn initialize_skia(
        &mut self,
        gl_config: &Config,
        gl_surface: &Surface<WindowSurface>,
    ) -> (skia_safe::Surface, skia_safe::gpu::DirectContext) {
        // Interface GL automática (sem crate gl)
        let interface = gpu::gl::Interface::new_load_with_cstr(|name| {
            if name == c"eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_surface.display().get_proc_address(name)
        })
        .expect("Failed to create Skia GL interface");

        // Contexto GPU ligado ao OpenGL ativo
        let mut gr_context = make_gl(interface, None).expect("Failed to create Skia DirectContext");

        return (
            self.make_skia_surface(gl_surface, gl_config, &mut gr_context, 0, 0),
            gr_context,
        );
    }
    fn make_skia_surface(
        &self,
        gl_surface: &Surface<WindowSurface>,
        gl_config: &Config,
        gr_context: &mut DirectContext,
        width: u32,
        height: u32,
    ) -> skia_safe::Surface {
        // Pega tamanho da janela
        let width = if width != 0 {
            width
        } else {
            gl_surface.width().unwrap()
        };
        let height = if height != 0 {
            height
        } else {
            gl_surface.height().unwrap()
        };
        type GlGetIntegerv = unsafe extern "system" fn(pname: u32, data: *mut i32);
        const GL_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
        let gl_get_integerv: GlGetIntegerv =
            unsafe { std::mem::transmute(gl_surface.display().get_proc_address(c"glGetIntegerv")) };
        let mut fboid: i32 = 0;
        unsafe {
            gl_get_integerv(GL_FRAMEBUFFER_BINDING, &mut fboid);
        }
        let (format, color_type) =
            color_buffer_to_skia(gl_config.color_buffer_type().expect("fuck you"));
        let fb_info = gpu::gl::FramebufferInfo {
            fboid: fboid as _, // default framebuffer
            format: format.into(),
            protected: gpu::Protected::No,
        };

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;
        let backend_render_target = backend_render_targets::make_gl(
            (width as _, height as _),
            num_samples,  // samples
            stencil_size, // stencil bits
            fb_info,
        );

        gpu::surfaces::wrap_backend_render_target(
            &mut *gr_context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft,
            color_type,
            None,
            None,
        )
        .expect("Failed to create Skia surface")
    }
    pub(super) fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self).unwrap();
        self.exit_state.unwrap();
    }
}

struct SurfaceAndWindow {
    skia_surface: skia_safe::Surface,
    skia_context: skia_safe::gpu::DirectContext,
    gl_surface: Surface<WindowSurface>,
    // NOTE: Window should be dropped after all resources created using its
    // raw-window-handle.
    window: Rc<dyn Window>,
}

fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() < accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

fn color_buffer_to_skia(color_buffer: ColorBufferType) -> (Format, ColorType) {
    match color_buffer {
        ColorBufferType::Rgb {
            r_size,
            g_size,
            b_size,
        } => match (r_size, g_size, b_size) {
            (8, 8, 8) => (Format::RGBA8, ColorType::RGBA8888),
            (10, 10, 10) => (Format::RGB10_A2, ColorType::RGBA1010102),
            (5, 6, 5) => (Format::RGB565, ColorType::RGB565),
            _ => (Format::Unknown, ColorType::Unknown),
        },
        ColorBufferType::Luminance(size) => match size {
            8 => (Format::R8, ColorType::Gray8),
            _ => (Format::Unknown, ColorType::Unknown),
        },
    }
}
