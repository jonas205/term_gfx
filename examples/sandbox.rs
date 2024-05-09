use std::process::exit;

use term_gfx::{
    app::{AppError, AppStartupConfig, Scene},
    event::Event,
    Color, Framebuffer, Renderer,
};

fn error_handler(err: AppError) {
    eprintln!("Got an error: {:?}", err);
    exit(-1);
}

struct ExampleScene {
    img: Framebuffer,
    img_sml: Framebuffer,
    img_small_size: (usize, usize),
}

impl ExampleScene {
    fn new() -> ExampleScene {
        let img = Framebuffer::new_image("res/test_image.png").unwrap();
        let img_small_size = (img.width() - 5, img.height() - 5);

        let img_sml = Framebuffer::new_resized(&img, img_small_size.0, img_small_size.1);

        ExampleScene {
            img,
            img_sml,
            img_small_size,
        }
    }

    fn draw_border(&mut self, renderer: &mut Renderer) {
        let (w, h) = renderer.screen_size();

        for i in 0..w {
            renderer.pixel(i, 0, Color::rgb(255, 0, 0));
            renderer.pixel(i, h - 1, Color::rgb(0, 255, 0));
        }

        for i in 0..h {
            renderer.pixel(0, i, Color::rgb(0, 0, 255));
            renderer.pixel(w - 1, i, Color::rgb(255, 255, 0));
        }
    }

    fn draw_triangle(&mut self, renderer: &mut Renderer) {
        let (w, h) = renderer.screen_size();

        let (x0, y0) = (w / 3, h / 4);
        let (x1, y1) = (w / 3 * 2, h / 2);
        let (x2, y2) = (w / 7 * 2, h / 3 * 2);

        let line_with_dot = |r: &mut Renderer, x0: i64, y0: i64, x1: i64, y1: i64| {
            r.line(x0, y0, x1, y1, Color::rgb(0, 255, 255));
            r.pixel(x0, y0, Color::grey(255));
            r.pixel(x1, y1, Color::grey(255));
        };

        line_with_dot(renderer, x0, y0, x1, y1);
        line_with_dot(renderer, x1, y1, x2, y2);
        line_with_dot(renderer, x0, y0, x2, y2);
    }
}

impl Scene for ExampleScene {
    fn attach(&mut self, _app_info: &term_gfx::app::AppInfo) {}

    fn detach(&mut self) {}

    fn update(&mut self, renderer: &mut Renderer) {
        self.draw_border(renderer);
        self.draw_triangle(renderer);

        renderer.line(10, 10, 30, 10, Color::rgb(255, 0, 255));
        renderer.draw_framebuffer(5, 5, &self.img);
        renderer.draw_framebuffer(5 + self.img.width() as i64 + 1, 5, &self.img_sml);
    }

    fn event(&mut self, event: &Event) {
        if let Event::CharEvent(c) = event {
            match c {
                'w' => self.img_small_size.1 -= 1,
                'a' => self.img_small_size.0 -= 1,
                's' => self.img_small_size.1 += 1,
                'd' => self.img_small_size.0 += 1,
                _ => return,
            }

            if self.img_small_size.1 == 0 {
                self.img_small_size.1 = 1;
            }
            if self.img_small_size.0 == 0 {
                self.img_small_size.0 = 1;
            }

            self.img_sml =
                Framebuffer::new_resized(&self.img, self.img_small_size.0, self.img_small_size.1);
        }
    }
}

fn main() {
    let cfg = AppStartupConfig { fps: 60 };

    let scene = Box::new(ExampleScene::new());

    term_gfx::run(scene, cfg, error_handler);
}
