mod vec3;

use vec3::Vec3;

#[derive(Copy, Clone)]
struct Datapoint {
    x: f64,
    y: f64,
    label: f64,
    name: &'static str,
}

const DATA_POINTS: [Datapoint; 4] = [
    Datapoint {
        name: "a",
        x: 3.0,
        y: 1.0,
        label: 1.0,
    },
    Datapoint {
        name: "b",
        x: 2.0,
        y: -1.0,
        label: 1.0,
    },
    Datapoint {
        name: "c",
        x: -2.0,
        y: 1.0,
        label: 1.0,
    },
    Datapoint {
        name: "d",
        x: -1.0,
        y: -3.0,
        label: -1.0,
    },
];

#[derive(Default)]
struct Perceptron {
    weights: Vec3
}

impl Perceptron {
    fn update(&mut self) -> i32 {
        let mut incorrect = 0;
        for point in &DATA_POINTS {
            let x = Vec3(1.0, point.x, point.y);
            let y = point.label;
            let w_dot_x = self.weights.dot(&x);
            if y * w_dot_x <= 0.0 {
                incorrect += 1;
                println!("{}: {}", point.name, y * w_dot_x);
                self.weights += y * x;
            }
        }
        incorrect
    }
}

fn main() {
    let mut perceptron = Perceptron::default();

    loop {
        let incorrect = perceptron.update();
        println!("Finished update with {incorrect} incorrect.");
        if incorrect == 0 {
            break;
        }
    }
}
