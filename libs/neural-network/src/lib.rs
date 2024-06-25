use std::{intrinsics::nearbyintf32, process::Output};

#[derive(Debug)]
pub struct Network;

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {  // Нейронная сеть
    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

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
    fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size))
            .collect();

        Self { neurons }
    }

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
    fn random(input_size: usize) -> Self {
        let bias = todo!();

        let weights = (0..input_size)
            .map(|_| todo!())
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}