use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

//Use the Model struct to hold the data that we need during the duration of our program
struct Model {
    circle_a: Circle,
    circle_b: Circle,
}

struct Circle {
    position: Vec2, // this type stores 2 dimensional positions
    speed: f32, // 32-bit float
    radius: f32,
    color: Rgb<u8>, //A color type with 3 8 bit values. One for each color Red, Green and Blue
}

//The impl keyword is used to implement functions of structs
impl Circle {
    //The first arg is &mut, which tells us that we intend to modify
    // something in our struct by calling this method
    fn grow(&mut self, by: f32) {
        self.radius += by;
    }
    fn shrink(&mut self, by: f32) {
        self.grow(-by); // call itself with negated value, more DRY
    }
}

// Our doubling function which double the f32 value you give it
fn double_it(x: f32) -> f32 {
    x * 2.0
}

//We use this function to initialize our program's state, i.e. the properties
// for our circles.
//This function is called only once, from the `main` function.
fn model(app: &App) -> Model {
    let r = app.window_rect();
    let radius_a = 50.0;
    let a = Circle {
        position: vec2(r.right() - radius_a, 0.0),
        speed: 1.0,
        radius: radius_a,
        color: MAGENTA,
    };
    let b = Circle {
        position: vec2(r.right() - (a.radius / 2.0), 0.0),
        speed: double_it(a.speed),
        radius: a.radius / 2.0,
        color: ORANGE,
    };
    //Notice that the last line has no semicolon.
    //This is how we describe what is returned from our function.
    //Looking at the function signatur above, the return type is notated
    // with `-> Model`, so this looks aaaall right.
    Model {
        circle_a: a,
        circle_b: b,
    }
}

//Notice we removed the leading underscore to the `app` and `model`
// parameter. Stricly speaking we aren't required to do this; we could just
// use `_app` inside our function, but the reason for the the leading underscore
// is to tell the Rust compiler that we intend not to use the parameter in our
// code. When we don't prepend an underscore to a parameter name, the compiler
// will warn us, as this may be a cause for bugs.
fn update(app: &App, model: &mut Model, _update: Update) {
    let r = app.window_rect();
    let time = app.time;

    //The `&mut` syntax means that we assign a mutable reference to the 
    // `circle` variable. This means that the circle variable does not own
    // the model.circle_a instance, but rather holds a reference to it. And 
    // since the reference is mutable, it means that we can change the value 
    // of its data members.
    let circle = &mut model.circle_a;
    let ts = time * circle.speed;
    circle.grow(0.02);
    let radius = circle.radius;
    circle.position = vec2(
        map_range( ts.sin(), -1.0, 1.0, r.left() + radius, r.right() - radius ),
        map_range( ts.cos(), -1.0, 1.0, r.bottom() + radius, r.top() - radius ),
        );

    let circle = &mut model.circle_b;
    let ts = time * circle.speed;
    circle.shrink(0.01);
    let radius = circle.radius;
    circle.position = vec2(
        map_range( ts.sin(), -1.0, 1.0, r.left() + radius, r.right() - radius ),
        map_range( ts.cos(), -1.0, 1.0, r.bottom() + radius, r.top() - radius ),
        );
}

//Since we have done all the modification of state in the update function,
// we can only draw the results here. This design choice separates the view
// from the model.
fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(CYAN);

    //In the update function we accessed `circle_a` using a mutable reference.
    //In this function we assign a read-only reference to `circle_a`, meaning
    // that we can't change its value. Since the parameter, `model &Model`, is 
    // not declared mutable, we wouldn't be able to assign it as a mutable 
    // reference even if we wanted to.
    let circle = &model.circle_a;
    draw.ellipse()
        .xy(circle.position)
        .radius(circle.radius)
        .color(circle.color);

    let circle = &model.circle_b;
    draw.ellipse()
        .xy(circle.position)
        .radius(circle.radius)
        .color(circle.color);

    draw.to_frame(app, &frame).unwrap();
}

