use macroquad::prelude::*;

use perceptron::{Datapoint, Perceptron};

mod perceptron;
mod vec3;

#[macroquad::main("Perceptron Fun")]
async fn main() {
    let datapoints: Vec<Datapoint> = vec![
        Datapoint((3, 1), -1),
        Datapoint((2, -1), 1),
        Datapoint((-2, 1), 1),
        Datapoint((-1, -3), -1),
    ];
    let mut perceptron = Perceptron::default();
    perceptron.set_datapoints(datapoints);

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Tab) {
            perceptron.update();
        }

        draw_text("Press tab to update perceptron.", 0.0, 30.0, 30.0, WHITE);

        perceptron.draw();
        next_frame().await;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
}
