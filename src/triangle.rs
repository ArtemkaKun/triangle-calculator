use crate::point::Point;
use crate::draw_window::{init_vulkan};

pub(crate) struct Triangle {
    pub(crate) a: Point,
    pub(crate) b: Point,
    pub(crate) c: Point,
    pub(crate) ac_length: f32,
    pub(crate) ab_length: f32,
    pub(crate) cb_length: f32,
}

impl Triangle {
    pub(crate) fn calc_b_point(&self) -> Point {
        let pow_ab_distance = self.ab_length.powf(2.0);
        let a_p2_distance = (pow_ab_distance - self.cb_length.powf(2.0) + self.ac_length.powf(2.0)) / (2.0 * self.ac_length);
        let triangle_height = (pow_ab_distance - a_p2_distance.powf(2.0)).sqrt();

        Point {
            x: a_p2_distance,
            y: triangle_height,
        }
    }

    pub(crate) fn normalize_triangle(&mut self) {
        while self.ac_length > 1.0 || self.ab_length > 1.0 || self.cb_length > 1.0 {
            self.ac_length /= 2.0;
            self.ab_length /= 2.0;
            self.cb_length /= 2.0;
        }
    }

    pub(crate) fn draw(&self) {
        init_vulkan(&self);
    }
}