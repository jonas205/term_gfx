use std::io::{stdout, Stdout};

use crate::{
    framebuffer::{Framebuffer, FramebufferError}, profile, Color
};

#[derive(Debug)]
pub enum RendererError {
    FBError(FramebufferError),
}

pub struct Renderer {
    out: Stdout,
    fb: Framebuffer,
}

impl Renderer {
    pub(crate) fn new() -> Result<Renderer, RendererError> {
        profile!();
        let fb = match Framebuffer::new_terminal_size(Color::grey(0)) {
            Ok(fb) => fb,
            Err(e) => return Err(RendererError::FBError(e)),
        };

        let mut out = stdout();

        match fb.hide_cursor(&mut out, true) {
            Ok(_) => (),
            Err(e) => return Err(RendererError::FBError(e)),
        }

        Ok(Renderer { out, fb })
    }

    pub(crate) fn render(&mut self) -> Result<(), RendererError> {
        profile!();
        match self.fb.render(&mut self.out) {
            Ok(_) => (),
            Err(e) => return Err(RendererError::FBError(e)),
        };
        match self.fb.reset_cursor(&mut self.out) {
            Ok(_) => (),
            Err(e) => return Err(RendererError::FBError(e)),
        };
        self.fb.clear(Color::grey(0));

        Ok(())
    }

    pub(crate) fn resize(&mut self, w: i64, h: i64) {
        self.fb = Framebuffer::new(w as usize, h as usize, Color::black());
    }

    pub fn screen_size(&self) -> (i64, i64) {
        (self.fb.width() as i64, self.fb.height() as i64)
    }

    pub fn pixel(&mut self, x: i64, y: i64, color: Color) -> bool {
        self.fb.pixel(x, y, color)
     }

    pub fn line(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, color: Color) {
        self.fb.line(x0, y0, x1, y1, color);
    }

    pub fn draw_framebuffer(&mut self, x: i64, y: i64, fb: &Framebuffer) {
        self.fb.draw_framebuffer(x, y, fb);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.fb.hide_cursor(&mut self.out, false).unwrap();
    }
}
