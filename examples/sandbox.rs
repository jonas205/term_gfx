use std::process::exit;

use term_gfx::{
    app::{AppError, AppStartupConfig, Scene},
    Color, Renderer,
};

fn error_handler(err: AppError) {
    eprintln!("Got an error: {:?}", err);
    exit(-1);
}

struct ExampleScene;

impl Scene for ExampleScene {
    fn attach(&mut self, _app_info: &term_gfx::app::AppInfo) {}

    fn detach(&mut self) {}

    fn update(&mut self, renderer: &mut Renderer) {
        let (w, h) = renderer.screen_size();

        for i in 0..w {
            renderer.pixel(i, 0, Color::rgb(255, 0, 0));
            renderer.pixel(i, h - 1, Color::rgb(0, 255, 0));
        }

        for i in 0..h {
            renderer.pixel(0, i, Color::rgb(0, 0, 255));
            renderer.pixel(w - 1, i, Color::rgb(255, 255, 0));
        }

        let (x0, y0) = (w / 3, h / 4);
        let (x1, y1) = (w / 3 * 2, h / 2);
        let (x2, y2) = (w / 7 * 2, h / 3 * 2);

        let line = |r: &mut Renderer, x0: i64, y0: i64, x1: i64, y1: i64| {
            r.line(x0, y0, x1, y1, Color::rgb(0, 255, 255));
            r.pixel(x0, y0, Color::grey(255));
            r.pixel(x1, y1, Color::grey(255));
        };

        line(renderer, x0, y0, x1, y1);
        line(renderer, x1, y1, x2, y2);
        line(renderer, x0, y0, x2, y2);

        renderer.line(10, 10, 30, 10, Color::rgb(255, 0, 255));
    }
}

fn main() {
    let cfg = AppStartupConfig { fps: 3 };

    let scene = Box::new(ExampleScene);

    term_gfx::run(scene, cfg, error_handler);
}
