use macroquad::prelude::*;

use perceptron::{Datapoint, Perceptron};
use plot::Plot;

mod perceptron;
mod plot;
mod vec3;

#[macroquad::main("Perceptron Fun")]
async fn main() {
    let mut perceptron = Perceptron::new(vec![
        Datapoint::new((3, 1), -1),
        Datapoint::new((2, -1), 1),
        Datapoint::new((-2, 1), 1),
        Datapoint::new((-1, -3), -1),
    ]);

    const SCALE: f32 = 8.0;

    let plot = Plot::new(SCALE);
    let mut auto_update = false;

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::A) {
            auto_update = !auto_update;
        }

        if is_key_pressed(KeyCode::Space) || auto_update {
            perceptron.update();
        }

        let status = if perceptron.has_converged() {
            "Perceptron has converged."
        } else {
            if auto_update {
                "Auto-updating perceptron (press 'A' to stop)."
            } else {
                "Press space to update perceptron (press 'A' to auto-update)."
            }
        };
        draw_text(status, 0.0, 30.0, 30.0, WHITE);

        plot.draw_axes();
        perceptron.draw(&plot);
        next_frame().await;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
}
