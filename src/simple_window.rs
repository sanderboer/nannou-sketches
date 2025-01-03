use nannou::prelude::*;
use nannou::wgpu::{DeviceDescriptor, Limits};

fn main() {
    nannou::app(model).run();
}

struct Model;

fn model(app: &App) -> Model {
     let device_desc = DeviceDescriptor {
                limits: Limits::downlevel_webgl2_defaults(),
                .. DeviceDescriptor::default()
     };
    let img_w = 800;
    let img_h = 600;
    let w_id = app
        .new_window()
        .size(img_w, img_h)
        .device_descriptor(device_desc)
        .view(view)
        .build()
        .unwrap();

    Model
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    // Clear the frame to black
    frame.clear(BLACK);
}

// use nannou::prelude::*;

// struct Model {}

// fn main() {
//     nannou::app(model)
//         .event(event)
//         .simple_window(view)
//         .run();
// }

// fn model(_app: &App) -> Model {
//     Model {}
// }

// fn event(_app: &App, _model: &mut Model, _event: Event) {
// }

// fn view(_app: &App, _model: &Model, _frame: Frame) {
// }
