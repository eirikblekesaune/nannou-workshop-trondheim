# Rust and `nannou` cheat sheet
This document gathers snippets of code that illustrates many of the more common things you may want to do in Rust or `nannou`.

Feel free to submit additional code snippet and cheats to this document. :smile:


### Get window size
```rust
let r = app.window_rect(); // => Rect
r.w(); // the width
r.h(); // the height
r.len() // the longest side
```

### Convert number ranges
```
map_range(<in>, <inMin>, <inMax>, <outMin>, <outMax>)
```

### Clip/clamp number ranges
```
clamp(<in>, <min>, <max>);
```
