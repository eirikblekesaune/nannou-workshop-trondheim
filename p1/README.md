# Part 1 - Shapes, colors, and positions

In this part we will start look more closely on more Rust specific topics.
Looking at the result we got in the previous part, we see that we can structure our code better.
When working with this part we will start seeing some of the aspects to the Rust programming language that makes it stand out from many other programming languages used in the industry today.

## Learning goals
When we are done with this part of the workshop, you will have a basic understanding of these topics:
* Structure a nannou app using `struct`
* Implement your own functions, `fn`.
* Implement function on a `struct` using `impl`
* Mutability of variables and function arguments.
* Ownership of data, and getting our acquaintance with our favorite beast: _The Borrow Checker_


## Programming, learning, and thinking in Rust
As we'll see soon, the strictness of the Rust compiler may seem daunting at first.
And it _is_ challenging to start learning Rust.
Many very experienced programmers learning Rust for the first time, have struggled in the beginning with the concepts and rules that the Rust compiler enforces.
So if you find yourself in having a hard time understanding all of this, you are certainly in good company.

But, there are good reasons why certain things are challenging in Rust.
Because some things in programming is really hard.
Making a computer program consists of making a lot of choices to solves the many problems that arise in implementing the functionality of your program.
Sometimes we want to--or simply have to--cut corners in order to meet a deadline.
Hopefully the consequences of these choices won't come back and bite ut later in the development process.
But with increasing complexity in systems, chances are that defects in the code will surface as the most mysterious bugs, faults, security issues.
Many of these defects are in a class of errors called _memory errors_, that can cause a program to access or write to areas of memory that wasn't intended.
Rust is by some called a _memory-safe language_, that makes it easier to avoid those types of errors.
The memory safety comes at a cost of a steep learning curve.
But when you finally get your Rust code to compile, you have a set of guarantees that certain types of defects won't exist in your code.

From this you have these points in the back of your mind when you program Rust:
* If something is really hard to do easily in Rust, that can be an indication that the approach you have chosen is not the best one.
  * For example if you struggle modifying a member of your data multiple places at the same time, chances are that this is something you generally don't want to do. Especially if you system is going to scale well, or processes will run in multiple threads at the same time.
* When the compiler and the borrow checker complains about something in your code, and you can't make heads or tails of what the error means, pause for a moment and ask yourself: Why doesn't the Rust compiler want me not to do the thing I am doing?
  * The Rust compiler wants to guarantee that your code will run fine (in certain aspect). Its complaints are well intended.
  * Chances are that if you understand _why_ the compiler complains, the correct solution will be easier to find.
* Again, learning Rust is challenging, so give yourself some slack and don't shy away from asking for help.
  * The [Rust community](https://www.rust-lang.org/community) is very helpful and friendly, and building an open and inclusive culture is an active effort from the people that come together to develop the project.


