# Rust and `nannou` cheat sheet
This document gathers snippets of code that illustrates many of the more common things you may want to do in Rust or `nannou`.

Feel free to submit additional code snippet and cheats to this document. :smile:


## Application
### Get the running time or elapsed frames
`app.time` for the app time.

`app.elapsed_frames()` for the number of frames since start.

### Get window size
```rust
let r = app.window_rect(); // => Rect
r.w(); //total width
r.h(); //total height
r.top(); //top of the window
r.botton(); //bottom of the window
r.right(); //right edge of the window
r.left(); //left edge of the window
```

### Set the window size
```rust
fn main() {
  nannou::app(model)::update(update)::run();
}
fn model(app: &App) -> Model {
  //The new window is connected to the app using the `view()` function.
  //Notice that we don not call the `view()` function in the `main` function in this case.
  //The leading underscore means that we intend _not_ to use the resulting variable.
  let _win = app.new_window().size(1024, 720).view(view).build().unwrap();
  Model{}
}
```

## Shapes
### Draw circles and ellipses
```rust
draw.ellipse()
    .x_y(0.0, 0.0)      // set x, y, and z position as separate values
    .radius(100.0)      // set radius in pixels
    .color(GREEN)       // set fill color
    .no_fill()          // Don't fill the circle. This overrides the `.color` above
    .stroke(RED)        // The stroke color
    .stroke_weight(5.0);// thickness of stroke line;
```

## Geometry
### Draw points evenly around in a circle
```rust
let num_items = 5;
let rotation_radius = 120.0;
for i in 0..num_items {
    let angle = i as f32 * (1.0/num_items as f32) * TAU;
    draw.ellipse()
        .x_y(
            rotation_radius * angle.cos(),
            rotation_radius * angle.sin(),
            )
        .radius(20.0)
        .color(GREEN);
}
```

### Move the position of the drawing context
`let draw = draw.x_y(100.0, -40.0);`
The next thing you draw will now be at (x: 100, y:-40) pixels.

### Create 3D vector from 2D vector
Some functions specifically demand a `Vec3`.
So what to do when all you have is a `Vec2`?
```rust
let a = vec2(100.0, -40.0);
let b = a.extend(0.0); //extends into 3 dimensions with 0.0 as the z value
//In this case, translate does the same as the `draw.x_y()` examples above,
// i.e. it moves the position of the drawing context.
let draw = draw.translate(b); 
```

## Math
### Generate a random value
`random()` produces decimal numbers in the range of 0.0 - 1.0
Exists in different versions depending on value type, so must be specified using
_turbofish_ syntax:
`random::<f32>()` for 32-bit floats
`random::<f64>()` for 64-bit floats

### Convert number ranges
`map_range(<in>, <inMin>, <inMax>, <outMin>, <outMax>)`

### Clip/clamp number ranges
`clamp(<in>, <min>, <max>);`

### Get min or max of two numbers
`partial_max` or `partial_min`

## Rust

### Define a `struct` with a constructor
```rust
struct Widget{
    position: Vec2,
    name: String,
}
impl Widget{
    fn new(p: Vec2, n: String) -> Self {
        Widget{
            position: p,
            name: n,
        }
    }
}
```
