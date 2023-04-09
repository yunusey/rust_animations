use macroquad::prelude::*;
use macroquad::text::*;

pub mod particle_system;
pub mod particle;
use particle_system::ParticleSystem;

fn my_text_draw (text: &str, font: &Result<Font, FontError>, x: f32, y: f32, color: Color) {
    match font {
        Ok(font) => {
            draw_text_ex(text, 
                         x, y, 
                         TextParams { 
                             font: font.clone(), font_size: 20, color, 
                             ..Default::default() 
                         });
        },
        Err(_) => {
            draw_text(text, x, y, 20.0, color);
        },
    }
}

#[macroquad::main("Particle System")]
async fn main() {

    request_new_screen_size(1400.0, 800.0);
    next_frame().await;

    let font_path = "/mnt/c/Users/yucel/Downloads/CodeNewRoman/Code New Roman Bold Nerd Font Complete Mono Windows Compatible.otf";
    // You should change here first!..
    let width: f32 = screen_width();
    let height: f32 = screen_height();

    let system_size: (Vec2, Vec2) = (vec2(30.0, 30.0), vec2(width - 410.0, height - 30.0));
    let num_balls: i32 = 100;

    let mut particle_system: ParticleSystem = ParticleSystem::new(system_size, num_balls, false);
    let font = load_ttf_font(font_path).await;

    loop {
        clear_background(BLACK);

        particle_system.update(get_frame_time());
        particle_system.draw();

        let (energy, momentum) = particle_system.info();
        my_text_draw(format!("Energy  : {energy}").as_str(), &font, 
                     width - 400.0, height / 2.0 - 10.0, YELLOW);
        my_text_draw(format!("Momentum: {momentum}").as_str(), &font, 
                     width - 400.0, height / 2.0 + 10.0, YELLOW);
        my_text_draw("**Precision mistakes are expected", &font, 
                     width - 400.0, height / 2.0 + 70.0, Color::new(1.0, 0.5, 0.5, 1.0));

        next_frame().await;
    }
}
