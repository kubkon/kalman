#[derive(Show)]
pub struct KalmanFilter1D {
    pub state: f64,
    pub variance: f64,
}

impl KalmanFilter1D {
    pub fn new(init_state: f64, init_variance: f64) -> KalmanFilter1D {
        KalmanFilter1D {
            state: init_state,
            variance: init_variance,
        }
    }

    pub fn estimate(&mut self,
                    measurement: f64,
                    system_param: f64,
                    system_variance: f64,
                    measurement_param: f64,
                    measurement_variance: f64) {
        // 1. State estimate extrapolation
        let state_estimate = system_param * self.state;
        // 2. Error variance extrapolation
        let var_estimate = system_param * system_param * self.variance + system_variance;
        // 3. Kalman gain
        let k = (var_estimate * measurement_param)
                / (measurement_param * measurement_param * var_estimate + measurement_variance);
        // 4. State estimate update
        self.state = state_estimate + k * (measurement - measurement_param * state_estimate);
        // 5. Error variance update
        self.variance = (1.0 - k * measurement_param) * var_estimate;
    }
}

