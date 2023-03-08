use nannou::prelude::*;
use nannou::color::{Lch, Srgb, Hue};

fn main() {
    nannou::sketch(view)
        .run();
}

fn get_lab_complementary(color: &Srgb) -> Srgb {
    let c: Lch = color.into_linear().into();
    let c = c.shift_hue(180.0);
    Srgb::from(c)
}

fn view(app: &App, frame: Frame){
    let draw = app.draw();
    let time = app.time;

    let background_color = BLACK;

    let outer_color = hsv((time / 5.0) % 1.0, 1.0, 0.5);
    let inner_color = get_lab_complementary(&outer_color.into());
    let x = 0.0;
    let radius = app.window_rect().w().min(app.window_rect().h()) * 0.5;
    draw.ellipse()
        .x_y(x, 0.0)
        .radius(radius)
        .color(outer_color);
    draw.ellipse()
        .x_y(x, 0.0)
        .radius(radius * 0.5)
        .color(inner_color);

    draw.background().color(background_color);
    draw.to_frame(app, &frame).unwrap();
}

