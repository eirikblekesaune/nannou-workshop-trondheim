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
    let r = app.window_rect();

    draw.background().color(CYAN);

    let circle_radius_a = 50.0;
    let pos_a = vec2(
        map_range(app.time.sin(), -1.0, 1.0, r.left() + circle_radius_a,   r.right() - circle_radius_a),
        map_range(app.time.cos(), -1.0, 1.0, r.bottom() + circle_radius_a, r.top() - circle_radius_a),
        );

    let circle_radius_b = circle_radius_a / 2.0;
    let pos_b = vec2(
        map_range((2.0 * app.time).sin(), -1.0, 1.0, r.left() + circle_radius_b,   r.right() - circle_radius_b ),
        map_range((2.0 * app.time).cos(), -1.0, 1.0, r.bottom() + circle_radius_b, r.top() - circle_radius_b ),
        );

    //Use the draw instance to draw an ellipse.
    draw.ellipse()
        .xy(pos_a)
        .radius(circle_radius_a)
        .color(MAGENTA);
    draw.ellipse()
        .xy(pos_b)
        .radius(circle_radius_b)
        .color(ORANGE);

    draw.to_frame(app, &frame).unwrap();
}

