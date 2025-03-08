use perceptron::{Datapoint, Perceptron};

mod vec3;
mod perceptron;

fn main() {
    let datapoints: Vec<Datapoint> = vec![
        Datapoint((3, 1), 1),
        Datapoint((2, -1), 1),
        Datapoint((-2, 1), 1),
        Datapoint((-1, -3), -1),
    ];
    let mut perceptron = Perceptron::default();
    perceptron.set_datapoints(datapoints);

    loop {
        let incorrect = perceptron.update();
        println!("Finished update with {incorrect} incorrect.");
        if incorrect == 0 {
            break;
        }
    }
}
