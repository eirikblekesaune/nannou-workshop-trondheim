# Rust and `nannou` cheat sheet
This document gathers snippets of code that illustrates many of the more common things you may want to do in Rust or `nannou`.

Feel free to submit additional code snippet and cheats to this document. :smile:


## `nannou`
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

### Move, scale, and rotate the position of the drawing context
`let draw = draw.x_y(100.0, -40.0);`
The next thing you draw will now be at (x: 100, y:-40) pixels.

`let draw = draw.scale(2.0)`
Everything you draw from now will be twice as large.

`let draw = draw.rotate(90.0.deg_to_rad());`


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

### Draw gradient colored polygon
Use a Vector of (Vec2, Rgb) tuples as elements.
The `points_colored()` function interpolates the colors.
```rust
let a = 100.0;
let points = vec![
  (pt2(-a, a), rgb(1.0, 0.0, 0.0)),
  (pt2(a, a), rgb(0.0, 1.0, 0.0)),
  (pt2(a, -a), rgb(1.0, 1.0, 0.0)),
  (pt2(-a, -a), rgb(1.0, 0.0, 1.0)),
];
draw.polygon().points_colored(points);
```

### Draw a line
The two endpoint of the line is set using the `.points()` function:
```rust
let a = pt2(/*...*/);
let b = pt2(/*...*/);
draw.polyline()
    .weight(1.0)
    .points(a, b)
    .color(rgba(1.0, 1.0, 1.0, 0.8));
```

### Draw multiline
Make a Vector of points and draw using the `.points()` function:
```rust
let = (0..20).map(|i| pt2(/*... point coords here..*/));
draw.polyline()
    .weight(1.0)
    .points(points)
    .color(rgba(1.0, 1.0, 1.0, 0.8));
```

## Geometry
### Draw points evenly around in a circle
```rust
let num_items = 5;
let rotation_radius = 120.0;
for i in 0..num_items {
    let angle = i as f32 * (num_items as f32).recip() * TAU + (app.time * 1.0);
    draw.ellipse()
        .x_y(
            rotation_radius * angle.cos(),
            rotation_radius * angle.sin(),
            )
        .radius(20.0)
        .color(GREEN);
}
```

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

## Colors
### Blend color modes
A draw context can be set to a given color blending mode using `.color_blend()`, using one of the following constants as argument:
* `BLEND_NORMAL`
* `BLEND_ADD`
* `BLEND_SUBTRACT`
* `BLEND_REVERSE_SUBTRACT`
* `BLEND_DARKEST`
* `BLEND_LIGHTEST`

## Math
### Generate a random value
`random()` produces decimal numbers in the range of 0.0 - 1.0
Exists in different versions depending on value type, so must be specified using
_turbofish_ syntax:
`random::<f32>()` for 32-bit floats
`random::<f64>()` for 64-bit floats

`random_range()` generates a number within a specified range.

### Convert number ranges
`map_range(<in>, <inMin>, <inMax>, <outMin>, <outMax>)`

### Clip/clamp number ranges
`clamp(<in>, <min>, <max>);`

### Get min or max of two numbers
`partial_max` or `partial_min`

## Rust

### Define a custom type alias
```rust
type Line = (Point2, Point2);
```

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
