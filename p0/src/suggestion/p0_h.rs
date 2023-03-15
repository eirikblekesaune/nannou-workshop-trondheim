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
    let win = app.window_rect();

    draw.background().color(CYAN);

    let circle_radius_a = 50.0;
    let pos_a = vec2( (app.time * 0.5).sin() * app.window_rect().w() * 0.25, 0.0);

    draw.ellipse()
        .xy(pos_a)
        .radius(circle_radius_a)
        .color(MAGENTA);

    let circle_radius_b = circle_radius_a / 2.0;
    let pos_b = vec2(
        (3.0 * app.time).sin() * 100.0,
        (3.0 * app.time).cos() * 100.0,
        );

    draw.xy(pos_a) // change the drawing context, this time temporarily
        .ellipse() // then start a drawing of an ellipse
        .xy(pos_b) // and set its position, relative to the draw context
        .radius(circle_radius_b)
        .color(ORANGE);

    draw.tri()
        .x_y(0.0, 0.0)
        .color(BLUE);

    //Draw the basic structure of the building
    let unit_size = 50.0;
    let num_floors = 4;
    let windows_per_floor = 2;
    let building_rect = Rect::from_w_h(
        unit_size * windows_per_floor as f32, // convert the i32 primitive type to f32 using the
                                              // `as` keyword. This way of converting types is only
                                              // applicable for primitive types such as f32, u32
                                              // etc.
        unit_size * num_floors as f32
        )
        .mid_bottom_of(win);
    draw.rect()
        .xy(building_rect.xy()) // use the rect properties directly on the drawing
        .wh(building_rect.wh())
        .color(GREY);

    draw.ellipse()
        .xy(pos_a + pos_b)
        .radius(10.0)
        .color(RED);

    draw.to_frame(app, &frame).unwrap();
}

