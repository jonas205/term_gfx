
#[cfg(feature = "profiling")]
#[macro_use]
extern crate lazy_static;

pub mod color;
pub mod framebuffer;
pub mod app;
pub mod renderer;
pub mod profiler;
pub mod event;

pub use color::Color;
pub use app::run;
pub use renderer::Renderer;
pub use framebuffer::Framebuffer;

extern "C" {
    pub(crate) fn term_disable_stdio_buffer();
    pub(crate) fn term_reenable_stdio_buffer();
    pub(crate) fn term_read_char() -> u8;
}


