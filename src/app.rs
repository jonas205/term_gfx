use std::{cell::RefCell, rc::Rc, thread::sleep, time::{Duration, Instant}};

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
    fn attach(&mut self, app_info: &AppInfo);
    fn detach(&mut self);
}

pub struct AppInfo {
    pub running: Rc<RefCell<bool>>,
    pub renderer: Rc<RefCell<Renderer>>,
}

impl AppInfo {
    pub fn running(&self) -> bool {
        *self.running.as_ref().borrow()
    }
    pub fn set_running(&mut self, r: bool) {
        *self.running.as_ref().borrow_mut() = r;
    }
}

impl Clone for AppInfo {
    fn clone(&self) -> Self {
        AppInfo {
            running: self.running.clone(),
            renderer: self.renderer.clone(),
        }
    }
}

struct App {
    sleep_time: Duration,
    app_info: AppInfo,
    scene: Box<dyn Scene>,
}

impl App {
    fn new(scene: Box<dyn Scene>, startup_config: AppStartupConfig) -> Result<App, AppError> {
        let renderer = match Renderer::new() {
            Ok(renderer) => renderer,
            Err(e) => return Err(AppError::RendererError(e)),
        };

        Ok(App {
            sleep_time: Duration::from_millis(1000 / startup_config.fps),
            scene,
            app_info: AppInfo {
                running: Rc::new(RefCell::new(false)),
                renderer: Rc::new(RefCell::new(renderer)),
            }
        })
    }

    fn run(&mut self) -> Result<(), AppError> {
        self.app_info.set_running(true);

        self.scene.attach(&self.app_info);
        while self.app_info.running() {
            let start = Instant::now();

            self.scene.update(&mut self.app_info.renderer.as_ref().borrow_mut());

            match self.app_info.renderer.as_ref().borrow_mut().render() {
                Ok(_) => (),
                Err(e) => return Err(AppError::RendererError(e)),
            }
            sleep(start - Instant::now() + self.sleep_time);
        }
        Ok(())
    }
}
