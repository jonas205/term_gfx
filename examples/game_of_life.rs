use std::process::exit;

use term_gfx::{
    app::{AppError, AppStartupConfig, Scene},
    profile, Color, Renderer,
};

const BORDER_COLOR: u8 = 127;

fn error_handler(err: AppError) {
    eprintln!("Got an error: {:?}", err);
    exit(-1);
}

struct GameOfLifeScene {
    cells: Vec<bool>, 
    width: usize,
    height: usize,
}

impl GameOfLifeScene {
    fn neighbours_helper(&self, old: &Vec<bool>, x: i32, y: i32) -> u32 {
        if y < 0 || x < 0 || y as usize >= self.height || x as usize >= self.width {
            return 0;
        }

        if old[(y as usize) * self.width + (x as usize)] {
            1
        } else {
            0
        }
    }

    fn neighbours(&self, old: &Vec<bool>, x: i32, y: i32) -> u32 {
        let mut erg = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    erg += self.neighbours_helper(old, x + i, y + j);
                }
            }
        }
        erg
    }

    fn cycle(&mut self) {
        profile!();
        let old = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let n = self.neighbours(&old, x as i32, y as i32);

                if old[y * self.width + x] {
                    self.cells[y * self.width + x] = n == 2 || n == 3;
                } else {
                    self.cells[y * self.width + x] = n == 3;
                }
            }
        }
    }
}

impl Scene for GameOfLifeScene {
    fn detach(&mut self) {}

    fn attach(&mut self, app_info: &term_gfx::app::AppInfo) {
        profile!();
        let (w, h) = app_info.renderer.as_ref().borrow().screen_size();
        let w = (w - 2) as usize;
        let h = (h - 2) as usize;

        self.width = w;
        self.height = h;

        self.cells = Vec::with_capacity(h * w);

        for _ in 0..(w * h) {
            self.cells.push(false);
        }


        self.cells[3 * self.width + 5] = true;
        self.cells[4 * self.width + 5] = true;
        self.cells[5 * self.width + 5] = true;
        self.cells[5 * self.width + 4] = true;
        self.cells[4 * self.width + 3] = true;
    }

    fn update(&mut self, renderer: &mut Renderer) {
        let (w, h) = renderer.screen_size();

        {
            profile!("Draw Border");
            for i in 0..w {
                renderer.pixel(i, 0, Color::grey(BORDER_COLOR));
                renderer.pixel(i, h - 1, Color::grey(BORDER_COLOR));
            }

            for i in 0..h {
                renderer.pixel(0, i, Color::grey(BORDER_COLOR));
                renderer.pixel(w - 1, i, Color::grey(BORDER_COLOR));
            }
        }

        self.cycle();

        {
            profile!("Draw Grid");
            for y in 1..(h - 1) {
                for x in 1..(w - 1) {
                    let i = (y as usize - 1) * self.width + (x as usize - 1);
                    if self.cells[i] {
                        renderer.pixel(x, y, Color::rgb(0, 255, 0));
                    } else {
                        renderer.pixel(x, y, Color::rgb(0, 127, 0));
                    }
                }
            }
        }
    }
}

fn main() {
    #[cfg(not(feature = "profiling"))]
    {
        println!("Please enable profiling for this example: cargo run --example game_of_life --features profiling");
        exit(-1);
    }

    println!("\x1b[?1003h");
    loop {}

    let cfg = AppStartupConfig { fps: 60 };

    let scene = Box::new(GameOfLifeScene { cells: Vec::new(), width: 0, height: 0 });

    term_gfx::run(scene, cfg, error_handler);
}
