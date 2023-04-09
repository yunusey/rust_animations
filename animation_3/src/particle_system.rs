use macroquad::prelude::*;
use crate::particle::Particle;

pub struct ParticleSystem {
    particles: Vec<Particle>,
    coord: (Vec2, Vec2),
    change_on_collision: bool,
}

// Main methods
impl ParticleSystem {
    pub fn new (coord: (Vec2, Vec2), num_balls: i32, change_on_collision: bool) -> ParticleSystem {
        let (topleft, botright) = coord;
        let mut particles: Vec<Particle> = Vec::new();
        for _ in 0..num_balls {
            let p = Particle::random_ball(topleft, botright);
            particles.push(p);
        }
        Self {
            particles,
            coord,
            change_on_collision,
        }
    }

    pub fn add_particle (&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update (&mut self, dt: f32) {

        for i in 0..self.particles.len() {
            let (a, b) = self.particles.split_at_mut(i + 1);
            let p1 = &mut a[a.len() - 1];
            for p2 in b {
                let collided: bool = p1.collide(p2);
                if collided && self.change_on_collision {
                    p1.change_to_a_random_color();
                    p2.change_to_a_random_color();
                }
            }
        }

        let (topleft, botright) = self.coord;
        for i in &mut self.particles {
            i.update(dt, topleft, botright);
        }
    }

    pub fn draw (&self) {
        for i in &self.particles {
            i.draw();
        }
        let (topleft, botright) = self.coord;
        let size = botright - topleft;
        draw_rectangle_lines(topleft.x, topleft.y, size.x, size.y, 3.0, WHITE);
    }
}

// Utils
impl ParticleSystem {
    pub fn get_energy (&self) -> f32 {
        let mut energy = 0.0;
        for i in &self.particles {
            energy += i.get_energy();
        }
        energy
    }

    pub fn get_momentum (&self) -> f32 {
        let mut momentum = 0.0;
        for i in &self.particles {
            momentum += i.get_momentum();
        }
        momentum
    }

    pub fn info(&self) -> (f32, f32) {
        (self.get_energy(), self.get_momentum())
    }
}
