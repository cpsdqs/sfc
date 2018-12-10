use std::f64::consts::PI;
use std::time::{Duration, Instant};

const MAX_TIME_INTERVAL: f64 = 1. / 60.;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub value: f64,
    pub velocity: f64,
    pub target: f64,
    pub force: f64,
    pub damping: f64,
}

impl Spring {
    pub fn new(damping: f64, response: f64) -> Spring {
        let force_sqrt = 2. * PI / response;
        let damping = damping * (2. * force_sqrt);
        let force = force_sqrt * force_sqrt;
        Self::with_force_damping(force, damping)
    }

    pub fn with_force_damping(force: f64, damping: f64) -> Spring {
        Spring {
            value: 0.,
            velocity: 0.,
            target: 0.,
            force,
            damping,
        }
    }

    pub fn update(&mut self, elapsed: Duration) {
        let mut time_left = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9;
        time_left = time_left.min(1.);
        while time_left > 0. {
            let dt = MAX_TIME_INTERVAL.min(time_left);
            let force = self.current_force();
            self.value += self.velocity * dt;
            self.velocity += force * dt;
            time_left -= dt;
        }
    }

    pub fn needs_update(&self, tolerance: f64) -> bool {
        (self.value - self.target).abs() > tolerance || self.velocity > tolerance
    }

    pub fn finish(&mut self) {
        self.value = self.target;
        self.velocity = 0.;
    }

    pub fn current_force(&self) -> f64 {
        -self.force * (self.value - self.target) - self.damping * self.velocity
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RealTimeSpring {
    pub spring: Spring,
    pub prev_update: Instant,
}

impl RealTimeSpring {
    pub fn new(spring: Spring) -> RealTimeSpring {
        RealTimeSpring {
            spring,
            prev_update: Instant::now(),
        }
    }

    pub fn update_time(&mut self) {
        self.prev_update = Instant::now();
    }

    pub fn update(&mut self) -> f64 {
        self.spring.update(self.prev_update.elapsed());
        self.update_time();
        self.spring.value
    }
}
