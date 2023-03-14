# Part 1 - Shapes, Rusty colors, and positions

In this part we will start by looking at more more Rust-specific topics, before we develop the graphics further.
When working with this part we will start seeing some of the aspects to the Rust programming language that makes it stand out from many other programming languages used in the industry today.
In the beginning of the exercises we won't be seeing any change with graphics, but the code...my oh my... that's gonna get some sweet improvements.

## Learning goals
When we are done with this part of the workshop, you will have a basic understanding of these topics:
* Structure data using using `struct`
* Separate model-update and window-drawing into separate functions.
* Mutability of variables and function arguments.
* Implement your own functions, `fn`.
* Add behaviour to the circles by implementing function on a `struct` using `impl`.
* Draw other shapes: rectangles, lines, polygons, polylines.
* Describe colors in color spaces `rgb`, `hsv`, and `CIE L*a*b*`
* Ownership of data, and getting our acquaintance with our favorite beast: _The Borrow Checker_

## Programming, learning, and thinking in Rust
As we'll see soon, the strictness of the Rust compiler may seem daunting at first.
And it _is_ challenging to start learning Rust.
Many very experienced programmers learning Rust for the first time have struggled in the beginning with the concepts and rules that the Rust compiler enforces.
So if you find yourself in having a hard time understanding all of this, you are certainly in good company.

But, there are good reasons why certain things are challenging in Rust.
Because some things in programming is really hard.
Making a computer program consists of making a lot of choices to solves the many problems that arise in implementing the functionality of your program.
Sometimes we want to---or simply have to---cut corners in order to meet a deadline.
Hopefully the consequences of these choices won't come back and bite ut later in the development process.
But with increasing complexity in systems, chances are that defects in the code will surface as the most mysterious bugs, faults, and security issues.

Many of these defects are in a class of errors called _memory errors_, that can cause a program to access or write to areas of memory that wasn't intended.
This can be accessing an item outside the bounds of an array, or passing a reference to a part of memory that can be manipulated in such a way that you can extract secret information such as passwords or the like.

Rust is by some called a _memory-safe language_, that makes it easier to avoid those types of errors.
The memory safety does comes at a cost of a steep learning curve.
But when you finally get your Rust code to compile, and you have adhered to the compilers warnings and errors, you have a set of guarantees that certain types of defects won't exist in your code.
Moreover, these rules also makes it easier for the compiler to optimize your code better, so you can potentially end up with performance on par with programs writtens in `C`/`C++`.
That being said, this does not mean that anything that compiles with the Rust compiler won't ever have bugs, but some kinds of bugs will be harder to put into your program.


## Useful reminders
You can have these points in the back of your mind when you program Rust:
* If something is really hard to do easily in Rust, that can be an indication that the approach you have chosen is not the best one.
  * For example if you struggle modifying a member of your data multiple places at the same time, chances are that this is something you generally don't want to do. Especially if you system is going to scale well, or processes will run in multiple threads at the same time.
* When the compiler and the borrow checker complains about something in your code, and you can't make heads or tails of what the error means, pause for a moment and ask yourself: Why doesn't the Rust compiler want me not to do the thing I am doing?
  * The Rust compiler wants to guarantee that your code will run fine (in certain aspect). Its complaints are well intended.
  * Chances are that if you understand _why_ the compiler complains, the correct solution will be easier to find.
* Again, learning Rust is challenging, so give yourself some slack and don't shy away from asking for help.
  * The [Rust community](https://www.rust-lang.org/community) is very helpful and friendly. The people that come together to develop the Rust project take an active effort the fascilitate for an open and inclusive community.

## Exercises Part 1
For the exercises in this part we will use the file `p1/src/main.rs` as our starting point.
To run the program we are working on you can run:
```rust
cargo run -p p1
```

Looking at the result we got in the previous part, we see that we can structure our code better.
When you start getting variables that contain numbers or single letters, e.g. `circle_radius_a` and `circle_radius_b`, it is often an indicator that you can start to gather data into separate data entities.
This would later make it easier for us to create and modify arbitrary numbers of circles.
Which of course is always a [good thing](https://xkcd.com/974/).
In Rust we can use `struct` to consolidate data.

## Exercise 1-A - Structure it
Based on the code in `p1/src/main.rs` we are going to do the following:
* Define a `struct` called `Circle` that has data members for its `position`, `speed`, `size`, and `color`.
* Initialize two instances of this struct at the startup of our program.
* Modify the data for these circles through our `Model` instance, once per frame.
* Refer to these structs in our `view()` function in order to draw the circles to the window.

Oooohhhh , that's a lot on our plate😅.

Best to tackle this in smaller chunks, step by step.
We'll go through this one together, but if you are eager to just jump into it without any support wheels or anything, just go ahead and do that.

You can compare your own solution with the suggestion in [`p1/src/suggestion/p1_a.rs`](./p1/suggestion/p1_a.rs).

To run the solution suggestion you can execute this command:
```rust
cargo run --bin p1_a
```

### Define a `Circle` struct

<details><summary>💡 Define a struct with data member names and appropriate data types: </summary>

```rust
//Definition of a struct with a data member `power` that contains a signed 
// 32-bit number.
struct Widget {
  power: i32
}
```

</details>
<br/>

<details><summary>💡 Some useful data types: </summary>
Rust has two main categories of types: primitive types, and custom types.
You can tell them apart by looking at the case of the first letter; if the first letter is lowercase it's a primitive type.
The uppercase types are customly defined most often using <code>struct</code> or <code>enum</code> keywords.

```rust
struct Widget {
  a: i32, //a signed 32-bit number
  b: i64, //a signed 64-bit number
  c: u32, //an unsigned 32-bit number
  d: f32, //a 32-bit decimal number, float
  e: f64, //a 64-bit decimal number, double
  f: String, //a String to store e.g. text data
  g: Vec3, //a 3-dimensional vector, can also be used as position
  h: Vec2, //a 2-dimensional vector,
  i: Point2, //a synonym for Vec2, i.e. they are the same type
  j: Rgb<u8>, //One color type where the RGB values are 8-bit unsigned integers
  k: Rgb<f32>, //A color type where the RGB values are 32-bit floats
}
```

</details>
<br/>

### Add the circles to our <code>Model</code>
<details><summary>💡 Change our <code>Model</code> </summary>
We have to change our <code>Model</code> which is currently empty, by adding our circles to it.

```rust
struct Model {
    circle_a: Circle,
    circle_b: Circle,
}
```

</details>
<br/>

### Initialize our <code>Model</code>
<details><summary>💡 Initialize our <code>Model</code> </summary>
Initializing the state takes place in our <code>model</code> function.

```rust
let r = app.window_rect();
let radius_a = 50.0;
let a = Circle {
    position: vec2(r.right() - radius_a, 0.0),
    speed: 1.0,
    radius: radius_a,
    color: MAGENTA,
};
//[...some more code here]
Model {
    circle_a: a,
    circle_b: b,
}
```

</details>
<br/>

### Update our `Model` each frame
<details><summary>💡 To update our model: </summary>
We use the <code>update</code> function to update the data in our model.
Notice the function signature:

```rust
fn update(_app: &App, _model: &mut Model, _update: Update)
```

The `&mut` tells ut that we can mutate the contents of our `Model` instance.
Btw. the leading underscore in `_model` is just the programmers way to tell Rust that we are not going to use that parameter in our code, so that rust doesn't warn us about it.
But in this case we are actually going to use it... so...

We can access and modify the data in our model like this:

```rust
let circle = &mut model.circle_a;
circle.speed = 1.1;
```

Also here we need to tell Rust that we want a mutable reference.
Try to write it without the `&mut` part and see what happens.. I dare you.

</details>
<br/>

### Drawing our updated circles
<details><summary>💡 Draw our circles: </summary>
We use thew <code>view</code> function only for drawing thing to the window.
Notice the function signature:

```rust
fn update(app: &App, _model: &Model, frame: Frame)
```

We now longer have mutable access to the <code>Model</code> it is read-only,
But our drawing code becomes much simpler and cleaner now:

```rust
let circle = &model.circle_a;
draw.ellipse()
    .xy(circle.position)
    .radius(circle.radius)
    .color(circle.color);
```

</details>
<br/>


Alright! We did it! We are happy programmers.
We have:
* 🎉 Mastered the art of making simple structs
* 🎉 Initialized our data in the `model` function.
* 🎉 Updated our data in the `update` function.
* 🎉 Learned how to get a mutable reference to a struct's data member.
* 🎉 


* CHALLENGE: Mutability of struct members
  * Try first to move allocate
  * Then give it a mut declaration
  * Then assign it as a mutable reference.
* CHALLENGE: Code not DRY, repeating code.
* CHALLENGE: Does not scale well.

## Exercise 1-B - Calculate the speed
Define your own function that calculates the doubled value for a decimal number, and use this function to calculate the speed of circle `b`.
Circle `b` will move twice as fast as circle `a`.

## Exercise 1-C - Give a circle some speed control
We want to control the speed of our circles using a function for each circle.
The syntax for our circle control should look like this:
```rust
circle_a.set_speed(1.0);
```
We still want the speed of circle `b` to be twice that of circle `a`.

## Exercise 1-D: Construct those circles
We are not too fond of the syntax for initializing the starting positions for our circles.
Right now the circle structs are initialized inline.
It would be soo much cooler if we could make a function that creates and initializes our circles, so that we simply could do:
```rust
let a = Circle::new()
```

## Make a draw function for the item

## Make it a rotating circle using traits
Rotate a vector
Define and implement a trait





