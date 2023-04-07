// All the traits and structs related to shapes will be found here...

use macroquad::prelude::{Color, draw_circle, draw_line};

// For an object with coordinates (x, y) and a scale of (s_x, s_y);
// object.cord = (x + s_x, y + s_y).
pub trait Shape {
    fn update(&mut self, other: &Pool);
    fn bottom_point(&self) -> (f32, f32);
    fn draw(&self, scale: &Scale);
}

pub trait Resettable {
    fn reset(&mut self);
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Scale { 
    pub x: f32, pub y: f32,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Circle {
    pub radius: f32,
    pub x: f32, pub y: f32,
    pub vel: f32, pub acc: f32,
    pub color: Color,
    pub init: Option<Box<Circle>>,
}

impl Circle {
    pub fn new(_radius: f32, _x: f32, _y: f32, 
           _vel: f32, _acc: f32, _color: Color) -> Self {

        let init: Circle = Self { 
            radius: _radius, 
            x: _x, y: _y, 
            vel: _vel, acc: _acc, 
            color: _color, 
            init: None 
        };
        Self { 
            radius: _radius, 
            x: _x, y: _y, 
            vel: _vel, acc: _acc, 
            color: _color, 
            init: Some(Box::new(init)) 
        }
    }
}

impl Shape for Circle {
    fn bottom_point(&self) -> (f32, f32) {
        (self.x, self.y + self.radius)
    }

    fn update(&mut self, pool: &Pool) {
        if pool.collides(self) {
            self.vel *= -1.0
        }
        else {
            self.vel += self.acc;
        }
        self.y += self.vel;
    }

    fn draw(&self, scale: &Scale) {
        draw_circle(self.x + scale.x, self.y + scale.y, 
                    self.radius, self.color);
    }
}

impl Resettable for Circle {
    fn reset(&mut self) {
        let init = self.init.clone();
        *self = *self.init.clone().unwrap();
        self.init = init;
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Pool {
    pub x1: f32, pub x2: f32, pub y: f32, 
    pub c: DynamicColor,
    pub init: Option<Box<Pool>>,
}

impl Pool {
    pub fn new(_x1: f32, _x2: f32, _y: f32, _c: DynamicColor) -> Self {
        let init: Pool = Self { x1: _x1, x2: _x2, y: _y, 
            c: _c.clone(), 
            init: None 
        };
        Self { x1: _x1, x2: _x2, y: _y, 
            c: _c, 
            init: Some(Box::new(init)) 
        }
    }

    pub fn collides(&self, shape: &impl Shape) -> bool {
        let bot_point: (f32, f32) = shape.bottom_point();
        self.x1 <= bot_point.0 
            && bot_point.0 <= self.x2 
            && bot_point.1 >= self.y
    }

    pub fn update(&mut self) {
        self.c.update();
    }

    pub fn draw(&self, scale: &Scale) {
        draw_line(self.x1 + scale.x, self.y + scale.y, 
                  self.x2 + scale.x, self.y + scale.y, 
                  10.0, self.c.get_color());
    }
}

impl Resettable for Pool {
    fn reset(&mut self) {
        let init = self.init.clone();
        *self = *self.init.clone().unwrap();
        self.init = init;
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct DynamicColor {
    pub c: Color,
    pub r: f32, pub g: f32, pub b: f32, pub a: f32,
}

impl DynamicColor {
    pub fn new(_c: Color, _r: f32, _g: f32, _b: f32, _a: f32) -> Self {
        Self { c: _c, r: _r, g: _g, b: _b, a: _a }
    }

    pub fn update(&mut self) {
        self.c.r += self.r;
        self.c.g += self.g;
        self.c.b += self.b;
        self.c.a += self.a;
    }

    pub fn get_color(&self) -> Color {
        self.c
    }
}

