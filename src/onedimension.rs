#[derive(Show)]
pub struct KalmanFilter1D {
    pub init_state: f64,
    pub init_variance: f64,
}

impl KalmanFilter1D {
    pub fn new(init_state: f64, init_variance: f64) -> KalmanFilter1D {
        KalmanFilter1D {
            init_state: init_state,
            init_variance: init_variance,
        }
    }

    pub fn estimate(&mut self,
                    measurement: f64,
                    system_param: f64,
                    system_variance: f64,
                    measurement_param: f64,
                    measurement_variance: f64) -> f64 {
        // 1. State estimate extrapolation
        // 2. Error variance extrapolation
        // 3. Kalman gain
        // 4. State estimate update
        // 5. Error variance update
        1.0
    }
}

