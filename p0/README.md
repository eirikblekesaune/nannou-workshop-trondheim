# Part 0 - Basic Nannou app

In this first part of the workshop we will look at how to set up a basic Nannou app, where we will draw simple geometric shapes.
We will draw the shapes in different colors, move them around on the screen, and create composite graphic elements.

## Learning goals
After this part you will have a basic understanding of the following:
* Minimal setup for Nannou app.
* Draw a colored background
* Draw ellipses, rectangles, and triangles.
* Specify and control the positions and colors for the shapes.
* Understand the coordinate system for nannou; how position values correspond to location on the windows.
* Specify relative positions for groups of shapes.
* Animate parameters for the shapes relative to time.

## Minimal Nannou setup
There are two types of Nannou setups: _sketches_ and _apps_.

* _sketches_ is the simplest to set up and are mostly used for quick or simple experiements, where you don't need much state management, interaction with MIDI, audio etc.
* _apps_ is what you would use for a more full fledged application. This is the type of setup we will use for this workshop.

If you want more explanations of the differences between _apps_ and _sketches_ you can read the Nannou guide chapter [Basics - Sketch vs App](https://guide.nannou.cc/tutorials/basics/sketch-vs-app.html).
The nannou repo has templates for [_sketches_](https://github.com/nannou-org/nannou/blob/master/examples/templates/template_sketch.rs) and [_apps_](https://github.com/nannou-org/nannou/blob/master/examples/templates/template_app.rs), if you need a barebones setup to start with when you make your own project.

As mentioned above, we will use _apps_ for this workshop.
We will also base our explorations on the code in this repo, so no need to set up your own Nannou project for the work that we will do today.
If you want to set up your own Nannou project later, the Nannou-guide has a [chapter](https://guide.nannou.cc/getting_started/create_a_project.html) about it.
If you for some reason find yourself yearning more to explore Nannou on your own during the workshop, I can suggest reading and running [examples](https://guide.nannou.cc/getting_started/running_examples.html) in the `GitHub nannou` [repo](https://github.com/nannou-org/nannou).

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

<details><summary>💡 There is a Nannou function that makes it really easy to convert from one number range to another: </summary>

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
In this exercise we will change the circles `circle_a` and `circle_b` like this:

- Circle `circle_a` will move back and forth along the x axis, in the center of the window.
- Circle `circle_b` will circle around `circle_a`

<details><summary>💡 Changing the position of the drawing context makes drawing at relative positions much easier: </summary>
We have already seen that we can change the position of an ellipse using the <code>xy()</code> function.
Well, the same type of function can be used for the <code>draw</code> instance as well:

```rust
draw.xy(vec2(-100.0, 20.0)); // set the position using a Vec2
draw.x_y(-100.0, 20.0); // set the position using x and y as separate values
```

More info on this in the [cheat sheet](./cheat-sheet.md#move-scale-and-rotate-the-position-of-the-drawing-context)

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

---

Wouldn't it be cool if we had a small skyscraper with windows on the ground?

Let's add a building with 4 floors, two windows on each floor.

#### Exercise 0.G - A building where the pixels live

Start by making a rectangle for the building side.
A simple rectangle should be good.
How do we draw rectangles in Nannou you say?

<details><summary>💡 Let's see if the docs can help us...  </summary>
⚠️  Warning: Longer aside about reading the Rust docfiles <a href=https://github.com/eirikblekesaune/nannou-workshop-trondheim/blob/main/reading-rust-docfiles.md>here</a>⚠️

<br/>

It goes into detail on how you can navigate the documentation for nannou to find out what methods are defined for the different Primitives in Nannou.
If that is not interesting for you, feel free to jump ahead for the next tip that uncovers the magical methods for drawing rectangles.

</details>
<br/>

<details><summary>💡 How to draw a rectangle  </summary>
In the <code>Drawing</code> module we have the method <code>rect()</code> that returns a drawing-in-progress-of-a-rectangle, or <code>Drawing&ltRect&gt</code> as it called in Rust-speak.
The <code>Drawing&ltRect&gt</code> support a lot of interesting methods, so here is a short selection.
(If you want to dig deeper into how to find these methods in the documentation and source yourself, read the text linked to in the previous tip if you haven't done that already.)

```rust
.xy(); // set position using a Vec2
.x_y(); // set position using x and y as separate arguments
.wh(); // set the width and height using a Vec2
.w_h(); // set the width and height as separate argument
.rotate(); // rotate by radians
.rgb(); // color in RGB
.no_fill(); // don't fill the rectangle with a color
.stroke_color(); // set the color for the stroke
```

</details>
<br/>

Let's place the building rectangle at the center bottom of the window.

<details><summary>🙈 Here is one way to do it:  </summary>

```rust
//Draw the basic structure of the building
let unit_size = 50.0;
draw.rect()
    .x_y(0.0, win.bottom())
    .height(unit_size * 4.0) //4 floors
    .width(unit_size * 2.0) // 2 windows per floor
    .color(GREY);
```

</details>
<br/>

##### # 🪲 BUG ALERT! 🪲
Running the code above gives a strange result: The building moves along with `circle_a`.
A moving building...clearly that's not what we wanted.

Can you figure out why this happened?

<details><summary>💡 Here is a hint about what going on:  </summary>

What happened when we changed the position of the draw context using `.xy(pos_a)`?

Have a look at the method signature:

```rust
pub fn xy(&self, v: Vec2) -> Self
```

and the call site:

```rust
let draw = draw.xy(pos_a);
```

</details>
<br/>


<details><summary>💡 Here is another hint on how to fix it: </summary>
The result of `draw.xy(pos_a)` is assigned to our `draw` variable, which caused lasting change to the position of the drawing context.

We need to make this temporary.

</details>
<br/>


<details><summary>🙈  Here is a way to fix it: </summary>

```rust
pub fn xy(&self, v: Vec2) -> Self
```

The method signature for `.xy()` tells us that we call the object as a read-only _reference_, because of the first `&self` parameter.
This means that we cannot change the self object that the method is called on, but the `.xy()` method returns a new version with a new drawing context centered around `pos_a` in our case.
The `-> Self` part tells you that what gets returned from the function is an _owned type_, it has no `&`.
You can safely assume that a new version of the `Draw` type is returned, since there is no way for the original `Draw` instance to enter the function.
When we use this new version of `Draw` to assign to the `draw` variable, the old `draw` instance is replaced.

```rust
let draw = draw.xy(pos_a);
```

If we don't want to replace the existing `Draw` instance we can temporarily move the center to another position by calling the `.xy()` method inline, right before we start drawing our rectangle:

```rust
draw.xy(pos_a) // change the drawing context, this time temporarily
    .ellipse() // then start a drawing of an ellipse
    .xy(pos_b) // and set its position, relative to the draw context
    .radius(circle_radius_b)
    .color(ORANGE);

//Just a triangle to check that it draw smack in the center, i.e. it hasn't been
//  affected by changing the position of the drawing context in the rect drawing section above
draw.tri()
    .x_y(0.0, 0.0)
    .color(BLUE);
```

</details>
<br/>

##### 🪳 BUG ALERT!! 🪳
We have another bug in our code! 😱
You may have noticed that half of the building is outside the window.
This is because the position of the rectangle is defined in relation to its center.
So we could offset the position of the rectangle by half the height of the rectangle, but there is another more expressive way to place the rectangle: Using the [`nannou::geom::Rect`](https://docs.rs/nannou/0.18.1/nannou/geom/struct.Rect.html) module.
This module offers a lot of methods relating to describing the geometrical properties of a rectangle.


<details><summary>💡 but didn't we just use the `rect` thing for just drawing? surely, the `rect` thing can't do a lot of other fancy geomtrical tricks as well...? : </summary>

In programming, a general rule is that you try to keep your abstraction as well defined according to a given purpose as possible.
It would not be a good design if we designed a rectangle abstraction that defines all its geometrical properties as well as how it is going to draw itself on the screen, and send an email to your followers on social media while doing so.
That would be to break the [single-responsibility principle](https://en.wikipedia.org/wiki/single-responsibility_principle).

You can rest assured (but possibly confused) that the `rect` we use for drawing and `rect` used for geometrical properties are not the same.
They live in different places in the _namespace_ of nannou.

- The drawing type is `nannou::draw::primitive::rect::rect`
- The geometric type is `nannou::geom::rect`

The nodes in the namespace are separated by double colons, `::`.

</details>
<br/>


<details><summary>💡 Placing a `nannou::geom::Rect` </summary>
Here is a selection of [<code>nannou::geom::Rect</code>](https://docs.rs/nannou/0.18.1/nannou/geom/struct.Rect.html) methods:

```rust
use nannou::geom::Rect; //Bring it into current namespace so that only need `Rect`
let r = Rect::from_x_y_w_h(10.0, -100.0, 100.0, 100.0);
let r2 = Rect::from_w_h(50.0, 50.0).mid_top_of(r); // place second rect on the middle of the top of r
r2.wh(); //get the width and height as a Vec2
r2.xy(); //get the position as a Vec2
```

</details>
<br/>


<details><summary>🙈  Here is a way to fix the second bug, using `nannou::geom::Rect`: </summary>

```rust
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
```

</details>
<br/>

#### Exercise 0.H - Moon follower
Nearing the end of this part now.
Let's finish with a simpler one, just for fun.

Place a red circle inside the moon circle, (`circle_b`).
And let this be the last thing you draw in your `draw` function, right before drawing to frame.

<details><summary>💡 In case you are stuck, here is a hint: </summary>

The _global_ position of `circle_b` is a result of the _sum_ of the positions that went into placing it in the first place.

</details>
<br/>

<details><summary>🙈  Here is a simple way to do this: </summary>

```rust
draw.ellipse()
    .xy(pos_a + pos_b) // the sum of pos_a and pos_b is where the moon is
    .radius(10.0)
    .color(RED);

draw.to_frame(app, &frame).unwrap();
```

</details>
<br/>

---

That concludes the first part of this workshop.

Great job!

Let's take a small break and then continue on to [the second part](/p1/README.md).

