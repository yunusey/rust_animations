use macroquad::prelude::*;
use crate::shapes::*;
use crate::shapes::Circle; // prelude has the same struct, therefore we are telling the compiler
                           // which one we actually want to use...
pub mod shapes;

#[macroquad::main("Animation #1")]
async fn main() {

    // let my_vec: Vec<Box<dyn Resettable>> = Vec::new();
    request_new_screen_size(600.0, 800.0);
    next_frame().await;

    let width = screen_width();
    let height = screen_height();

    let mut circlem: Circle = Circle::new(15.0, 
                                          width / 2.0 - 100.0,  height / 2.0 - 750.0, 
                                          0.05, 0.005, RED);
    let mut circle1: Circle = Circle::new(15.0, 
                                          width / 2.0 + 0.0,  height / 2.0 - 550.0, 
                                          0.05, 0.005, BLUE);
    let mut circle2: Circle = Circle::new(15.0, 
                                          width / 2.0 + 100.0, height / 2.0 - 350.0,
                                          0.05, 0.005, GREEN);
    let mut circle3: Circle = Circle::new(15.0, 
                                          width / 2.0 + 200.0, height / 2.0 - 100.0,
                                          0.05, 0.005, RED);
    let mut pool:  Pool = Pool::new(width / 2.0 - 50.0, width - 150.0, height / 2.0, 
                                    DynamicColor::new(
                                        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, 
                                        0.0, 0.0, 0.0, 0.0));

    let x_diff: f32 = circle3.x - circlem.x;
    let y_diff: f32 = circle3.y - pool.y;
    let loopsn: i32 = 1000;
    let x_inc: f32 = x_diff / loopsn as f32;
    let y_inc: f32 = y_diff / loopsn as f32;
    let mut loops_gone: i32 = 0;

    let white_inc: f32 = 1.0 / loopsn as f32;

    let mut npool: Pool = Pool::new(width / 2.0 - 50.0, width - 150.0, height / 2.0, 
                                    DynamicColor::new(
                                        Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }, 
                                        white_inc, white_inc, white_inc, white_inc));

    loop {
        clear_background(BLACK);

        if loops_gone == loopsn {

            circlem.reset();
            circle1.reset();
            circle2.reset();
            circle3.reset();
            pool.reset();
            npool.reset();

            loops_gone = 0;

        }

        circlem.update(&pool);
        circle1.update(&pool);
        circle2.update(&pool);
        circle3.update(&pool);
        pool.update();

        let mut scale: Scale = Scale { x: 0.0, y: 0.0 };

        if circlem.y >= pool.y {

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
