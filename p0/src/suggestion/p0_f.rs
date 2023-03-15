use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(CYAN);

    let circle_radius_a = 50.0;
    let pos_a = vec2( (app.time * 0.5).sin() * app.window_rect().w() * 0.25, 0.0);

    draw.ellipse()
        .xy(pos_a)
        .radius(circle_radius_a)
        .color(MAGENTA);

    //Instead of moving the next circle, we move the whole draw instance to a new position.
    //This effectively moves the center for the drawing to the position of circle `a`, thus drawing
    // circle `b` in relation to that point becomes much easier.
    let draw = draw.xy(pos_a);

    let circle_radius_b = circle_radius_a / 2.0;
    let pos_b = vec2(
        (3.0 * app.time).sin() * 100.0,
        (3.0 * app.time).cos() * 100.0,
        );

    draw.ellipse()
        .xy(pos_b)
        .radius(circle_radius_b)
        .color(ORANGE);

    draw.to_frame(app, &frame).unwrap();
}

