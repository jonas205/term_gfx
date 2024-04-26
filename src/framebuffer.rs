use std::io;

use crate::Color;

const PIXEL_WIDTH: usize = 2;
const PIXEL: &[u8] = b"  ";

#[derive(Debug)]
pub enum FramebufferError {
    CantGetTerminalSize,
    OutOfBoundsError,
    IoError(io::Error),
}

pub(crate) struct Framebuffer {
    colors: Vec<Vec<Color>>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, color: Color) -> Framebuffer {
        let mut colors: Vec<Vec<Color>> = Vec::with_capacity(height);

        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(color.clone());
            }
            colors.push(row);
        }

        Framebuffer { colors }
    }

    pub fn new_terminal_size(color: Color) -> Result<Framebuffer, FramebufferError> {
        if let Some((w, h)) = term_size::dimensions() {
            Ok(Framebuffer::new(w / PIXEL_WIDTH, h, color))
        } else {
            Err(FramebufferError::CantGetTerminalSize)
        }
    }

    pub fn clear(&mut self, color: Color) {
        for y in 0..self.colors.len() {
            for x in 0..self.colors[y].len() {
                self.colors[y][x] = color.clone();
            }
        }
    }

    pub fn height(&self) -> usize {
        self.colors.len()
    }

    pub fn width(&self) -> usize {
        self.colors[0].len()
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), FramebufferError> {
        if y >= self.colors.len() || x >= self.colors[y].len() {
            Err(FramebufferError::OutOfBoundsError)
        } else {
            self.colors[y][x] = color;
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
        match out.write(format!("\x1b[{}F", self.height()-1).as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(FramebufferError::IoError(e)),
        }

        Ok(())
    }

    pub fn render<R>(&self, out: &mut R) -> Result<(), FramebufferError>
    where
        R: std::io::Write,
    {
        let mut current: Option<&Color> = None;

        let mut i = 0;
        for row in &self.colors {
            for c in row {
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
            }
            current = None;
            match Color::reset(out) {
                Ok(_) => (),
                Err(e) => return Err(FramebufferError::IoError(e)),
            };

            i += 1;
            if i != self.height() {
                match out.write(b"\n") {
                    Ok(_) => (),
                    Err(e) => return Err(FramebufferError::IoError(e)),
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
