use super::vec3::Vec3;

pub struct Datapoint(pub (i32, i32), pub i32);

#[derive(Default)]
pub struct Perceptron {
    datapoints: Vec<Datapoint>,
    weights: Vec3
}

impl Perceptron {
    pub fn set_datapoints(&mut self, datapoints: Vec<Datapoint>) {
        self.datapoints = datapoints;
    }

    pub fn update(&mut self) -> i32 {
        let mut incorrect = 0;
        for point in &self.datapoints {
            let x = Vec3(1.0, point.0.0 as f64, point.0.1 as f64);
            let y = point.1 as f64;
            let w_dot_x = self.weights.dot(&x);
            if y * w_dot_x <= 0.0 {
                incorrect += 1;
                self.weights += y * x;
            }
        }
        incorrect
    }
}
