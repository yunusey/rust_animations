use macroquad::prelude::*;

pub mod shapes;
use crate::shapes::*;
use crate::shapes::Circle;

#[macroquad::main("Animation #2")]
async fn main() {

    let width: f32 = screen_width();
    let height: f32 = screen_height();

    let mut triangles: Vec<EqTriangle> = Vec::new();
    let triangle_count: usize = 9;
    let mut angle: f32 = -30.0;
    let angle_change: f32 = 0.001;
    for _ in 0..triangle_count {
        let t: EqTriangle = EqTriangle::new( 
            Point { x: width / 2.0, y: height / 2.0 }, 
            angle.to_radians(), 
            angle_change,
            60.0,
            );
        triangles.push(t);
        angle += 360.0 / triangle_count as f32;
    }

    let mut c: Circle = Circle { 
        center: Point { x: width / 2.0, y: height / 2.0 }, 
        radius: 30.0 
    };

    let mut x: f32 = 0.0;

    let cloopspertloops = 1.0; // How many loops should a triangle get in the circle before it
                               // completes the loop around itself.
                               // 1 means windmill.
                               // 2 means ninja star
                               // .5 means slow ninja star
                               // These are just examples though
    loop {
        clear_background(BLACK);
        
        c.update();
        c.draw();

        for t in &mut triangles {
            let v: f32 = c.center.y - (cloopspertloops * x + t.st_angle + 30.0f32.to_radians()).sin() * c.radius;
            let h: f32 = c.center.x + (cloopspertloops * x + t.st_angle + 30.0f32.to_radians()).cos() * c.radius;
            t.change_main_point(&Point { x: h, y: v });
            t.update();
            t.draw();
        }

        x += angle_change;
        next_frame().await
    }
}
