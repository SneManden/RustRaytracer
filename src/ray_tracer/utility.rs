use std::f32::consts::PI;

pub fn deg_2_rad(deg: f32) -> f32 {
    PI / 180.0 * deg
}

pub fn rad_2_deg(rad: f32) -> f32 {
    rad * 180.0 / PI
}

