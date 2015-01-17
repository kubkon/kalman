#![crate_name = "kalman"]
#![experimental]

pub use onedimension::KalmanFilter1D;

mod onedimension;

#[macro_use]
mod macros;

#[cfg(test)]
mod test;
