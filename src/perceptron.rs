use super::vec3::Vec3;
use macroquad::prelude::*;

pub struct Datapoint {
    pos: (i32, i32),
    label: i32,
}

impl Datapoint {
    pub fn new(pos: (i32, i32), label: i32) -> Self {
        Datapoint { pos, label }
    }
}

pub struct Perceptron {
    datapoints: Vec<Datapoint>,
    weights: Vec3,
    /// The current index in `datapoints` that we're looking at next.
    curr_index: usize,
    /// How many weight updates have been made in this generation of
    /// updates. A generation is an iteration through all the datapoints.
    updates_this_generation: usize,
    /// Whether or not the Perceptron has gone through an entire
    /// generation without needing to update its weights.
    has_converged: bool
}

impl Perceptron {
    pub fn new(datapoints: Vec<Datapoint>) -> Self {
        Perceptron {
            datapoints,
            weights: Default::default(),
            curr_index: 0,
            updates_this_generation: 0,
            has_converged: false
        }
    }

    /// Returns whether all future calls to `update` will do nothing.
    pub fn has_converged(&self) -> bool {
        self.has_converged
    }

    /// Try to make a single weight update to the Perceptron based on the
    /// next datapoint that the Perceptron doesn't classify correctly.
    ///
    /// Weights won't be updated if the Perceptron's solution has converged.
    pub fn update(&mut self) {
        // This is based on "Why Machines Learn" by Anil Ananthaswamy, pg. 51.
        loop {
            match self.datapoints.get(self.curr_index) {
                Some(point) => {
                    let x = Vec3(1.0, point.pos .0 as f64, point.pos .1 as f64);
                    let y = point.label as f64;
                    let w_dot_x = self.weights.dot(&x);
                    self.curr_index += 1;
                    if y * w_dot_x <= 0.0 {
                        self.weights += y * x;
                        self.updates_this_generation += 1;
                        return;
                    }
                },
                None => {
                    if self.updates_this_generation == 0 {
                        self.has_converged = true;
                    } else {
                        self.updates_this_generation = 0;
                        self.curr_index = 0;
                        self.update();
                    }
                    return;
                }
            }
        }
    }

    pub fn draw(&self) {
        const SCALE: f32 = 8.0;
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let screen_x = |x: f32| -> f32 { center_x + x * SCALE };
        let screen_y = |y: f32| -> f32 { center_y + y * SCALE };

        // Draw axes.
        draw_line(0.0, center_y, screen_width(), center_y, 1.0, DARKGRAY);
        draw_line(center_x, 0.0, center_x, screen_height(), 1.0, DARKGRAY);

        // Draw datapoints.
        for point in &self.datapoints {
            draw_circle(
                screen_x(point.pos .0 as f32),
                screen_y(point.pos .1 as f32),
                SCALE / 2.0,
                if point.label <= 0 { RED } else { GREEN },
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
        let line: Option<(f32, f32, f32, f32)>;

        if self.weights.2 != 0.0 {
            let f = |x: f32| -> f32 {
                // Derived by solving for `y` with `0 = w1 * x + w2 * y + w0`, since
                // the line is defined as all (x, y) such that the weighted sum with
                // bias is zero.
                ((-self.weights.0 - self.weights.1 * x as f64) / self.weights.2) as f32
            };
            let x1 = -1000.0;
            let y1 = f(x1);
            let x2 = 1000.0;
            let y2 = f(x2);
            line = Some((x1, y1, x2, y2));
        } else if self.weights.1 != 0.0 {
            // Derived similar to the previous case, but since we know `w2` is
            // zero, we can remove that entire term and solve for `x`.
            let x = (-self.weights.0 / self.weights.1) as f32;
            line = Some((x, -1000.0, x, 1000.0))
        } else {
            // The weights represent a directionless vector, there's no line
            // to draw.
            line = None
        }

        if let Some((x1, y1, x2, y2)) = line {
            draw_line(
                screen_x(x1),
                screen_y(y1),
                screen_x(x2),
                screen_y(y2),
                1.0,
                BLUE,
            );
        }
    }
}
