use std::ops::{Add, Mul, Sub};

use nannou::prelude::*;

#[derive(Copy, Clone)]
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
        let mut point_vector = _app.mouse.position().sub(self.position);
        let win = _app.window_rect().pad(self.radius);
        
        // Fg = Gm1m2/r^2 * ^r
        point_vector = point_vector.with_magnitude((self.mass * 100.0) / point_vector.magnitude2());

        self.velocity = self.velocity.add(point_vector);
        
        self.position = self.position.add(self.velocity.mul(_update.since_last.as_secs_f32()));

        if self.position.x >= win.right() { self.velocity.x *= -1.0; self.position.x = win.right() }
        if self.position.x <= win.left() { self.velocity.x *= -1.0; self.position.x = win.left() }
        if self.position.y >= win.top() { self.velocity.y *= -1.0; self.position.y = win.top() }
        if self.position.y <= win.bottom() { self.velocity.y *= -1.0; self.position.y = win.bottom() }

    }

    pub fn display(&self, draw: &Draw, app: &App) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius)
            .color(self.color);

        draw.line()
            .start(self.position)
            .end(app.mouse.position());
    }
}