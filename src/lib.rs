pub use glam;
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

    fn dispose(&mut self) {
        self.canvas.dispose();
    }
}

pub fn render<F>(mut app: App, mut f: F)
where
    F: FnMut(&mut App, u128) + std::marker::Send + 'static,
{
    let (tx, rx) = mpsc::channel();

    let timer = Instant::now();
    let render_thread = thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                // Terminating
                app.dispose();
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        app.canvas.clear_screen();

        let (canvas_col, canvas_row) = canvas::Canvas::get_size();
        let sphere = app.scene.geometries.first().unwrap();

        for j in 1..=canvas_row {
            for i in 1..=canvas_col {
                // build ray
                let r = ray::Ray::new(app.camera.position, {
                    // remap x, y to (-1.0, 1.0)
                    let (x, y) = (
                        util::remap((i - 1) as f32, (0.0, (canvas_col - 1) as f32), (-1.0, 1.0)),
                        util::remap((j - 1) as f32, (0.0, (canvas_row - 1) as f32), (1.0, -1.0)),
                    );
                    Vec3::new(x, y, -1.0).normalize()
                });

                if let Some(hit_point) = r.hit(sphere) {
                    let mut color = color::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    };

                    // ambient
                    color += 0.5;

                    // diffuse
                    let light = app.scene.lights.first().unwrap();
                    let light_pos = light.get_pos();
                    let p_nor = (hit_point - sphere.origin).normalize();
                    let angle = p_nor.dot((*light_pos - p_nor).normalize());
                    color += if angle >= 0.0 {
                        util::remap(angle, (0.0, 1.0), (0.0, 0.5))
                    } else {
                        0.0
                    };

                    // todo: specular
                    app.canvas.write(i, j, color.get_char());
                } else {
                    app.canvas.write(i, j, ' ')
                }
            }
        }

        app.canvas.flush();

        thread::sleep(Duration::from_millis(200));

        f(&mut app, timer.elapsed().as_millis());
    });

    let stdin = stdin();
    let keys = stdin.keys();
    for c in keys {
        match c.unwrap() {
            // Exit.
            Key::Char('q') => {
                tx.send(()).unwrap();
                break;
            }
            _ => (),
        }
    }

    render_thread.join().unwrap();
}
