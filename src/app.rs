use std::{thread::sleep, time::{Duration, Instant}};

use crate::{framebuffer::FramebufferError, renderer, Renderer};

pub struct AppStartupConfig {
    pub fps: u64,
}


#[derive(Debug)]
pub enum AppError {
    FBError(FramebufferError),
    RendererError(renderer::RendererError),
}

pub fn run<F>(scene: Box<dyn Scene>, startup_config: AppStartupConfig, error_handler: F)
where
    F: FnOnce(AppError) -> (),
{
    let mut app = match App::new(scene, startup_config) {
        Ok(app) => app,
        Err(e) => {
            error_handler(e);
            panic!("Error handler returned")
        }
    };

    match app.run() {
        Ok(_) => (),
        Err(e) => {
            error_handler(e);
            panic!("Error handler returned")
        }
    };
}

pub trait Scene {
    fn update(&mut self, renderer: &mut Renderer);
}

struct App {
    sleep_time: Duration,
    running: bool,

    scene: Box<dyn Scene>,
    renderer: Renderer,
}

impl App {
    fn new(scene: Box<dyn Scene>, startup_config: AppStartupConfig) -> Result<App, AppError> {
        let renderer = match Renderer::new() {
            Ok(renderer) => renderer,
            Err(e) => return Err(AppError::RendererError(e)),
        };

        Ok(App {
            sleep_time: Duration::from_millis(1000 / startup_config.fps),
            running: false,
            scene,
            renderer,
        })
    }

    fn run(&mut self) -> Result<(), AppError> {
        self.running = true;

        while self.running {
            let start = Instant::now();

            self.scene.update(&mut self.renderer);

            match self.renderer.render() {
                Ok(_) => (),
                Err(e) => return Err(AppError::RendererError(e)),
            }
            sleep(start - Instant::now() + self.sleep_time);
        }
        Ok(())
    }
}
