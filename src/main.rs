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
    ball_list: [Ball; 1],
}

fn model(app: &App) -> Model {
    let _window = app
                            .new_window()
                            .title("Gravity")
                            .view(view)
                            .build()
                            .unwrap();
    let win = app.window_rect().pad(15.0);

    //Custom Code
    let mut ball_list = [Ball::new(win.x(), win.y(), 0.0, 0.0, 15.0, 10.0, BLACK); 1];
    for i in 0..1 {
        ball_list[i] = Ball::new(win.right() - (i as f32 * 0.01), win.top() - (i as f32 * 0.01), 0.0, 0.0, 15.0, 1000.0, BLACK)
    }
    
    Model { _window, ball_list }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //Custom Code
    for i in 0..1 {
        _model.ball_list[i].logic(_app, _update)
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    //Custom Code
    for i in 0..1 {
        _model.ball_list[i].display(&draw, app)
    }


    draw.to_frame(app, &frame).unwrap();
}