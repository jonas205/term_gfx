use std::process::exit;

use term_gfx::{app::{AppError, AppStartupConfig, Scene}, Color, Renderer};

fn error_handler(err: AppError) {
    eprintln!("Got an error: {:?}", err);
    exit(-1);
}

struct ExampleScene;

impl Scene for ExampleScene {
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

        renderer.line(w / 3, h / 4, w / 3 * 2, h / 2, Color::rgb(0, 255, 255));
        renderer.line(w / 3, h / 4, w / 7 * 2, h / 3 * 2, Color::rgb(0, 255, 255));
        renderer.pixel(w / 3, h / 4, Color::grey(255));
        renderer.pixel(w / 3 * 2, h / 2, Color::grey(255));
        renderer.pixel(w / 7 * 2, h / 3 * 2, Color::grey(255));

        renderer.line(10, 10, 30, 10, Color::rgb(255, 0, 255));
    }
}

fn main() {
    let cfg = AppStartupConfig {
        fps: 3,
    };

    let scene = Box::new(ExampleScene);

    term_gfx::run(scene, cfg, error_handler);
}
