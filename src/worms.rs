// Heavily inspired from https://www.local-guru.net/blog/2020/12/24/nannou-experiment---particles

use nannou::noise::NoiseFn;
use nannou::rand::*;
use nannou::prelude::*;
use nannou::wgpu::{DeviceDescriptor, Limits};

const CAPTURE: bool = false;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
}

impl Particle {
    fn new(x:f32, y:f32) -> Particle {
        Particle {
            pos: vec2(x,y),
            vel: vec2(0.,0.),
        }
    }

    fn update(&mut self, dir: Vec2) {
        self.pos += self.vel;
        self.vel += dir;
        self.vel *= 0.8;
    }
}


struct Model {
    particles: Vec<Particle>
}

fn model(app: &App) -> Model {
    //app.new_window().size(600,600).view(view).build().unwrap();
    let device_desc = DeviceDescriptor {
                limits: Limits::downlevel_defaults(),
                .. DeviceDescriptor::default()
     };

    app
        .new_window()
        .device_descriptor(device_desc)
        .size(600,600)
        .view(view)
        .build()
        .unwrap();
    // app.main_window().set_fullscreen(true);
    let r = app.window_rect().right() as f32;
    let l = app.window_rect().left() as f32;

    let w = l - r;
    let t = app.window_rect().top() as f32;
    let b = app.window_rect().bottom() as f32;

    let h = t - b;

    let mut p = vec![];
    for _i in 0..2000 {
        let x = random_f32() * w + r;
        let y = random_f32() * h + b;
        p.push(Particle::new(x,y));
    }

    Model{
        particles: p
    }
}

fn update(app: &App, model: &mut Model, _update:Update) {
    let noise = nannou::noise::Perlin::new();
    let t = app.elapsed_frames() as f64/ 100.;
    for i in 0..model.particles.len() {
        let p = &mut model.particles[i];
        let x = noise.get([ p.pos.x as f64 / 128., p.pos.y as f64 / 137., t + i as f64/ 1000. ]);
        let y = noise.get([ -p.pos.y as f64 / 128., p.pos.x as f64 / 137., t + i as f64/ 1000. ]);

        let a = vec2(x as f32, y as f32);
        p.update(a);
    }
}

fn view( app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let t = (app.elapsed_frames() as f32) * 0.02;
    let w = (t * 0.832).cos();
    for p in &model.particles {
        draw.ellipse().xy(p.pos).w_h(2.0, 2.0).color(hsla(0.1,1. + w,5.,0.01));
    }
    draw.to_frame(app, &frame).unwrap();
    if CAPTURE {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}
