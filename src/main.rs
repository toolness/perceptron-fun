use macroquad::prelude::*;

use perceptron::{Datapoint, Perceptron};
use plot::Plot;

mod perceptron;
mod plot;
mod vec3;

/// Each point on the plot is scaled by this many screen pixels.
const PLOT_SCALE: f32 = 8.0;

/// Time in seconds between perceptron auto-updates (when auto-updating
/// is enabled).
const AUTO_UPDATE_TIME: f64 = 0.25;

/// Empty space between left side of screen and text, in pixels.
const LEFT_PADDING: f32 = 10.0;

const HELP_TEXT: &'static str = r#"Help

H - Toggle help
SPACE - Update perceptron
A - Toggle auto-update mode
1 - Paint green datapoint (at mouse cursor)
2 - Paint purple datapoint (at mouse cursor)
X - Delete datapoint (at mouse cursor)
C - Clear all datapoints
"#;

#[macroquad::main("Perceptron Fun")]
async fn main() {
    let mut datapoints = vec![
        Datapoint::new((3, 1), 1),
        Datapoint::new((2, -1), 1),
        Datapoint::new((-2, 1), 1),
        Datapoint::new((-1, -3), -1),
    ];
    let mut perceptron = Perceptron::new(datapoints.clone());

    let plot = Plot::new(PLOT_SCALE);
    let mut auto_update = false;
    let mut show_help = false;
    let mut last_frame_time = get_time();
    let mut time_to_auto_update = AUTO_UPDATE_TIME;
    let help_lines: Vec<&'static str> = HELP_TEXT.split('\n').collect();

    loop {
        let now = get_time();
        let delta_time = now - last_frame_time;
        last_frame_time = now;

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

        if is_key_pressed(KeyCode::H) {
            show_help = !show_help;
        }

        if is_key_pressed(KeyCode::A) {
            auto_update = !auto_update;
            time_to_auto_update = 0.0;
        }

        let should_update = if auto_update {
            time_to_auto_update -= delta_time;
            if time_to_auto_update <= 0.0 {
                time_to_auto_update = AUTO_UPDATE_TIME;
                true
            } else {
                false
            }
        } else {
            is_key_pressed(KeyCode::Space)
        };

        if should_update {
            perceptron.update();
        }

        plot.draw_axes();
        plot.draw_circle(mouse.0 as f32, mouse.1 as f32, 0.75, DARKGRAY);

        perceptron.draw(&plot);

        let auto_label = if auto_update { "AUTO" } else { "" };
        let conv_label = if perceptron.has_converged() {
            "CONVERGENCE"
        } else {
            ""
        };
        draw_text(
            &format!(
                "Weights: {:3?} {auto_label:4} {conv_label:11}",
                perceptron.weights()
            ),
            LEFT_PADDING,
            screen_height() - 30.0,
            30.0,
            WHITE,
        );

        if show_help {
            for (index, line) in help_lines.iter().enumerate() {
                draw_text(
                    line,
                    LEFT_PADDING,
                    30.0 + (index as f32 * 30.0),
                    30.0,
                    WHITE,
                );
            }
        } else {
            draw_text("Press 'H' for help.", LEFT_PADDING, 30.0, 30.0, WHITE);
        }

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
