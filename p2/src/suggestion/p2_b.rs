use nannou::prelude::*;

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

    draw.background().color(BLACK);
    //import the point module from the lyon crate, so we don't have to
    // write the whole namespace every time.
    use nannou::lyon::math::point;

    //Make a builder object that defines paths
    let mut builder = nannou::geom::path::Builder::new().with_svg();
    //Move the 'pen' to 20.0 pizels above the bottom of the screen
    builder.move_to(point(0.0, win.bottom() + 20.0));

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

    //Start the Drawing of a Path
    draw.path()
        .stroke()
        .color(WHITE)
        .weight(1.0)
        .events(path.iter()); //Get an iterator for the path to add the points to the path drawing

    //Add another line with the path, just with the draw context scaled with a negative value
    // effetively mirroring it
    draw.scale_x(-1.0)
        .path()
        .stroke()
        .color(WHITE)
        .weight(1.0)
        .events(path.iter());

    draw.to_frame(app, &frame).unwrap();

}
