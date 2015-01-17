use KalmanFilter1D;
use std::rand::thread_rng;
use std::rand::distributions::{Normal, IndependentSample};

#[test]
fn kalman_filter_1d() {
    // This test case is based on Example 4.2-1
    // from the book "Applied Optimal Estimation"
    //
    // System model:
    // x(k+1) = x(k)
    // Measurement model:
    // z(k) = x(k) + v(k)
    // where
    // v(k) ~ N(0, r)
    //
    // Assumptions:
    // x(0) = 0
    // var(0) = 1
    // r = 1
    let state = 5.0f64;
    let variance = 1.0f64;
    let r = 1.5f64;
    let normal = Normal::new(0.0, r);
    let mut rng = thread_rng();

    let mut kf = KalmanFilter1D::new(state, variance);
    for i in range(1, 10u) {
        let measurement = state + normal.ind_sample(&mut rng);
        let denominator = 1.0 + variance / r * i as f64;
        let exp_state = kf.state
                        + ((variance / r) / denominator)
                        * (measurement - kf.state);
        let exp_variance = variance / denominator;
        kf.estimate(measurement, 1.0, 0.0, 1.0, r);
        assert_almost_eq!(kf.state, exp_state, 1e-6);
        assert_almost_eq!(kf.variance, exp_variance, 1e-6);
    }
}
