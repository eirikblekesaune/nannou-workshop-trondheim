# Reading Rust docfiles

_This article is linked from [Exercise 0.G](Exercise 0.G - A building where the wild pixels live) as a tip on how to read the docfiles that `cargo doc` generates.
I turned out to be a long one, so I put it in a separate document._

Generally in Rust, documentation of crates are written in the source code itself.
By using doc comments, i.e. `///` triple forward slashes, documentation is written along with the code.
The documentation comments also support code sections using backticks, so code examples can be included in the source code.
After the documentation comments have been finished, you can run `cargo doc` to generate HTML documentation with links and the whole ordeal, bells and whistles.
See (this chapter in the Rust Book)[https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments] for more about Rust documentation feature.

## Automatic docs generation

So we get almost automatically generated documentation of our code.
That being said, the level of details for our docfiles can be daunting to some.
Not only do you get documentation of all functions, types, enums etc. for your code, you also get a lot of other stuff automatically included.
Stuff that you can't clearly see in the code documents themselves.

If you have the `rust-analyzer` installed for your VSCode, you will get helpful comment-looking bits of hint along with your code.
Looking at the returned value from e.g. `draw.ellipse()` you see that this method returns an instance of `Drawing<Ellipse>`.
The angle brackets are common in many other programming languages, and usually indicate something which is evaluated at _compile time_.
The data type `Drawing` is what is called a [_generic type_](https://doc.rust-lang.org/book/ch10-01-syntax.html), and in our case it is specialized on an `Ellipse` type.
So we can read the type `Drawing<Ellipse>` as 'a drawing of an ellipse in progress'.
We can further extrapolate that we can do certain things with this drawing-of-an-ellipse-in-progress.

For instance we can change its position, size, color and orientation.
Or we can specify whether we want the ellipse filled, stroked, or both.
But where are these methods defined?
Well, in the source code for `Drawing` obviously, where also the [documentation](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html) is written.

## Reading the signatures

Let us take the `.xy()` method we used to set the ellipse position as an example and look at its description in the docs under [this heading](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html#impl-3).
We see the method signature is seemingly uncomplicated.
It is a _public_ function (`pub`) that gets two arguments: `self` and a `Point2`, for which it returns a instance of the same type as itself, i.e. `Drawing`.

```rust
pub fn xy(self, p: Point2) -> Self`
```

If we look at the part right above, where it says `impl<'a, T>Drawing<'a, T>`, it easy to lose track of what going on here.
Without going into too much detail, the signature in question in a generic implementation over the `SetPosition` [_trait_](https://doc.rust-lang.org/book/ch10-02-traits.html) and it looks like this out in the wild (i.e. the source code):

```rust 
impl<'a, T> Drawing<'a, T>
where
    T: SetPosition + Into<Primitive>,
    Primitive: Into<Option<T>>,
{
    //[...snip...]

    /// Set the **Position** with some two-dimensional point.
    pub fn xy(self, p: Point2) -> Self {
        self.map_ty(|ty| SetPosition::xy(ty, p))
    }

    //[...snip...]
}
```

To fully understand all the concepts related to this is way out the scope for our workshop, but for the curious we could state in like this in more humanesque lingo:
This is is an implementation for the struct `Drawing`, for the _lifetime_ `'a` over the generic type `T`, where `T` implements the `SetPosition` trait and maybe be (hence the `Option`-part) be converted into (hence the `Into` part) a `Primitive`.
A `Primitive` is an _Enum_() which in our specific case is an instance of `Primitive::Ellipse()`.

...at least as humanesque as I can make it for now.

Feeble attemps to make this readable aside, how can I see what other methods are available for my `Drawing<Ellipse>`?

## Implemented traits on `Drawing`

You can see this by looking at which traits the `Drawing` struct implements.
If you scroll through the the `Drawing` docfile again you will see some other similar definitions as our trusty `SetPosition` thing.
Let me list them for you here:

- `SetPosition` is described [here](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html#impl-3)
- `SetDimensions` is described [here](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html#impl-2)
- `SetColor` is described [here](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html#impl-1)
- `SetFill` is described [here](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html#impl-5)
- `SetStroke` is desribed [here](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html#impl-6)
- `SetPolygon` is described [here](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html#impl-15)

## So what does this trait thingy actually mean?
Traits as a concept is not totally unique to Rust.
It can be compared to _interfaces_ in Java or TypeScript, _concepts_ in C++20, and _typeclasses_ in Haskell.

Simply stated, a trait says something about the _bahviour_ of an object.
In object oriented programming we have the concept of _inheritance_, i.e. that Ellipse _is-a_ Drawing.
Rust isn't an object oriented programming language, as for instance Java is.
But we can have _polymorphism_ in Rust through the traits mechanisms, which is a concepts derived from OOP.

So we can from this say: 'An Ellipse drawing is a thing that can be positioned, since it implements the `SetPosition` trait'
What other things can be position you may ask?

## See what implements a given trait
To see what other things implement for instance the `SetPosition` trait, we can click on the link in the docfile.
The link is also [here](https://docs.rs/nannou/0.18.1/nannou/draw/properties/spatial/position/trait.SetPosition.html) so that you don't have search for it.
Take a look under the `Implementors` heading.

Here you will find a list over other structs that implements the `SetPosition` trait, meaning all those things are things that can be positioned.

## Positioning the `Draw` instance
Looking through the list of Implementors for the `SetPosition` trait you may come to miss our goood old `Draw` thingy..
Why is that not there?
Surely, we just recently changed the position of the drawing context?

That there exists a trait called `SetPosition` does not mean that the things listed under the _Implementors_ heading are the _only_ things that can be positioned.
It only means that they implement a certain behaviour in a given situation (...I'm not going to repeat my previous attempt to humanesque whatever that thing was).

It turns out that `Draw` also implement `.xy()`, and with a very similar function signature:

```rust 
impl Draw {
  //[...snip...]

  pub fn xy(&self, v: Vec2) -> Self { /*...*/ }

  //[...snip...]
}
```

The `impl` part looks much simpler though.
It is _not_ implemented over a lifetime nor a generic type.

There is one small detail that is different in the function signature that is a forwarning to a future error.
So keep that in mind, as we soon will close this rather lenghty tip on how to read Rust docfile for Nannou, and finally find out how we can draw a rectangle, and what we ask rectangles to do in Nannou.

## Specific functions for specific Primitive types
As the Ellipse Drawing supports the general idea of begin _positioned_, it shares this behaviour with other Primitives, such as lines, rectangles, and triangles.
But there are certain things that only ellipses can be defined by, that other Primitives can't.
For our Ellipse it makes sense to set its _radius_, but this is not something that is applicable to a Line.

One example of such a _specialication_ is the description of the `stroke`, `radius`, and `resolution` methods [here](https://docs.rs/nannou/0.18.1/nannou/draw/struct.Drawing.html#impl-8).
If you click on the link to the source code, the `[src]` part on the left side of the page of the `impl<'a> Drawing<'a, Ellipse>`-part, you will maybe notice something surprising.

Notice that we are not in the source code file for `Drawing` anymore!
We are now in the source code file where `Ellipse` is defined, specifically in [this section](https://docs.rs/nannou/0.18.1/src/nannou/draw/primitive/ellipse.rs.html#170-188).
So even though we are looking at the docfile for what apparently is the `Drawing` file, the source code for some of the functions are defined in other files.

No wonder that certain functions shows up in the docfiles that we didn't specifically put there!

--- 

This concludes our deep dive into how to read and understand the Rust docfiles and how it relates to how the source code is written and structured.
Having auto-generated documentation of your Rust modules is a great asset both for yourself and the other people using your libraries.
But the docfiles that cargo generates doesn't hide any details so it can sometimes be a meticulous reading experience, although library authors vary on the readability of their docfiles.

Let us Return to (Exercise 0.G)[] to finally get to draw this rectangle.
