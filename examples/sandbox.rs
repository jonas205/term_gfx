use std::{io::stdout, thread::sleep, time::Duration};

use term_gfx::{framebuffer::FramebufferError, Color, Framebuffer};

fn main() -> Result<(), FramebufferError> {
    let mut fb = Framebuffer::new_terminal_size(Color::rgb(0, 0, 0))?;

    let w = fb.width();
    let h = fb.height();


    let mut x = 1;
    let out = &mut stdout();

    loop {
        fb.clear(Color::grey(127));

        for i in 0..w {
            fb.set_pixel(i, 0, Color::rgb(255, 0, 0))?;
            fb.set_pixel(i, h - 1, Color::rgb(0, 255, 0))?;
        }

        for i in 0..h {
            fb.set_pixel(0, i, Color::rgb(0, 0, 255))?;
            fb.set_pixel(w - 1, i, Color::rgb(255, 255, 0))?;
        }

        fb.set_pixel(0 + x, 3, Color::rgb(255, 0, 0))?;

        x += 1;
        if x > w - 2 {
            x = 1;
        }

        fb.draw(out)?;
        fb.reset_cursor(out)?;

        sleep(Duration::from_millis(1000 / 60));
    }

    // Ok(())
}
