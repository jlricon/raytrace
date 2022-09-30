pub fn degrees_to_radius(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
pub fn random_double() -> f64 {
    rand::random::<f64>()
}
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
