extern crate piston_window;
use piston_window::{PistonWindow, WindowSettings};

pub fn create_window(title: &str, width: f64, height: f64) -> PistonWindow {
    WindowSettings::new(title, [width, height])
        .exit_on_esc(true).build().unwrap()
}

pub fn shadow(color: [f32; 4], amount: f32) -> [f32; 4] {
    let max0: f64 = [0.0, (color[0] as f64) - (amount as f64)].iter().copied().fold(f64::NAN, f64::max);
    let max1: f64 = [0.0, (color[1] as f64) - (amount as f64)].iter().copied().fold(f64::NAN, f64::max);
    let max2: f64 = [0.0, (color[2] as f64) - (amount as f64)].iter().copied().fold(f64::NAN, f64::max);
    [max0 as f32, max1 as f32, max2 as f32, 1.0 as f32]
}
