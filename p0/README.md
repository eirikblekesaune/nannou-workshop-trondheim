# Part 0 - Basic `nannou` app

In this first part of the workshop we will look at how to set up a basic `nannou` app, where we will draw simple geometric shapes.
We will draw the shapes in different colors, move them around on the screen, and create composite graphic elements.

## Learning goals
After this part you will have a basic understanding of the following:
* Minimal setup for `nannou` app.
* Draw a colored background
* Draw ellipses and triangles.
* Specify and control the positions and colors for the shapes.
* Understand the coordinate system for nannou; how position values correspond to location on the windows.
* Specify relative positions for groups of shapes.
* Animate parameters for the shapes relative to time.

## Minimal `nannou` setup
There are two types of `nannou` setups: _sketches_ and _apps_.

* _sketches_ is the simplest to set up and are mostly used for quick or simple experiements, where you don't need much state management, interaction with MIDI, audio etc.
* _apps_ is what you would use for a more full fledged application. This is the type of setup we will use for this workshop.

If you want more explanations of the differences between _apps_ and _sketches_ you can read the `nannou` guide chapter [Basics - Sketch vs App](https://guide.nannou.cc/tutorials/basics/sketch-vs-app.html).
The nannou repo has templates for [_sketches_](https://github.com/nannou-org/nannou/blob/master/examples/templates/template_sketch.rs) and [_apps_](https://github.com/nannou-org/nannou/blob/master/examples/templates/template_app.rs), if you need a barebones setup to start with when you make your own project.

As mentioned above, we will use _apps_ for this workshop.
We will also base our explorations on the code in this repo, so no need to set up your own `nannou` project for the work that we will do today.
If you want to set up your own `nannou` project later, the `nannou`-guide has a [chapter](https://guide.nannou.cc/getting_started/create_a_project.html) about it.
If you for some reason find yourself yearning more to explore `nannou` on your own during the workshop, I can suggest reading and running [examples](https://guide.nannou.cc/getting_started/running_examples.html) in the `GitHub nannou` [repo](https://github.com/nannou-org/nannou).

Our starting point is a basic app in the [p0-basic-app/src/main.rs] file, where I have added some (admittedly verbose) comments about the structure and the syntax, to help you understand how those element connect together.
We will quickly go through the choreography of the program, so that you get an idea of how a basic app runs from start to finish.

You can run this `p0-basic_app` using this command in the terminal:
```
cargo run -p p0-basic-app
```

### Exercises Part 0
We are now going to develop the `p0-basic-app` furter in some exercises.

---

#### Exercise 0.A - Make the circle move up and down
You can decide how high and low the ball will move.
<details> <summary>💡 Tip: You can use the app time to move the circle. </summary>

```rust
app.time;
```

</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
    draw.ellipse()
        .y(app.time.sin() * 200.0 )
        .color(MAGENTA);
```

</details>

---

#### Exercise 0.B - Change the circle movement
Make the circle move all the way from the top of the screen to the bottom.
<details> <summary>💡 Tip: You can use the data from the <code>App</code> instance to get the screen height. </summary>

```rust
let r = app.window_rect();
r.w(); //total width
r.h(); //total height
r.top(); //top of the window
r.botton(); //bottom of the window
r.right(); //right edge of the window
r.left(); //left edge of the window
```

</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
    //Get the window rect
    let r = app.window_rect();
    //The output range from the `sin()` function is -1.0 - 1.0
    //Since the window coordinates for nannou has x:0.0,y:0.0 as the center of the window,
    // converting the range -1.0 to 1.0 to the full height of the window is as simple as
    // multiplying with half the window height.
    let y_pos = app.time.sin() * r.h() * 0.5;
    //Use the draw instance to draw an ellipse.
    draw.ellipse(i)
        .y(y_pos)
        .color(MAGENTA);
```

</details>

---

#### Exercise 0.C - A smaller and more precisely positioned movement
Convert the movement range to having the top at _2:3_ of the window height, and the bottom at _1:4_ of the screen height.

<details><summary>💡 There is a <code>nannou</code> function that makes it really easy to convert from one number range to another: </summary>

```rust
let a = 0.5;
let b = map_range(a, 0.0, 1.0, 10.0, 20.0); // => 15.0
```

</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
    //Use the top and bottom values from the window rect to make it simpler to convert value
    // ranges.
    let y_top = map_range(2.0/3.0, 0.0, 1.0, r.bottom(), r.top());
    let y_bottom = map_range(1.0/4.0, 0.0, 1.0, r.bottom(), r.top());
    let y_pos = map_range(app.time.sin(), -1.0, 1.0, y_bottom, y_top);
```

</details>
<br/>

--- 

#### Exercise 0.D - Full circle rotation
Change the movement of the circle to do move in a circle around the center.
The circle should go to the edges of the window both on the left/right edges and the top/bottom edges.


<details><summary>💡 To travel in a circular motion we can combine <code>sin()</code> with its co(s)mpanion... </summary>

```rust
//We can combine `sin` and `cos` to create movement in circles.
draw.ellipse()
    .y(app.time.sin() * 200.0)
    .x(app.time.cos() * 200.0)
    .color(MAGENTA);
```

</details>
<br/>

---

#### Exercise 0.E - Double circle rotation
Add another circle to the drawings.
The new circle should be half the size and travel twice as fast.
Choose another nice color for the second circle.
Both circles should touch the edge of the screen, i.e. nothing of the circles should go outside the window.

💡 You can find function for setting circle/ellipse properties, and much more, in the [cheat sheet](/cheat-sheet.md#draw-circles-and-ellipses).


<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
    let circle_radius_a = 50.0;
    let pos_a = vec2(
        map_range(app.time.sin(), -1.0, 1.0, r.left() + circle_radius_a,   r.right() - circle_radius_a),
        map_range(app.time.cos(), -1.0, 1.0, r.bottom() + circle_radius_a, r.top() - circle_radius_a),
        );

    let circle_radius_b = circle_radius_a / 2.0;
    let pos_b = vec2(
        map_range((2.0 * app.time).sin(), -1.0, 1.0, r.left() + circle_radius_b,   r.right() - circle_radius_b ),
        map_range((2.0 * app.time).cos(), -1.0, 1.0, r.bottom() + circle_radius_b, r.top() - circle_radius_b ),
        );

    //Use the draw instance to draw an ellipse.
    draw.ellipse()
        .xy(pos_a)
        .radius(circle_radius_a)
        .color(MAGENTA);
    draw.ellipse()
        .xy(pos_b)
        .radius(circle_radius_b)
        .color(ORANGE);
```

</details>
<br/>

---
#### Exercise 0.F - Planet and a moon
We have two circles.
In this exercise we will change the circles `a` and `b` like this:

- Circle `a` will move back and forth along the x axis, in the center of the window.
- Circle `b` will circle around `a`

<details><summary>💡 Changing the position of the drawing context makes drawing at relative positions much easier: </summary>
We have already seen that we can change the position of an ellipse using the <code>xy()</code> function.
Well, the same type of function can be used for the `draw` instance as well:

```rust
draw.x_y(-100.0, 0.0);
```

More info on this in the [cheat sheet](https://github.com/eirikblekesaune/nannou-workshop-trondheim/blob/main/cheat-sheet.md#move-scale-and-rotate-the-position-of-the-drawing-context)

</details>
<br/>

<details><summary> 🙈 Spoiler alert! A possible solution: </summary>

```rust
let circle_radius_a = 50.0;
let pos_a = vec2( (app.time * 0.5).sin() * app.window_rect().w() * 0.25, 0.0);

draw.ellipse()
    .xy(pos_a)
    .radius(circle_radius_a)
    .color(MAGENTA);

//Instead of moving the next circle, we move the whole draw instance to a new position.
//This effectively moves the center for the drawing to the position of circle `a`, thus drawing
// circle `b` in relation to that point becomes much easier.
let draw = draw.xy(pos_a);

let circle_radius_b = circle_radius_a / 2.0;
let pos_b = vec2(
    (3.0 * app.time).sin() * 100.0,
    (3.0 * app.time).cos() * 100.0,
    );

draw.ellipse()
    .xy(pos_b)
    .radius(circle_radius_b)
    .color(ORANGE);
```

</details>
<br/>

