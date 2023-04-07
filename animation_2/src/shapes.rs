use macroquad::{shapes::{draw_triangle, draw_circle}, prelude::{WHITE, Vec2}};

pub trait Shapes {
    fn update(&mut self);
    fn draw(&self);
}

#[derive(Clone)]
pub struct Point {
    pub x: f32, pub y: f32
}

impl Point {
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

pub struct EqTriangle {
    pub point: Point,
    pub st_angle: f32, // starting angle
    pub angle: f32,
    pub a_chg: f32,
    pub len: f32,
}

impl EqTriangle {
    pub fn new(_p: Point, _angle: f32, _a_ch: f32, _len: f32) -> Self {
        Self { 
            point: _p, 
            st_angle: _angle, 
            angle: _angle, 
            a_chg: _a_ch, 
            len: _len 
        }
    }
    pub fn change_main_point(&mut self, other: &Point) {
        self.point = other.clone();
    }
}

impl Shapes for EqTriangle {
    fn update(&mut self) {
        self.angle += self.a_chg;
    }
    fn draw(&self) {
        let v2: f32 = self.len * self.angle.sin();
        let h2: f32 = self.len * self.angle.cos();
        let p2: Point = Point { x: self.point.x + h2, y: self.point.y - v2 };
        let v3: f32 = self.len * (self.angle + 60.0f32.to_radians()).sin();
        let h3: f32 = self.len * (self.angle + 60.0f32.to_radians()).cos();
        let p3: Point = Point { x: self.point.x + h3, y: self.point.y - v3 };

        draw_triangle(self.point.to_vec2(), p2.to_vec2(), p3.to_vec2(), WHITE);
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Shapes for Circle {
    fn update(&mut self) {
        // Nothing for now...
    }
    fn draw(&self) {
        draw_circle(self.center.x, self.center.y, self.radius, WHITE);
    }
}
