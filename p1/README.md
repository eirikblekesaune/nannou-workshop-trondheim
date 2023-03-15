# Part 1 - Shapes, Rusty colors, and positions

In this part we will start by looking at more more Rust-specific topics, before we develop the graphics further.
When working with this part we will start seeing some of the aspects to the Rust programming language that makes it stand out from many other programming languages used in the industry today.
In the beginning of the exercises we won't be seeing any change with graphics, but the code...my oh my... that's gonna get some sweet improvements.

If you already are comfortable with Rust concepts like `struct`, `impl`, defining your own functions etc.  please feel free to jump ahead to the [next part](/p2/README.md).

## Learning goals
When we are done with this part of the workshop, you will have a basic understanding of these topics:
* Structure data using using `struct`
* Separate model-update and window-drawing into separate functions.
* Mutability of variables and function arguments.
* Implement your own functions, `fn`.
* Add behaviour to the circles by implementing function on a `struct` using `impl`.

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
This is based on the suggestion from _Exercise 0.F_.

To run the program we are working on you can run:

```rust
cargo run -p p1
```

Looking at the code we see that we can structure our code better.
When you start getting variables that contain numbers or single letters, e.g. `circle_radius_a` and `circle_radius_b`, it is often an indicator that you can start to gather data into separate data entities.
This would later make it easier for us to create and modify arbitrary numbers of circles.
Which of course is always a [good thing](https://xkcd.com/974/).
In Rust we can use `struct` to consolidate data.

Based on the code in `p1/src/main.rs` we are going to do the following:
* Define a `struct` called `Circle` that has data members for its `position`, `speed`, `size`, and `color`.
* Initialize two instances of this struct at the startup of our program.
* Modify the data for these circles through our `Model` instance, once per frame.
* Refer to these structs in our `view()` function in order to draw the circles to the window.

Oooohhhh , that's a lot on our plate😅.

Best to tackle this in smaller chunks, step by step.
We'll go through this one together, but if you are eager to just jump into it without any support wheels or anything, just go ahead and do that.

You can compare your own solution with the suggestion in [`p1/src/suggestion/p1_e.rs`](/p1/suggestion/p1_e.rs).
To run the solution suggestion you can execute this command:
```rust
cargo run --bin p1_a
```

### Exercise 1.A - Structure the things

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


<details><summary>🙈  Suggestion: </summary>

```rust
struct Circle {
    position: Vec2, // this type stores 2 dimensional positions
    speed: f32, // 32-bit float
    radius: f32,
    color: Rgb<u8>, //A color type with 3 8 bit values. One for each color Red, Green and Blue
}
```

</details>
<br/>

### Exercise 1.B - Add the circles to our <code>Model</code> definition
<details><summary>🙈 Suggestion: Change our <code>Model</code> </summary>
We have to change our <code>Model</code> which is currently empty, by adding our circles to it.

```rust
//Use the Model struct to hold the data that we need during the duration of our program
struct Model {
    circle_a: Circle,
    circle_b: Circle,
}
```

</details>
<br/>

### Exercise 1.C - Initialize our <code>Model</code>
<details><summary>💡 Initialize our <code>Model</code> </summary>

Initializing the state takes place in our <code>model</code> function.

</details>
<br/>

<details><summary>🙈 Suggestion: </summary>

```rust
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
        speed: a.speed * 2.0,
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
```

</details>
<br/>

### Exercise 1.D - Update our `Model` each frame
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

<details><summary>🙈 Suggestion: </summary>

```rust
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
    let radius = circle.radius;
    circle.position = vec2(
        map_range( ts.sin(), -1.0, 1.0, r.left() + radius, r.right() - radius ),
        map_range( ts.cos(), -1.0, 1.0, r.bottom() + radius, r.top() - radius ),
        );

    let circle = &mut model.circle_b;
    let ts = time * circle.speed;
    let radius = circle.radius;
    circle.position = vec2(
        map_range( ts.sin(), -1.0, 1.0, r.left() + radius, r.right() - radius ),
        map_range( ts.cos(), -1.0, 1.0, r.bottom() + radius, r.top() - radius ),
        );
}
```

</details>
<br/>


### Exercise 1.E - Drawing our updated circles
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

<details><summary>🙈 Suggestion: </summary>

```rust
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
```

</details>
<br/>

---

Alright! We did it! We are happy programmers.
We have:
* 🎉 Mastered the art of making simple structs
* 🎉 Initialized our data in the `model` function.
* 🎉 Updated our data in the `update` function.
* 🎉 Learned how to get a mutable reference to a struct's data member.


### Exercise 1-F - Calculate the speed using a function
Define your own function that calculates the doubled value for a decimal number, and use this function to calculate the speed of circle `b`.
Circle `b` will move twice as fast as circle `a`.
I suggest using the function name `doubled` for this, and make it accept an `f32` parameter and return an `f32` value which is double in value of the input argument.

<details><summary>🙈 Suggestion: </summary>

```rust 
// Our doubling function which doubles the f32 value you give it
fn double_it(x: f32) -> f32 {
    x * 2.0
}
//[...snip...]
  fn model(app: &App) -> Model {
      //[...snip...]
      let b = Circle {
          position: vec2(r.right() - (a.radius / 2.0), 0.0),
          speed: double_it(a.speed), // replace this with a call to our new favorite function
          radius: a.radius / 2.0,
          color: ORANGE,
      };
      //[...snip...]
  }
```

</details>
<br/>

🎉 You made your own custom function!

### Exercise 1-G - Give circles shrink and grow control
We want to control the radius of our circles using a function for each circle.
The syntax for our circle control should look like this:

```rust
circle_a.grow(1.0); // increase radius by 1.0
circle_a.shrink(1.0); // decrease radius by 1.0
```

<details><summary>🙈 Suggestion: </summary>

```rust 
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
```

And let's add the shrink and grow method calls to the <code>update</code> function so that the circle slowly change size over time.

```rust
    let circle = &mut model.circle_a;
    let ts = time * circle.speed;
    circle.grow(0.02); // grow every so little each frame
    let radius = circle.radius;
    circle.position = vec2(
        map_range( ts.sin(), -1.0, 1.0, r.left() + radius, r.right() - radius ),
        map_range( ts.cos(), -1.0, 1.0, r.bottom() + radius, r.top() - radius ),
        );

    let circle = &mut model.circle_b;
    let ts = time * circle.speed;
    circle.shrink(0.01); // shrink ever so ever so litle each frame
    let radius = circle.radius;
    circle.position = vec2(
        map_range( ts.sin(), -1.0, 1.0, r.left() + radius, r.right() - radius ),
        map_range( ts.cos(), -1.0, 1.0, r.bottom() + radius, r.top() - radius ),
        );
```

</details>
<br/>
🎉 You made your own custom function that mutates the state of your circles.

### Exercise 1-H: Grow and shrink cycle
We want the circles to grow and shrink between half the initial size and double the initial size.

### Exercise 1-I - Make a draw function for the item
We want to draw our circles using their own `draw()` function so in the view function we just do:

```rust
model.circle_a.draw(/*some args here*/);
model.circle_b.draw(/*some args here*/);
```

### Exercise 1-J - Gather all circles in a `Vec` and update and draw them in loops
If we want to add and remove circle dynamically in our program it is quite cumbersome to have variable naming in the format `circle_a`, `circle_b` etc.
We rather want to put all circles in a `Vec` of `Circle`.
Then both the update and the view function will have to change significantly.







