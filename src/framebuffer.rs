use std::{
    cmp::{max, min},
    io,
};

use image::{GenericImageView, Pixel};

use crate::{profile, Color};

const PIXEL_WIDTH: usize = 1;
const PIXEL: &[u8] = b" ";

#[derive(Debug)]
pub enum FramebufferError {
    CantGetTerminalSize,
    OutOfBoundsError,
    IoError(io::Error),
    ImageError(image::ImageError),
}

pub struct Framebuffer {
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

    pub fn new_image(path: &str) -> Result<Framebuffer, FramebufferError> {
        profile!();

        let img = match image::open(path) {
            Ok(i) => i,
            Err(e) => return Err(FramebufferError::ImageError(e)),
        };

        let width = img.width() as usize;
        let height = img.height() as usize;

        let mut colors: Vec<Color> = Vec::with_capacity(height * width);

        for j in 0..height {
            for i in 0..width {
                let pixel = img.get_pixel(i as u32, j as u32).to_rgb();

                let color = Color::rgb(pixel.0[0], pixel.0[1], pixel.0[2]);
                colors.push(color);
            }
        }

        Ok(Framebuffer {
            colors,
            width,
            height,
        })
    }

    fn sample_linear(&self, x0: f32, y0: f32, x1: f32, y1: f32) -> Color {
        let samples = 5;

        let mut r: u64 = 0;
        let mut g: u64 = 0;
        let mut b: u64 = 0;

        let dx = (x1 - x0) / samples as f32;
        let dy = (y1 - y0) / samples as f32;

        let mut n = 0;

        for i in 0..=samples {
            for j in 0..=samples {
                let x = (x0 + dx * j as f32) as i64;
                let y = (y0 + dy * i as f32) as i64;

                if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
                    continue;
                }

                let p = self.get_pixel(x, y).unwrap();

                r += p.red as u64;
                g += p.green as u64;
                b += p.blue as u64;

                n += 1;
            }
        }

        Color::rgb((r / n) as u8, (g / n) as u8, (b / n) as u8)
    }

    pub fn new_resized(old: &Framebuffer, width: usize, height: usize) -> Framebuffer {
        profile!();

        let mut colors: Vec<Color> = Vec::with_capacity(height * width);

        for j in 0..height {
            for i in 0..width {
                let (w, ow, h, oh) = (
                    width as f32,
                    old.width as f32,
                    height as f32,
                    old.height as f32,
                );
                let i = i as f32;
                let j = j as f32;

                let x0 = i * ow / w;
                let x1 = (i + 1.0) * ow / w;
                let y0 = j * oh / h;
                let y1 = (j + 1.0) * oh / h;

                let color = old.sample_linear(x0, y0, x1, y1);

                colors.push(color);
            }
        }

        Framebuffer {
            colors,
            width,
            height,
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

    pub fn get_pixel(&self, x: i64, y: i64) -> Result<Color, FramebufferError> {
        if x < 0 || y < 0 {
            return Err(FramebufferError::OutOfBoundsError);
        }

        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();

        if y >= self.height || x >= self.width {
            return Err(FramebufferError::OutOfBoundsError);
        }

        Ok(self.colors[y * self.width + x].clone())
    }

    pub fn pixel(&mut self, x: i64, y: i64, color: Color) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();

        if y >= self.height || x >= self.width {
            return false;
        }

        self.colors[y * self.width + x] = color;
        true
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

    pub(crate) fn hide_cursor<R>(&self, out: &mut R, hide: bool) -> Result<(), FramebufferError>
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

    pub(crate) fn reset_cursor<R>(&self, out: &mut R) -> Result<(), FramebufferError>
    where
        R: std::io::Write,
    {
        match out.write(format!("\x1b[{}F", self.height() - 1).as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(FramebufferError::IoError(e)),
        }

        Ok(())
    }

    pub(crate) fn render<R>(&self, out: &mut R) -> Result<(), FramebufferError>
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

    pub fn draw_framebuffer(&mut self, x: i64, y: i64, fb: &Framebuffer) {
        let (w, h) = (fb.width(), fb.height());

        for j in (max(0, y))..(min(y + h as i64, self.height() as i64)) {
            for i in (max(0, x))..(min(x + w as i64, self.width() as i64)) {
                if let Ok(c) = fb.get_pixel(i - x, j - y) {
                    self.pixel(i, j, c);
                };
            }
        }
    }
}
