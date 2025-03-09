use crate::plot::Plot;

use super::vec3::Vec3;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Datapoint {
    pub pos: (i32, i32),
    pub label: i32,
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
    /// The index in `datapoints` that caused the most recent weight update.
    last_update_index: Option<usize>,
    /// How many weight updates have been made in this generation of
    /// updates. A generation is an iteration through all the datapoints.
    updates_this_generation: usize,
    /// Whether or not the Perceptron has gone through an entire
    /// generation without needing to update its weights.
    has_converged: bool,
}

impl Perceptron {
    pub fn new(datapoints: Vec<Datapoint>, weights: (i32, i32, i32)) -> Self {
        Perceptron {
            datapoints,
            weights: Vec3(weights.0 as f64, weights.1 as f64, weights.2 as f64),
            curr_index: 0,
            updates_this_generation: 0,
            last_update_index: None,
            has_converged: false,
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
                    let x = Vec3(1.0, point.pos.0 as f64, point.pos.1 as f64);
                    let y = point.label as f64;
                    let w_dot_x = self.weights.dot(&x);
                    if y * w_dot_x <= 0.0 {
                        self.weights += y * x;
                        self.updates_this_generation += 1;
                        self.last_update_index = Some(self.curr_index);
                        self.curr_index += 1;
                        return;
                    }
                    self.curr_index += 1;
                }
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

    fn get_point_color(&self, point: &Datapoint, index: usize) -> Color {
        let is_last_update = !self.has_converged && self.last_update_index == Some(index);
        if point.label <= 0 {
            if is_last_update {
                PURPLE
            } else {
                DARKPURPLE
            }
        } else {
            if is_last_update {
                GREEN
            } else {
                DARKGREEN
            }
        }
    }

    /// Return the weights.
    ///
    /// Note that due to the nature of the learning algorithm and the fact that
    /// all data points are integers, this means that the weights will always
    /// be integers.
    pub fn weights(&self) -> (i32, i32, i32) {
        (
            self.weights.0 as i32,
            self.weights.1 as i32,
            self.weights.2 as i32,
        )
    }

    pub fn draw(&self, plot: &Plot) {
        // Draw datapoints.
        for (index, point) in self.datapoints.iter().enumerate() {
            plot.draw_circle(
                point.pos.0 as f32,
                point.pos.1 as f32,
                0.5,
                self.get_point_color(point, index),
            );
        }

        // Draw weights, as a line dividing the space in half.
        let line = get_weight_line(&self.weights);

        if let Some((x1, y1, x2, y2)) = line {
            plot.draw_line(x1, y1, x2, y2, BLUE);
        }
    }
}

/// Returns the geometric representation the given weights
/// as a line with coordinates (x1, y1, x2, y2).
///
/// If the weights don't represent a vector, returns None.
///
/// Technically, this is actually the line *perpendicular*
/// to the weight vector, but conceptually it's the line
/// that divides the space in half.
fn get_weight_line(weights: &Vec3) -> Option<(f32, f32, f32, f32)> {
    if weights.2 != 0.0 {
        let f = |x: f32| -> f32 {
            // Derived by solving for `y` with `0 = w1 * x + w2 * y + w0`, since
            // the line is defined as all (x, y) such that the weighted sum with
            // bias is zero.
            ((-weights.0 - weights.1 * x as f64) / weights.2) as f32
        };
        let x1 = -1000.0;
        let y1 = f(x1);
        let x2 = 1000.0;
        let y2 = f(x2);
        Some((x1, y1, x2, y2))
    } else if weights.1 != 0.0 {
        // Derived similar to the previous case, but since we know `w2` is
        // zero, we can remove that entire term and solve for `x`.
        let x = (-weights.0 / weights.1) as f32;
        Some((x, -1000.0, x, 1000.0))
    } else {
        // The weights represent a directionless vector, there's no line
        // to draw.
        None
    }
}
