use std::ops::{Add, Div, Mul, Sub};

use nannou::prelude::*;

pub struct Ball {
    pub position: Point2,
    pub velocity: Point2,

    pub radius: f32,
    pub mass: f32,
    pub color: Srgb<u8>,
}

impl Ball {
    pub fn new(pos_x: f32, pos_y: f32, vel_x: f32, vel_y: f32, radius: f32, mass: f32, color: Srgb<u8>) -> Self {
        let position = pt2(pos_x, pos_y);
        let velocity = pt2(vel_x, vel_y);
        Ball {position, velocity, radius, mass, color}
    }

    pub fn logic(&mut self, _app: &App, _update: Update) {
        self.velocity = self.velocity.add(_app.mouse.position().sub(self.position).div(self.mass));
        self.position = self.position.add(self.velocity.mul(_update.since_last.as_secs_f32()));
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(self.color);
    }
}