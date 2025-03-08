use super::vec3::Vec3;
use macroquad::prelude::*;

pub struct Datapoint(pub (i32, i32), pub i32);

#[derive(Default)]
pub struct Perceptron {
    datapoints: Vec<Datapoint>,
    weights: Vec3,
}

impl Perceptron {
    pub fn set_datapoints(&mut self, datapoints: Vec<Datapoint>) {
        self.datapoints = datapoints;
    }

    pub fn update(&mut self) -> i32 {
        let mut incorrect = 0;
        for point in &self.datapoints {
            let x = Vec3(1.0, point.0 .0 as f64, point.0 .1 as f64);
            let y = point.1 as f64;
            let w_dot_x = self.weights.dot(&x);
            if y * w_dot_x <= 0.0 {
                incorrect += 1;
                self.weights += y * x;
            }
        }
        incorrect
    }

    pub fn draw(&self) {
        const SCALE: f32 = 8.0;
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let screen_x = |x: f32| -> f32 { center_x + x * SCALE };
        let screen_y = |y: f32| -> f32 { center_y + y * SCALE };

        // Draw axes
        draw_line(0.0, center_y, screen_width(), center_y, 1.0, DARKGRAY);
        draw_line(center_x, 0.0, center_x, screen_height(), 1.0, DARKGRAY);

        // Draw points
        for point in &self.datapoints {
            draw_circle(
                screen_x(point.0 .0 as f32),
                screen_y(point.0 .1 as f32),
                SCALE / 2.0,
                if point.1 <= 0 { RED } else { GREEN },
            );
        }

        draw_text(
            &format!("Weights: {:?}", self.weights),
            0.0,
            screen_height() - 30.0,
            30.0,
            DARKBLUE,
        );

        // Draw weights, as a line dividing the space in half.
        if self.weights.2 != 0.0 {
            let f = |x: f32| -> f32 {
                // Derived by solving for `y` with `0 = w1 * x + w2 * y + w0`, since
                // the line is defined as all (x, y) such that the weighted sum with
                // bias is zero.
                ((-self.weights.0 - self.weights.1 * x as f64) / self.weights.2) as f32
            };
            let x1 = -1000.0;
            let x2 = 1000.0;
            draw_line(
                screen_x(x1),
                screen_y(f(x1)),
                screen_x(x2),
                screen_y(f(x2)),
                1.0,
                BLUE,
            );
        }
    }
}
