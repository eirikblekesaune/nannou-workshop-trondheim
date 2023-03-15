# Part 2 - Kaleidoscopic Beziers

In this part we won't be as stepwise as in the previous part.
This time we will be looking at a specific technique for generating lines.
We take these lines and spread around the center to create a kaleidoscopic effect.

This time we will start with the code in `p2/src/main.rs` and work from there.

## Draw a Bezier line
A significant part of Nannou is a collection of other useful crates.
Some of these crates are imported into Nannou, e.g. through the `nannou::prelude::*` statement we have seen.

Other crates may need to be specially imported, which is the case with some of the things from the `lyon`-crate, the API we are using to draw _bezier_ curves in this part.

The view function is the most relevant for this part of the workshop, as we won't be using the `model` or `update` function now.
We could just as well made this into a [sketch](https://guide.nannou.cc/tutorials/basics/sketch-vs-app.html), but I'll leave that decision up to you.

```rust
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

    draw.to_frame(app, &frame).unwrap();
}
```

Running `cargo run --bin p2` shows us the nice wavy line.

## 2.B - Mirroring the line

We humans are often suckers for symmetry, so lets mirror the mirror the line around the center axis.
We can use the `Drawing::scale_x` for this, and give a negative value:

```rust
    //Add another line with the path, just with the draw context scaled with a negative value
    // effetively mirroring it
    draw.scale_x(-1.0)
        .path()
        .stroke()
        .color(WHITE)
        .weight(1.0)
        .events(path.iter());
```

Running `cargo run --bin p2_b` shows our mirrored wonder.

## 2.C - Spread out
A defining feature for kaleidoscopic effects is that they are spread around a center point in even angles.
We set up a loop for 12 arms around the center, and scale the draw context by half.

```rust
    //pull the variable out in case we want to change the color
    // as I'm pretty sure we going to do soon.
    let color = WHITE;
    const NUM_ARMS: u16 = 12; //Make a constant value

    let draw = draw.scale(0.5); // scale down the draw context the draw in half the size
    //Iterate over the numbers 0 to 11.
    //The 0..NUM_ARMS is an non-inclusive range.
    //If you wanted the numbers 0 to 12 you would write 0..=NUM_ARMS
    // for an inclusive range.
    for i in  0..NUM_ARMS {
        let angle = i as f32 * (360.0 / NUM_ARMS as f32);
        draw.rotate(angle.deg_to_rad()) // rotate the drawing context
            .translate(vec3(0.0, win.h() * 0.5, 0.0)) //move it to the center of the screen
            .path()
            .stroke()
            .color(color)
            .weight(1.0)
            .events(path.iter());

        draw.rotate(angle.deg_to_rad())
            .translate(vec3(0.0, win.h() * 0.5, 0.0))
            .scale_x(-1.0) //mirrored
            .path()
            .stroke()
            .color(color)
            .weight(1.0)
            .events(path.iter());
    }

```

Running `cargo run --bin p2_c` shows our kaleidoscopic arms dancing in all 12 directions.

## 2.D - Creating veils
To get a sense of depth or slow movement a common trick is to paint a ver transparent rectangle over the whole image on each update.
This keeps the history of past drawing longer, and let them fade out over time.

A common bug that people run into using nannou is that our `draw.background().color()` won't work for transparent colors.
We will have to use a rectangle for this:

```rust
fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    //Replace the `background()` with this to be able to use transparent colors
    draw.rect()
        .x_y(0.0, 0.0) 
        .w_h(win.w(), win.h()) //fill the whole window
        .color(rgba(0.0, 0.0, 0.0, 0.01)); //Very transparent black color
    //[...snip...]
}
```

Running `cargo run --bin p2_d` shows our veiled arms swaying

## 2.E - Adding colors
Colors in Nannou is not only your regular RGB Hex codes.
The color API that Nannou uses is based on the `palette` crate.

The [Color section](/cheat-sheet.md#colors) has more info about colors in Nannou.

We will use the color `Lch` for our sketch, since it gives a consistent luminance, as it a polar color space, meaning we can change the hue, chroma, luminance, instead of rgb.
This gives us a more intutive control of the colors.

We have to import to be able to useit

```rust
use nannou::color::Lch;
```

And we can for example use it like this.

```rust
    let hue = (app.time * 0.05).cos() * 0.5 + 0.5; // slow sine oscillator
    let hue = (hue * 360.0 - 180.0) * 10.0; //scale oscillator values
    let color = Lch::new( 
        90.0, // luminance in percent
        10.0, // chroma, quite little saturation for the colors
        hue // hue in debgrees from -180.0 - 180.0
        );
```

Running `cargo run --bin p2_e` shows our colored veils.

## 2.F - More complex color behaviour

Looking at our current results I feel that the colors should be more dynamic, and less RGB fadey (i.e. less like the cheap LED strips you set on auto mode).

(Shout-out to the shader folks out there), the _smooth step function_ can give a bit more control over the curvature of transition between values.
I want to combine this with a stepped function profile, so that the colors sometimes don't change, and when they change they ... change.
Sooo, instead of continually changing the colors, we change the changes of the colors.

The function we use for this look like this:

```rust
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
```

Now the color changing code is this:

```rust
    let hue = multismoothstep(9, (app.time * 0.05).cos() * 0.5 + 0.5, 0.7);
    let hue = (hue * 360.0 - 180.0) * 10.0;
    let color = Lch::new( 
        90.0,
        10.0,
        hue
        );
```

Running `cargo run --bin p2_f` shows our colored veils with a bit more life in the colors.
Creating at times mother-of-pearls-like color sheens.

---

So there you have it:
* 🎉 Drawing bezier curves
* 🎉 Drawing paths
* 🎉 Using other color spaces
* 🎉 Mirroring and angular distribution of lines.
* 🎉 Smoothstepping and multismoothstepping values.


