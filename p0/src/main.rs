//The `use` imports stuff into the current namespace.
//The glob (`*`) after prelude means: Import everything from the `nannou::prelude` module.
//nannou's prelude module has the stuff that is most commonly used in nannou apps.
use nannou::prelude::*;

//The main function is the starting point of any rust app
fn main() {
    //double color is syntax for calling into a module namespace,
    // which in this case is calling the `app` function inside the `nannou`
    // module.
    //The `app` function argument is a function pointer to the `model` function.
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

//This is where your define what your Model.
//That means that if you e.g. want to draw circles in your program, you would add data fields in
// this struct that could describe for instance the positions, colors, and sizes for the circles.
struct Model {}

//The pointer to this function is used as an argument to the `nannou::app` function in the `main`
// function.
//The `Model` is the state of your program, so if you want to store the data for e.g. a list of 
// geometric object that will be read and modified throughout the programs runtime, this is the
// place to initialize it.
fn model(_app: &App) -> Model {
    Model {}
}

//The pointer to the `update` function is used as the argument to the `update` function in the
// `main` function above.
//This function runs once every frame, which is typically 60fps, or 60 times per second.
//In the `model` function you _initialized_ your Model, i.e. the program state.
//The `update` function is where you update and modify the data in your Model.
//Notice that the argument for the Model in the function parameters is declared `&mut`, which tell
// Rust that the Model argument for this function is a _shared mutable_, or _exclusively mutable_ 
// reference_.
//This means that Rust knows that you can modify the contents of the Model instance.
fn update(_app: &App, _model: &mut Model, _update: Update) {
}

//The pointer to the `view` function is used as the argument to the `view` in the `main` function.
//This is where you draw to the screen(s).
//You get the Model instance, so for instance if you have defined you Model to contain data about
//circles, this is the function where you use that data to set the graphics parameters.
//Notice that the Model parameter in this function is declared as type `&Model`, not `&mut Model` 
// as in the `update` function above.
//This means that the model is not possible to change when the program reaches here; this is a
// separation of model and view; the `update` function updates the _model_, the `view` function
// updates the _screen(s)_.
fn view(app: &App, _model: &Model, frame: Frame){
    //The `App` contains data about the program runtime:
    //- `time`: the runnning time of the application
    //- `window_rect()`: Function to get the screen dimensions
    //- `mouse`: mouse position and buttons
    //- `keys`: keyboard keys and modifiers

    //The `draw` instance is the current drawing that your are about to make.
    let draw = app.draw();

    //Use the draw instances' background and set the color.
    draw.background().color(CYAN);


    let num_items = 5;
    let rotation_radius = 120.0;
    for i in 0..num_items {
        let angle = i as f32 * (1.0/num_items as f32) * TAU + (app.time * 1.0);
        draw.ellipse()
            .x_y(
                rotation_radius * angle.cos(),
                rotation_radius * angle.sin(),
                )
            .radius(20.0)
            .color(GREEN);
    }

    //Use the draw instance to set its content to a reference to the Frame, i.e. window.
    draw.to_frame(app, &frame).unwrap();
}

