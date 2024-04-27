
#[cfg(feature = "profiling")]
#[macro_use]
extern crate lazy_static;

pub mod color;
pub mod framebuffer;
pub mod app;
pub mod renderer;
pub mod profiler;

pub use color::Color;
pub use app::run;
pub use renderer::Renderer;



