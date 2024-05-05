use std::{
    borrow::BorrowMut,
    cell::RefCell,
    io,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    event::{Event, EventHandler}, framebuffer::{self, FramebufferError}, profile, profiler::Profiler, renderer, Framebuffer, Renderer
};

pub struct AppStartupConfig {
    pub fps: u64,
}

#[derive(Debug)]
pub enum AppError {
    FBError(FramebufferError),
    RendererError(renderer::RendererError),
    IOError(io::Error),
}

pub fn run<F>(scene: Box<dyn Scene>, startup_config: AppStartupConfig, error_handler: F)
where
    F: FnOnce(AppError) -> (),
{
    let _p = match Profiler::new() {
        Ok(p) => p,
        Err(e) => {
            error_handler(AppError::IOError(e));
            panic!("Error handler returned");
        }
    };

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
    fn event(&mut self, event: &Event);
    fn attach(&mut self, app_info: &AppInfo);
    fn detach(&mut self);
}

pub struct AppInfo {
    pub renderer: Rc<RefCell<Renderer>>,
}

impl AppInfo {}

impl Clone for AppInfo {
    fn clone(&self) -> Self {
        AppInfo {
            renderer: self.renderer.clone(),
        }
    }
}

struct App {
    sleep_time: Duration,
    app_info: AppInfo,
    scene: Box<dyn Scene>,
    event_handler: EventHandler,
    pub running: Arc<Mutex<bool>>,
}

impl App {
    fn new(scene: Box<dyn Scene>, startup_config: AppStartupConfig) -> Result<App, AppError> {
        profile!();
        let renderer = match Renderer::new() {
            Ok(renderer) => renderer,
            Err(e) => return Err(AppError::RendererError(e)),
        };

        let running = Arc::new(Mutex::new(false));
        let mut running_closure = running.clone();

        ctrlc::set_handler(move || {
            *running_closure.borrow_mut().lock().unwrap() = false;
        })
        .unwrap();

        let event_handler = EventHandler::new();

        Ok(App {
            sleep_time: Duration::from_millis(1000 / startup_config.fps),
            scene,
            running,
            event_handler,
            app_info: AppInfo {
                renderer: Rc::new(RefCell::new(renderer)),
            },
        })
    }

    fn run_iteration(&mut self, events: &Vec<Event>) -> Result<(), AppError> {
        profile!();
        let start = Instant::now();

        for e in events {
            profile!("User Scene Event");

            if let Event::Resize(w, h) = e {
                self.app_info.renderer.as_ref().borrow_mut().resize(*w, *h);
            }

            self.scene.event(e);
        }

        {
            profile!("User Scene Update");
            self.scene
                .update(&mut self.app_info.renderer.as_ref().borrow_mut());
        }


        match self.app_info.renderer.as_ref().borrow_mut().render() {
            Ok(_) => (),
            Err(e) => return Err(AppError::RendererError(e)),
        }

        let loop_time = Instant::now() - start;
        if loop_time < self.sleep_time {
            profile!("Sleep");
            sleep(self.sleep_time - loop_time);
        }
        Ok(())
    }

    fn run(&mut self) -> Result<(), AppError> {
        profile!();
        // Setup
        *self.running.borrow_mut().lock().unwrap() = true;
        self.scene.attach(&self.app_info);

        // Loop
        while *self.running.borrow_mut().lock().unwrap() {
            let events = self.event_handler.get_events();

            // for e in events {
            //     println!("{:?}", e);
            // }

            self.run_iteration(&events)?;
        }

        // Cleanup
        let (_, h) = self.app_info.renderer.as_ref().borrow_mut().screen_size();
        for _ in 0..h {
            println!();
        }

        Ok(())
    }
}
