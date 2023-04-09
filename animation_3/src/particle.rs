use macroquad::prelude::*;
use ::rand::prelude::*;

pub struct Particle {
    pub center: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
}

impl Particle {
    pub fn collide(&mut self, other: &mut Particle) -> bool {
        let dist: f32 = (self.center - other.center).length();
        let overlap: f32 = (self.radius + other.radius) - dist;
        if overlap >= 0.0 {
            // Dynamic collisions...
            let relative_vel: Vec2 = self.vel - other.vel; // Relative velocity
            let normal: Vec2 = (-relative_vel).normalize(); // Normal Vector
            let impulse: Vec2 = (2.0 * self.mass * other.mass) / (self.mass + other.mass)
                * (relative_vel.dot(normal))
                * normal;
            self.vel  -= impulse / self.mass;
            other.vel += impulse / other.mass;

            // Static collisions...
            let dx: f32 = -overlap / 2.0; // Displacement required...

            self.center  -= dx * (self.center - other.center) / dist;
            other.center += dx * (self.center - other.center) / dist;

            return true;
        }
        return false;
    }

    pub fn update(&mut self, dt: f32, topleft: Vec2, botright: Vec2) {
        self.center += self.vel * dt;
        if self.center.x - self.radius <= topleft.x {
            self.center.x = self.radius + topleft.x;
            self.vel.x = -self.vel.x;
        }
        if self.center.x + self.radius >= botright.x {
            self.center.x = botright.x - self.radius;
            self.vel.x = -self.vel.x;
        }
        if self.center.y - self.radius <= topleft.y {
            self.center.y = self.radius + topleft.y;
            self.vel.y = -self.vel.y;
        }
        if self.center.y + self.radius >= botright.y {
            self.center.y = botright.y - self.radius;
            self.vel.y = -self.vel.y;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.center.x, self.center.y, self.radius, self.color);
    }

    pub fn get_energy(&self) -> f32 {
        self.mass * self.vel.length_squared() * 0.5
    }

    pub fn get_momentum(&self) -> f32 {
        self.mass * self.vel.length()
    }

    pub fn change_to_a_random_color(&mut self) {
        self.color = Self::rand_color();
    }
}

// Creating balls...
impl Particle {
    fn rand_color() -> Color {
        let brightness: f32 = 100.0 / 255.0;
        let r: f32 = thread_rng().gen_range(0.0..1.0);
        let g: f32 = thread_rng().gen_range(0.0..1.0);
        let b: f32 = thread_rng().gen_range((brightness - r - g).max(0.0)..1.0);
        Color { r, g, b, a: 1.0 }
    }

    // Returns a random ball...
    pub fn random_ball(topleft: Vec2, botright: Vec2) -> Self {
        let x: f32 = thread_rng().gen_range((topleft.x + 100.0)..(botright.x - 100.0));
        let y: f32 = thread_rng().gen_range((topleft.y + 100.0)..(botright.y - 100.0));
        let m: f32 = thread_rng().gen_range(1.0..10.0);
        let velx: f32 = thread_rng().gen_range(-90.0..90.0);
        let vely: f32 = thread_rng().gen_range(-90.0..90.0);
        let col: Color = Self::rand_color();
        let rad: f32 = m + 10.0; // The more mass you have, the greater your radius is.
        Particle {
            center: Vec2 { x, y },
            vel: Vec2 { x: velx, y: vely },
            radius: rad,
            mass: m,
            color: col,
        }
    }
}

