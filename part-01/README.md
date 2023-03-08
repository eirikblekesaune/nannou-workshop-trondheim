# Part 1 - Shapes, colors, and positions

In this first part of the workshop we will look at how to set up a basic `nannou` app, where we will draw simple geometric shapes.
We will draw the shapes in different color, move them around on the screen, and create composite graphic elements.

After this part you will have a basic understanding of the following:
* Minimal setup for `nannou` app.
* Draw a colored background
* Draw primitive shapes like triangles, ellipses, rectangles, lines, polygons.
* Specify and control the positions and colors for the shapes.
* Use mouse input to control positions and colors.
* Specify relative positions for groups of shapes.
* Animate parameters for the shapes relative to time.

## 1.1 Minimal `nannou` setup
There are two types of `nannou` setups: _sketches_ and _apps_.

* _sketches_ is the simplest to set up and is mostly used for quick or simple sketches where you don't need much state management, interaction with MIDI, audio etc.
* _apps_ are what you would use for a more full fledged application. This is the type of setup we will use for this workshop.

For more explanations of the difference between _apps_ and _sketches_ you can read the `nannou` guide chapter [Basics - Sketch vs App](https://guide.nannou.cc/tutorials/basics/sketch-vs-app.html).
The nannou repo has templates for [_sketches_](https://github.com/nannou-org/nannou/blob/master/examples/templates/template_sketch.rs) and [_apps_](https://github.com/nannou-org/nannou/blob/master/examples/templates/template_app.rs), if you need a barebones setup to start with.

As mentioned above, we will use _apps_ for this workshop.
We will also based our explorations on the code in this repo, so no need to set up your own `nannou` project for the work that we will do today.
If you want to set up your own `nannou` project later, the `nannou`-guide has a [chapter](https://guide.nannou.cc/getting_started/create_a_project.html) about it.
If you for some reason find yourself yearning more to explore `nannou` on your own, I can suggest reading and running [examples](https://guide.nannou.cc/getting_started/running_examples.html) in the `nannou` repo.

Our basic app is in the [basic-app/src/main.rs] file, where I have added some (admittedly verbose) comments about the structure and the syntax, to help you understand how those element connect together.
We will quickly go through the choreography of the program, so that you get an idea of how a basic app runs from start to finish.

You can run this `basic_app` using this command in the terminal:
```
cargo run -p basic-app
```

### Exercises 1.1
We are now going to develop the `basic-app` furter in some exerices.

#### Exercise 1.1.A - Make the circle bounce up and down
You can decide how high and low the ball will bounce.
Tip: You can use the app time to move the circle.

<details><summary> 🙈 Spoiler alert! A possible solution: </sumamry>

```rust
    draw.ellipse()
        .y(app.time.sin() * 200.0 )
        .color(MAGENTA);
```

</details>

#### Exercise 1.1.B - Change the circle bounce 
Make the circle bounce all the way from the top of the screen to the bottom.
Tip: You can use the data from the app window to get the screen height.
Tip: The `map_range()` function in `nannou` makes it really easy to convert from one number range to another:
```rust
let a = 0.5;
let b = map_range(a, 0.0, 1.0, 10.0, 20.0); // => 15.0
```


