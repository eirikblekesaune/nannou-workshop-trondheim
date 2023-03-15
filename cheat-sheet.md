# Rust and Nannou cheat sheet
This document gathers snippets of code that illustrates many of the more common things you may want to do in Rust or Nannou.

Feel free to submit additional code snippet and cheats to this document. :smile:


## Nannou
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
Nannou base their color API on the [palette](https://crates.io/crates/palette) crate, but adds a layer of specifying whether we use 8-bit or 32-bit floats for the RGB values.
You can set a 8-bit color in the RGB color space with `rgb8(0,0,0)`, and 32-bit floats with: `rgb(0.0,0.0,0.0)`.

If you want to set the transparency you can add an `a` to the function names: `rgba8(0,0,0,255)` / `rgba8(0.0,0.0,1.0,1.0)`.

You can also define color using other color spaces with the functions listed in the Nannou::color [docfile](https://docs.rs/nannou/0.18.1/nannou/color/index.html#functions).

```rust
let a = hsv()
```

### Color constants
In a lot of the Nannou code you will see you will se that color constants are used, e.g. `BLUE`, `BEIGE` etc.
You can find a list of these color constant [here](https://docs.rs/nannou/0.18.1/nannou/color/index.html#constants).

### Color contant type mismatch errors
The colors you get from color constants are 8-bit colors, so if you want to modify them with other color you may sometimes get a compiler complaint that the types don't match.
One example is this code:

```rust
use nannou::color::Blend;
let a = BLUE;
let b = lin_srgb(1.0, 0.0, 0.0);
let c = b.overlay(a); //blending the colors using overlay blend method
```

Which would give an error similar to this:

```
error[E0308]: mismatched types
   --> mything/src/main.rs:82:23
    |
82  |     let c = b.overlay(a); //blending the colors using overlay blend method
    |               ------- ^ expected `Rgb<Linear<Srgb>, {float}>`, found `Rgb<Srgb, u8>`
    |               |
    |               arguments to this method are incorrect
    |
    = note: expected struct `nannou::prelude::rgb::Rgb<nannou::color::encoding::Linear<Srgb>, {float}>`
               found struct `nannou::prelude::rgb::Rgb<Srgb, u8>`
help: the return type of this call is `nannou::prelude::rgb::Rgb<Srgb, u8>` due to the type of the argument passed
```

The compiler error tells us that we have mismatched our types.
Looking at the type signature for the `overlay` function: `fn overlay(self, other: Self) -> Self`, we see the `other` argument must have the same type as the callee.
But in our code the type of `a` is an 8-bit color (e.g. `Rgb<Srgb, u8>`), whereas `b` has type `Rgb<Linear<Srgb>, f64>`.
So we need to convert `a` _both_ to a decimal type _and_ to a linear color space.
So we'll use the `into_format()` and `into_linear()` functions for this:

```rust
use nannou::color::Blend;
// let a: Rgb<f32> = BLUE.into_format(); //converting to f32 color specifically
let a = BLUE.into_format(); //or Rust can convert to f32 color by type inferrence
let a = a.into_linear(); // blending has to take place in a linear color space
let b = lin_srgb(1.0, 0.0, 0.0);
let c = a.overlay(b); //blending the colors using overlay blend method
```

### Modify colors
Lightening or darkening, and changing and saturation of colors is supported by the `nannou::color` API.
But it has some Rust specific peculiarities to make it work.
The `palette` crate has a design flow where you are encouraged to decode the color to a linear space, before you modify the colors.
The primary reason for this is explained in this [video](https://www.youtube.com/watch?v=LKnqECcg6Gw)

```rust
use nannou::color::Shade; // import the Shade trait which is needed for the `darken`/`lighten` functions
let c = rgb(1.0, 0.3, 0.2).into_linear(); //color manipulation is best done in a linear color
                                          //space, which into_linear() fixes for us.
let ca = c.darken(0.1);
let cb = c.lighten(0.1);

use nannou::color::Saturate; // needed for the `darken`/`lighten` functions
let d: Hsv = c.into(); // Convert from RGB to HSV color space, since RGB color spaces doesn't
                       // directly support saturation
let da = d.saturate(0.1);
let db = d.desaturate(0.1);

let e = BLUE
    .into_format() //Since color constant are defined as 8-bit color, we use into_format() to
                   //convert to 32-bit float type
    .into_linear();
let ea = e.darken(0.1);
let eb = e.lighten(0.1);
```

### Change the color hue
Too rotate the hue for a color, the color type needs to implement the `Hue` trait.
To see which colors implement this trait in Nannou you can look under the `Implementors` header in the [docfile](https://docs.rs/nannou/0.18.1/nannou/color/trait.Hue.html#implementors)

```rust
    use nannou::color::Hue; // import to be able to shift and set the hue
    let f = Lch::new(180.0, 10.0, 50.0);
    let g = hsl(0.5, 0.8, 0.2);
    let h = hsv(0.5, 0.8, 0.2);
    use nannou::color::Hwb;
    let i = Hwb::new(0.5, 0.8, 0.2);

    //We can rotate the hue for all these color spaces
    let fa = f.shift_hue(180.0);
    let ga = g.shift_hue(180.0);
    let ha = h.shift_hue(180.0);
    let ia = i.shift_hue(180.0);
```

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
