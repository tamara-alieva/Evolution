use std::process::Output;

#[derive(Debug)]
pub struct Network;

impl Network {  // Нейронная сеть
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

#[derive(Debug)]
struct Layer {  // Слой сети
    neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[derive(Debug)]
struct Neuron {         // Нейрон
    bias: f32,          // Смещение
    weights: Vec<f32>   // Весы
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let mut output = 0.0;

        for i in 0..inputs.len() {
            output += inputs[i] * self.weights[i];
        }

        output += self.bias;

        if output > 0.0 {
            output
        } else {
            0.0
        }
    }
}