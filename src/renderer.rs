use std::{io::{stdout, Stdout}, process::exit};

use crate::{framebuffer::{Framebuffer, FramebufferError}, Color};

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

        Ok(Renderer {
            out,
            fb,
        })
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
        if y0 > y1 {
            return self.line(x1, y1, x0, y0, color);
        } else if y0 == y1 {
            let start = if x0 < x1 { x0 } else { x1 };
            let end = if x0 < x1 { x1 } else { x0 };

            for i in start..=end {
                self.pixel(i, y0, color.clone());
            }

            return;
        }

        let m = (x1 - x0) as f32 / (y1 - y0) as f32; 

        for i in 0..=(y1 - y0) {
            for j in 0..=(m as i64) {
                let x = x0 + (i as f32 * m) as i64 + j;
                self.pixel(x, y0 + i, color.clone());
            }
        }

        self.pixel(x0, y0, color.clone());
        self.pixel(x1, y1, color);
    }
}
