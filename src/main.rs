use nannou::prelude::*;

mod ball;
use crate::ball::Ball;

fn main() {
    nannou::app(model)
        .size(1280, 720)
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    ball: Ball,
}

fn model(app: &App) -> Model {
    let _window = app.
                            new_window()
                            .title("Gravity")
                            .view(view)
                            .build()
                            .unwrap();
    let win = app.window_rect();

    //Custom Code
    let ball = Ball::new(win.x(), win.y(), 0.0, 0.0, 10.0, 100.0, BLACK);

    
    Model { _window, ball }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //Custom Code
    _model.ball.logic(_app, _update);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    //Custom Code
    _model.ball.display(&draw);
    draw.to_frame(app, &frame).unwrap();
}