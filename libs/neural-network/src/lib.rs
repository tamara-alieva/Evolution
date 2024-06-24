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

#[derive(Debug)]
struct Neuron {         // Нейрон
    bias: f32,          // Смещение
    weights: Vec<f32>   // Весы
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}