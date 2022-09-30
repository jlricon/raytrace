use crate::vec3::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    // Write the translated [0,255] value of each color component.
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;
    let scale = 1.0 / samples_per_pixel as f64;
    // Sqrt for gamma correction
    let clamped_r = (r * scale).sqrt().clamp(0.0, 0.999);
    let clamped_g = (g * scale).sqrt().clamp(0.0, 0.999);
    let clamped_b = (b * scale).sqrt().clamp(0.0, 0.999);

    println!(
        "{} {} {}",
        (256.0 * clamped_r) as i32,
        (256.0 * clamped_g) as i32,
        (256.0 * clamped_b) as i32
    )
}
