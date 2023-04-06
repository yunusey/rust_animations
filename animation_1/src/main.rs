#![allow(unused)]
use macroquad::prelude::*;

struct Scale { // In reality, what we want is not to move the screen which would be much harder.
               // Instead, we want to move each object with a (x, y):
               // For an object with coordinates (x, y) and a scale of (s_x, s_y);
               // object.cord = (x + s_x, y + s_y).
    x: f32, y: f32,
}

trait Shape {
    fn update(&mut self, other: &Pool);
    fn bottom_point(&self) -> (f32, f32);
    fn draw(&self, scale: &Scale);
}

#[derive(Debug)]
struct Circle {
    radius: f32,
    x: f32, y: f32,
    vel: f32, acc: f32,
    color: Color,
}

impl Circle {
    fn new(_radius: f32, _x: f32, _y: f32,  _vel: f32, _acc: f32, _color: Color) -> Self {
        Self { radius: _radius, x: _x, y: _y, vel: _vel, acc: _acc, color: _color }
    }
}

impl Shape for Circle {
    fn bottom_point(&self) -> (f32, f32) {
        (self.x, self.y + self.radius)
    }

    fn update(&mut self, pool: &Pool) {
        if pool.collides(&self) {
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

struct Pool {
    x1: f32, x2: f32, y: f32, c: DynamicColor,
}

impl Pool {
    fn new(_x1: f32, _x2: f32, _y: f32, _c: DynamicColor) -> Self {
        Self { x1: _x1, x2: _x2, y: _y, c: _c}
    }

    fn collides(&self, circle: &Circle) -> bool {
        let bot_point: (f32, f32) = circle.bottom_point();
        self.x1 <= bot_point.0 && bot_point.0 <= self.x2 && bot_point.1 >= self.y
    }

    fn update(&mut self) {
        self.c.update();
    }

    fn draw(&self, scale: &Scale) {
        draw_line(self.x1 + scale.x, self.y + scale.y, 
                  self.x2 + scale.x, self.y + scale.y, 
                  10.0, self.c.get_color());
    }
}

struct DynamicColor {
    c: Color,
    r: f32, g: f32, b: f32, a: f32,
}

impl DynamicColor {
    fn new(_c: Color, _r: f32, _g: f32, _b: f32, _a: f32) -> Self {
        Self { c: _c, r: _r, g: _g, b: _b, a: _a }
    }

    fn update(&mut self) {
        self.c.r += self.r;
        self.c.g += self.g;
        self.c.b += self.b;
        self.c.a += self.a;
    }

    fn get_color(&self) -> Color {
        self.c
    }
}

#[macroquad::main("Animation #1")]
async fn main() {

    let width = screen_width();
    let height = screen_height();

    let mut circlem: Circle = Circle::new(15.0, width / 2.0 - 50.0,  height / 2.0 - 750.0, 0.05, 0.005, RED);
    let mut circle1: Circle = Circle::new(15.0, width / 2.0 + 50.0,  height / 2.0 - 550.0, 0.05, 0.005, BLUE);
    let mut circle2: Circle = Circle::new(15.0, width / 2.0 + 150.0, height / 2.0 - 350.0, 0.05, 0.005, GREEN);
    let mut circle3: Circle = Circle::new(15.0, width / 2.0 + 250.0, height / 2.0 - 100.0, 0.05, 0.005, RED);
    let mut pool:  Pool = Pool::new(width / 2.0, width - 100.0, height / 2.0, 
                                    DynamicColor::new(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, 0.0, 0.0, 0.0, 0.0));

    let x_diff: f32 = circle3.x - circlem.x;
    let y_diff: f32 = circle3.y - pool.y;
    let loopsn: i32 = 1000;
    let x_inc: f32 = x_diff / loopsn as f32;
    let y_inc: f32 = y_diff / loopsn as f32;
    let mut loops_gone: i32 = 0;

    let white_inc: f32 = 1.0 / loopsn as f32;

    let mut npool: Pool = Pool::new(width / 2.0, width - 100.0, height / 2.0, 
                                    DynamicColor::new(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }, white_inc, white_inc, white_inc, white_inc));

    loop {
        clear_background(BLACK);

        if loops_gone == loopsn {
            circlem = Circle::new(15.0, width / 2.0 - 50.0,  height / 2.0 - 750.0, 0.05, 0.005, RED);
            circle1 = Circle::new(15.0, width / 2.0 + 50.0,  height / 2.0 - 550.0, 0.05, 0.005, BLUE);
            circle2 = Circle::new(15.0, width / 2.0 + 150.0, height / 2.0 - 350.0, 0.05, 0.005, GREEN);
            circle3 = Circle::new(15.0, width / 2.0 + 250.0, height / 2.0 - 100.0, 0.05, 0.005, RED);
            pool = Pool::new(width / 2.0, width - 100.0, height / 2.0, 
                             DynamicColor::new(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, 0.0, 0.0, 0.0, 0.0));
            npool = Pool::new(width / 2.0, width - 100.0, height / 2.0, 
                              DynamicColor::new(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }, white_inc, white_inc, white_inc, white_inc));
            loops_gone = 0;
        }

        circlem.update(&pool);
        circle1.update(&pool);
        circle2.update(&pool);
        circle3.update(&pool);
        pool.update();

        let mut scale: Scale = Scale { x: 0.0, y: 0.0 };

        if circlem.y >= pool.y {
            // Now that we managed to set move the screen according to main circle's cord,
            // we need to get it to the same starting location with circle1!
            // To do so, we need to find the differences between the x-cords and y-cords.
            // let's assume they are named x_d and y_d. The scale needs to be scaled according to
            // this ratio... It needs to be done such a way that, when main circle's x coordinate
            // equals to circle1's, their y cords will be equal to each other as well.
            // scale = Scale { x: 0.0, y: -(circlem.y - pool.y) };

            let s_x: f32 = x_inc * loops_gone as f32;
            let s_y: f32 = -(circlem.y - pool.y) + y_inc * loops_gone as f32;
            scale = Scale { x: s_x, y: s_y };
            loops_gone += 1;

            npool.update();
            npool.draw(&Scale { x: 0.0, y: 0.0 });

        }

        circlem.draw(&scale);
        circle1.draw(&scale);
        circle2.draw(&scale);
        circle3.draw(&scale);
        pool.draw(&scale);

        next_frame().await;
    }
}
