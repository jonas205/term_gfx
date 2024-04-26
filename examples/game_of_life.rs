use std::process::exit;

use term_gfx::{
    app::{AppError, AppStartupConfig, Scene},
    Color, Renderer,
};

const BORDER_COLOR: u8 = 127;

fn error_handler(err: AppError) {
    eprintln!("Got an error: {:?}", err);
    exit(-1);
}

struct GameOfLifeScene {
    cells: Vec<Vec<bool>>,
}

impl GameOfLifeScene {

    fn neighbours_helper(old: &Vec<Vec<bool>>, x: i32, y: i32) -> u32 {
        if y < 0 || x < 0 || y as usize >= old.len() || x as usize >= old[0].len() {
            return 0;
        }

        if old[y as usize][x as usize] { 1 } else { 0 }
    }

    fn neighbours(old: &Vec<Vec<bool>>, x: i32, y: i32) -> u32 {
        let mut erg = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    erg += Self::neighbours_helper(old, x + i, y + j);
                }
            }
        }
        erg
    }

    fn cycle(&mut self) { 
        let old = self.cells.clone();

        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                let n = Self::neighbours(&old, x as i32, y as i32);

                if old[y][x] {
                    self.cells[y][x] = n == 2 || n == 3;
                } else {
                    self.cells[y][x] = n == 3;
                }
            }
        }
    }
}

impl Scene for GameOfLifeScene {
    fn detach(&mut self) {}

    fn attach(&mut self, app_info: &term_gfx::app::AppInfo) {
        let (w, h) = app_info.renderer.as_ref().borrow().screen_size();
        let w = (w - 2) as usize;
        let h = (h - 2) as usize;

        self.cells = Vec::with_capacity(h);

        for _ in 0..h {
            let mut row = Vec::with_capacity(w);
            for _ in 0..w {
                row.push(false);
            }
            self.cells.push(row);
        }

        self.cells[5][3] = true;
        self.cells[5][4] = true;
        self.cells[5][5] = true;
        self.cells[4][5] = true;
        self.cells[3][4] = true;
    }

    fn update(&mut self, renderer: &mut Renderer) {
        let (w, h) = renderer.screen_size();

        for i in 0..w {
            renderer.pixel(i, 0, Color::grey(BORDER_COLOR));
            renderer.pixel(i, h - 1, Color::grey(BORDER_COLOR));
        }

        for i in 0..h {
            renderer.pixel(0, i, Color::grey(BORDER_COLOR));
            renderer.pixel(w - 1, i, Color::grey(BORDER_COLOR));
        }

        self.cycle();

        for y in 1..(h - 1) {
            for x in 1..(w - 1) {
                if self.cells[y as usize - 1][x as usize - 1] {
                    renderer.pixel(x, y, Color::rgb(0, 255, 0));
                } else {
                    renderer.pixel(x, y, Color::rgb(0, 127, 0));
                }
            }
        }

    }
}

fn main() {
    let cfg = AppStartupConfig { fps: 5 };

    let scene = Box::new(GameOfLifeScene {
        cells: Vec::new(),
    });

    term_gfx::run(scene, cfg, error_handler);
}
