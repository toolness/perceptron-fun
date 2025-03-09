use macroquad::prelude::*;

use perceptron::{Datapoint, Perceptron};
use plot::Plot;

mod perceptron;
mod plot;
mod vec3;

#[macroquad::main("Perceptron Fun")]
async fn main() {
    let mut datapoints = vec![
        Datapoint::new((3, 1), 1),
        Datapoint::new((2, -1), 1),
        Datapoint::new((-2, 1), 1),
        Datapoint::new((-1, -3), -1),
    ];
    let mut perceptron = Perceptron::new(datapoints.clone());

    const SCALE: f32 = 8.0;

    let plot = Plot::new(SCALE);
    let mut auto_update = false;

    loop {
        clear_background(BLACK);

        let mouse_f32 = plot.from_screen_point(mouse_position());
        let mouse = (mouse_f32.0.round() as i32, mouse_f32.1.round() as i32);

        let modified_datapoints = if is_key_down(KeyCode::Key1) {
            modify_datapoint(&mut datapoints, mouse, Some(1))
        } else if is_key_down(KeyCode::Key2) {
            modify_datapoint(&mut datapoints, mouse, Some(-1))
        } else if is_key_down(KeyCode::X) {
            modify_datapoint(&mut datapoints, mouse, None)
        } else if is_key_down(KeyCode::C) {
            datapoints = vec![];
            true
        } else {
            false
        };

        if modified_datapoints {
            perceptron = Perceptron::new(datapoints.clone());
        }

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
        plot.draw_circle(mouse.0 as f32, mouse.1 as f32, 0.75, DARKGRAY);

        perceptron.draw(&plot);

        draw_text(
            &format!("Weights: {:?}", perceptron.weights()),
            0.0,
            screen_height() - 30.0,
            30.0,
            DARKBLUE,
        );

        next_frame().await;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
}

/// Modifies the datapoint with the given point.
///
/// If the label is none, the datapoint is removed (if it exists).
///
/// Otherwise, the datapoint is modified to have the given label (or if it's not in
/// datapoints, it's added).
///
/// Returns whether the datapoints were changed.
fn modify_datapoint(
    datapoints: &mut Vec<Datapoint>,
    point: (i32, i32),
    label: Option<i32>,
) -> bool {
    if let Some(label) = label {
        if let Some(dp) = datapoints.iter_mut().find(|dp| dp.pos == point) {
            if dp.label != label {
                dp.label = label;
                return true;
            }
        } else {
            datapoints.push(Datapoint::new(point, label));
            return true;
        }
    } else {
        if let Some(pos) = datapoints.iter().position(|dp| dp.pos == point) {
            datapoints.remove(pos);
            return true;
        }
    }
    false
}
