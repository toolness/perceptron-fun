use macroquad::prelude::*;

use perceptron::{Datapoint, Perceptron};

mod perceptron;
mod vec3;

#[macroquad::main("Perceptron Fun")]
async fn main() {
    let mut perceptron = Perceptron::new(
        vec![
            Datapoint::new((3, 1), -1),
            Datapoint::new((2, -1), 1),
            Datapoint::new((-2, 1), 1),
            Datapoint::new((-1, -3), -1),
        ]
    );

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            perceptron.update();
        }

        draw_text("Press space to update perceptron.", 0.0, 30.0, 30.0, WHITE);

        perceptron.draw();
        next_frame().await;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
    }
}
