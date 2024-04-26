use std::io::{stdout, Stdout};

use crate::{
    framebuffer::{Framebuffer, FramebufferError},
    Color,
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

    pub fn screen_size(&self) -> (i64, i64) {
        (self.fb.width() as i64, self.fb.height() as i64)
    }

    pub fn pixel(&mut self, x: i64, y: i64, color: Color) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        match self.fb.set_pixel(x as usize, y as usize, color) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn line(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, color: Color) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let mut x = x0;
        let mut y = y0;

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut e2;

        loop {
            self.pixel(x, y, color.clone());
            if x == x1 && y == y1 {
                break;
            }
            e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }
}
