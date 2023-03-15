use nannou::prelude::*;
use nannou::math::ConvertAngle;
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

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    //Replace the `background()` with this to be able to use transparent colors
    draw.rect()
        .x_y(0.0, 0.0) 
        .w_h(win.w(), win.h()) //fill the whole window
        .color(rgba(0.0, 0.0, 0.0, 0.01)); //Very transparent black color

    //import the point module from the lyon crate, so we don't have to
    // write the whole namespace every time.
    use nannou::lyon::math::point;

    //Make a builder object that defines paths
    let mut builder = nannou::geom::path::Builder::new().with_svg();
    //Move the 'pen' to 20.0 pizels above the bottom of the screen
    builder.move_to(point(0.0, win.bottom())); //removed the offset to go to the center

    // Sine oscillator swinging slowly
    let width = (app.time * 0.05).sin() * 400.0 + 40.0;

    //Set bezier control points
    let control_a = point((app.time * 0.3 ).cos() * width, (app.time * 0.1).sin() * width);
    let control_b = point((app.time * 0.3 + 0.1).sin() * width, (app.time * 0.1).cos() * width);

    //Add the bezier segment to the path builder
    builder.cubic_bezier_to(
        control_a,
        control_b,
        point(0.0, win.top() - 20.0)
        );

    // Build a path with the path builder.
    let path = builder.build();

    let hue = (app.time * 0.05).cos() * 0.5 + 0.5;
    let hue = (hue * 360.0 - 180.0) * 10.0;
    let color = Lch::new( 
        90.0,
        10.0,
        hue
        );
    const NUM_ARMS: u16 = 12; //Make a constant value

    let draw = draw.scale(0.5); // scale down the draw context the draw in half the size
    //Iterate over the numbers 0 to 11.
    //The 0..NUM_ARMS is an non-inclusive range.
    //If you wanted the numbers 0 to 12 you would write 0..=NUM_ARMS
    // for an inclusive range.
    for i in  0..NUM_ARMS {
        let angle = i as f32 * (360.0 / NUM_ARMS as f32);
        draw.rotate(angle.deg_to_rad())
            .translate(vec3(0.0, win.h() * 0.5, 0.0)) //move it to the center of the screen
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
