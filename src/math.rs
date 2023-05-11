use macroquad::prelude::Vec2;
use std::f32::consts::{FRAC_1_PI, PI};

/// Returns if the distance between pos_1 and pos_2 is less than min_distance
///
/// Note that the distance comparison is done on squared distances to avoid computing a square root.
pub fn closer_than(pos_1: Vec2, pos_2: Vec2, min_distance: f32) -> bool {
    let pos_1_to_2 = pos_2 - pos_1;
    let squared_distance = pos_1_to_2.dot(pos_1_to_2);
    let closer = squared_distance < min_distance * min_distance;
    closer
}

pub fn angle_to_cartesian(angle_degrees: f32) -> Vec2 {
    let angle_radians = 2.0 * PI * angle_degrees * (1.0 / 360.0);
    Vec2::new(angle_radians.cos(), angle_radians.sin())
}

#[allow(unused)]
pub fn cartesian_to_angle_degrees(vector: Vec2) -> f32 {
    f32::atan2(vector.y, vector.x) * 360.0 * 0.5 * FRAC_1_PI
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::{assert_float_eq, assert_vec2_eq};
    use std::f32::consts::FRAC_1_SQRT_2;

    #[test]
    fn test_angles() {
        assert_vec2_eq(angle_to_cartesian(0.0), Vec2::new(1.0, 0.0));
        assert_vec2_eq(
            angle_to_cartesian(45.0),
            Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        assert_vec2_eq(angle_to_cartesian(90.0), Vec2::new(0.0, 1.0));
        assert_vec2_eq(
            angle_to_cartesian(90.0 + 45.0),
            Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        );
        assert_vec2_eq(angle_to_cartesian(180.0), Vec2::new(-1.0, 0.0));
        assert_vec2_eq(
            angle_to_cartesian(180.0 + 45.0),
            Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        );
        assert_vec2_eq(angle_to_cartesian(270.0), Vec2::new(0.0, -1.0));
        assert_vec2_eq(angle_to_cartesian(-90.0), Vec2::new(0.0, -1.0));
        assert_vec2_eq(
            angle_to_cartesian(270.0 + 45.0),
            Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        );
        assert_vec2_eq(
            angle_to_cartesian(-45.0),
            Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        );
    }

    fn assert_angle_roundtrip(expected_angle: f32) {
        let cartesian = angle_to_cartesian(expected_angle);
        let angle = cartesian_to_angle_degrees(cartesian);
        assert_float_eq(angle, expected_angle);
    }

    #[test]
    fn test_angles_roundtrip() {
        assert_angle_roundtrip(0.0);
        assert_angle_roundtrip(40.0);
        assert_angle_roundtrip(45.0);
        assert_angle_roundtrip(50.0);
        assert_angle_roundtrip(85.0);
        assert_angle_roundtrip(90.0);
        assert_angle_roundtrip(95.0);
        assert_angle_roundtrip(90.0 + 40.0);
        assert_angle_roundtrip(90.0 + 45.0);
        assert_angle_roundtrip(90.0 + 50.0);
        assert_angle_roundtrip(175.0);
        assert_angle_roundtrip(179.0);
        assert_angle_roundtrip(-175.0);
        assert_angle_roundtrip(180.0 + 40.0 - 360.0);
        assert_angle_roundtrip(180.0 + 45.0 - 360.0);
        assert_angle_roundtrip(180.0 + 50.0 - 360.0);
        assert_angle_roundtrip(-90.0);
        assert_angle_roundtrip(-50.0);
        assert_angle_roundtrip(-45.0);
        assert_angle_roundtrip(-40.0);
    }
}
