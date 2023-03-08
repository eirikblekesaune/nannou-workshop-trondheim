use nannou::prelude::*;

fn main() {
    nannou::sketch(view)
        .run();
}

fn view(_app: &App, frame: Frame){
    frame.clear(CYAN);
}

