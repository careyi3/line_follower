pub struct PID {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    error_old: f32,
    integral: f32,
}

impl PID {
    pub fn new(kp: f32, ki: f32, kd: f32) -> PID {
        return PID {
            kp,
            ki,
            kd,
            error_old: 0.0,
            integral: 0.0,
        };
    }

    pub fn compute(&mut self, error: f32, dt: f32) -> f32 {
        let p = self.kp * error;

        self.integral = self.integral + (error * dt);
        let i = self.ki * self.integral;

        let d = self.kd * ((error - self.error_old) / dt);
        self.error_old = error;

        return p + i + d;
    }
}
