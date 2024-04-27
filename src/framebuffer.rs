use std::io;

use crate::{profile, Color};

const PIXEL_WIDTH: usize = 1;
const PIXEL: &[u8] = b" ";

#[derive(Debug)]
pub enum FramebufferError {
    CantGetTerminalSize,
    OutOfBoundsError,
    IoError(io::Error),
}

pub(crate) struct Framebuffer {
    colors: Vec<Color>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, color: Color) -> Framebuffer {
        profile!();
        let mut colors: Vec<Color> = Vec::with_capacity(height * width);

        for _ in 0..(height * width) {
            colors.push(color.clone());
        }

        Framebuffer {
            colors,
            width,
            height,
        }
    }

    pub fn new_terminal_size(color: Color) -> Result<Framebuffer, FramebufferError> {
        profile!();
        if let Some((w, h)) = term_size::dimensions() {
            Ok(Framebuffer::new(w / PIXEL_WIDTH, h, color))
        } else {
            Err(FramebufferError::CantGetTerminalSize)
        }
    }

    pub fn clear(&mut self, color: Color) {
        profile!();
        for i in 0..self.colors.len() {
            self.colors[i] = color.clone();
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), FramebufferError> {
        if y >= self.height || x >= self.width {
            Err(FramebufferError::OutOfBoundsError)
        } else {
            self.colors[y * self.width + x] = color;
            Ok(())
        }
    }

    pub fn hide_cursor<R>(&self, out: &mut R, hide: bool) -> Result<(), FramebufferError>
    where
        R: std::io::Write,
    {
        match if hide {
            out.write(b"\x1b[?25l")
        } else {
            out.write(b"\x1b[?25h")
        } {
            Ok(_) => (),
            Err(e) => return Err(FramebufferError::IoError(e)),
        };

        Ok(())
    }

    pub fn reset_cursor<R>(&self, out: &mut R) -> Result<(), FramebufferError>
    where
        R: std::io::Write,
    {
        match out.write(format!("\x1b[{}F", self.height() - 1).as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(FramebufferError::IoError(e)),
        }

        Ok(())
    }

    pub fn render<R>(&self, out: &mut R) -> Result<(), FramebufferError>
    where
        R: std::io::Write,
    {
        profile!();
        let mut current: Option<&Color> = None;

        let mut x = 0;
        let mut y = 0;
        for c in &self.colors {
            if current == None || current.unwrap() != c {
                current = Some(c);
                match c.apply(out) {
                    Ok(_) => (),
                    Err(e) => return Err(FramebufferError::IoError(e)),
                }
            }
            match out.write(PIXEL) {
                Ok(_) => (),
                Err(e) => return Err(FramebufferError::IoError(e)),
            }

            x += 1;

            if x == self.width {
                y += 1;

                current = None;
                match Color::reset(out) {
                    Ok(_) => (),
                    Err(e) => return Err(FramebufferError::IoError(e)),
                };
                if y != self.height() {
                    match out.write(b"\n") {
                        Ok(_) => (),
                        Err(e) => return Err(FramebufferError::IoError(e)),
                    }
                }
            }
        }

        match out.flush() {
            Ok(_) => (),
            Err(e) => return Err(FramebufferError::IoError(e)),
        }

        Ok(())
    }
}
