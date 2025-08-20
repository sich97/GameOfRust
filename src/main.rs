fn main() {
    println!("Hello, world!");
}

/*
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    rect: geom::Rect,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(200, 200).view(view).build().unwrap();
    let rect = geom::Rect::from_xywh(0.0, 0.0, 50.0, 50.0);
    Model { rect }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // You can update the model here, e.g., to move the rectangle
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .xy(model.rect.xy())
        .wh(model.rect.wh())
        .color(GREEN);
    draw.to_frame(app, &frame).unwrap();
}
*/
