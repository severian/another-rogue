use sdl2::pixels::Color;

#[derive(Debug, Copy, Clone)]
pub struct Animation {
    pub start_time: u32,
    pub step_duration: u32,
    pub duration: u32,
    pub color: Color
}

impl Animation { 
    pub fn new(start_time: u32, step_duration: u32, duration: u32, color: Color) -> Animation {
        Animation { start_time: start_time, step_duration: step_duration, duration: duration, color: color }
    }

    pub fn step(&self, now: u32) -> u32 {
        (now - self.start_time) / self.step_duration
    }

    pub fn is_expired(&self, now: u32) -> bool {
        return now > self.start_time + self.duration;
    }
}




