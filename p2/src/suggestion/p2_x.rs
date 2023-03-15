use nannou::lyon;
use nannou::math::ConvertAngle;
use nannou::prelude::*;
use nannou::color::Lch;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model{}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn multismoothstep(num_steps: u32, x: f32, steepness: f32) -> f32 {
    let n = num_steps as f32 - 1.0;
    let xn = x * n;
    let step_index = xn as i32;
    let remainder = xn - (step_index as f32);
    let steepness = clamp(steepness, 0.0, 1.0) * 0.5;
    (step_index as f32 + smoothstep(steepness, abs(1.0 - steepness), remainder)) / n
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(win.w(), win.h())
        .color(rgba(0.0, 0.0, 0.0, 0.01));

    use lyon::math::point;
    let mut builder = nannou::geom::path::Builder::new().with_svg();
    builder.move_to(point(0.0, win.bottom()));

    let width = (app.time * 0.05).sin() * 400.0 + 40.0;

    let control_a = point((app.time * 0.3 ).cos() * width, (app.time * 0.1).sin() * width);
    let control_b = point((app.time * 0.3 + 0.1).sin() * width, (app.time * 0.1).cos() * width);
    builder.cubic_bezier_to(
        control_a,
        control_b,
        point(0.0, win.top() - 20.0)
        );
    let path = builder.build();

    let draw = draw.scale(0.5);
    let hue = multismoothstep(9, (app.time * 0.05).cos() * 0.5 + 0.5, 0.7);
    let hue = (hue * 360.0 - 180.0) * 10.0;
    let color = Lch::new( 
        90.0,
        10.0,
        hue
        );

    const NUM_ARMS: u16 = 12;
    for i in 0..NUM_ARMS {
        let angle = i as f32 * (360.0 / NUM_ARMS as f32);
        draw.rotate(angle.deg_to_rad())
            .translate(vec3(0.0, win.h() * 0.5, 0.0))
            .path()
            .stroke()
            .color(color)
            .weight(1.0)
            .events(path.iter());

        draw.rotate(angle.deg_to_rad())
            .translate(vec3(0.0, win.h() * 0.5, 0.0))
            .scale_x(-1.0)
            .path()
            .stroke()
            .color(color)
            .weight(1.0)
            .events(path.iter());
    }
    draw.to_frame(app, &frame).unwrap();

}
