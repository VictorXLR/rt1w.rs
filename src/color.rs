use crate::vec3::Color;

pub fn write_color(x: Color) {
    println!(
        "{} {} {}",
        (255.999 * x.x()) as i64,
        (255.999 * x.y()) as i64,
        (255.999 * x.z()) as i64
    )
}
