pub use glam as math;
mod camera;
pub mod canvas;
pub mod color;
pub mod geometry;
pub mod light;
mod ray;
mod scene;
mod term;
mod util;

use glam::Vec3;
use std::io::stdin;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use termion;
use termion::event::Key;
use termion::input::TermRead;

pub struct App {
    pub canvas: canvas::Canvas,
    pub camera: camera::Camera,
    pub scene: scene::Scene,
}

impl App {
    pub fn new() -> Self {
        App {
            camera: camera::Camera::new(Vec3::new(0.0, 0.0, 20.0)),
            scene: scene::Scene::new(),
            canvas: canvas::Canvas::new(),
        }
    }

    pub fn render<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut App, u128) + std::marker::Send + 'static,
        Self: 'static,
    {
        let (tx, rx) = mpsc::channel();

        let handler = thread::spawn(move || {
            let stdin = stdin();
            let keys = stdin.keys();
            for c in keys {
                match c.unwrap() {
                    // Exit.
                    Key::Char('q') | Key::Esc => {
                        tx.send(()).unwrap();
                        break;
                    }
                    _ => (),
                }
            }
        });

        let timer = Instant::now();
        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    // Terminating
                    self.dispose();
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }

            self.canvas.hide_cursor();

            let (canvas_col, canvas_row) = canvas::Canvas::get_size();

            for j in 1..=canvas_row {
                for i in 1..=canvas_col {
                    // build ray
                    let r = ray::Ray::new(self.camera.position, {
                        // remap x, y to (-1.0, 1.0)
                        let (x, y) = (
                            util::remap(
                                (i - 1) as f32,
                                (0.0, (canvas_col - 1) as f32),
                                (-1.0, 1.0),
                            ),
                            util::remap(
                                (j - 1) as f32,
                                (0.0, (canvas_row - 1) as f32),
                                (1.0, -1.0),
                            ),
                        );
                        Vec3::new(x, y, -1.0).normalize()
                    });

                    let mut color = color::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    };

                    self.scene.intersect(&r, &mut color);

                    self.canvas.write_with_bg(i, j, color);
                }
            }

            self.canvas.flush();

            thread::sleep(Duration::from_millis(10));

            f(self, timer.elapsed().as_millis());
        }

        handler.join().unwrap();
    }

    fn dispose(&mut self) {
        self.canvas.dispose();
    }
}
