use std::{borrow::BorrowMut, process::exit};

use term_gfx::{
    app::{AppError, AppStartupConfig, Scene}, Framebuffer, Renderer
};

fn error_handler(err: AppError) {
    eprintln!("Got an error: {:?}", err);
    exit(-1);
}

struct ExampleScene {
    img_org: Option<Framebuffer>,
    img: Option<Framebuffer>,
}

impl ExampleScene {
    fn new() -> ExampleScene {
        let img = Framebuffer::new_image("res/best_cat_ever.jpeg").unwrap();

        ExampleScene {
            img: None,
            img_org: Some(img)
        }
    }
}

impl Scene for ExampleScene {
    fn attach(&mut self, _app_info: &term_gfx::app::AppInfo) {
        if let Some(i) = self.img_org.take() {
            let (w, h) = _app_info.renderer.as_ref().borrow().screen_size();

            self.img = Some(Framebuffer::new_resized(&i, w as usize, h as usize));
        }
    }

    fn detach(&mut self) {}

    fn update(&mut self, renderer: &mut Renderer) {
        if let Some(i) = &self.img { 
            renderer.draw_framebuffer(0, 0, i);
        }
    }
}

fn main() {
    let cfg = AppStartupConfig { fps: 3 };

    let scene = Box::new(ExampleScene::new());

    term_gfx::run(scene, cfg, error_handler);
}
