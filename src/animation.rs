use sdl2::pixels::Color;

#[derive(Debug, Copy, Clone)]
pub struct Animation {
    pub time_running: u32,
    pub step_duration: u32,
    pub duration: u32,
    pub color: Color
}

impl Animation { 
    pub fn new(step_duration: u32, duration: u32, color: Color) -> Animation {
        Animation { time_running: 0, step_duration: step_duration, duration: duration, color: color }
    }

    pub fn update(&mut self, time_delta: u32) {
        self.time_running += time_delta;
    }

    pub fn step(&self) -> u32 {
        self.time_running / self.step_duration
    }

    pub fn is_expired(&self) -> bool {
        return self.time_running > self.duration;
    }
}




