use rand::{Rng, RngCore};

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Clone, Copy, Debug)]
pub struct LayerTopology { // Топология слоёв
    pub neurons: usize,
}

impl Network {  // Нейронная сеть
    pub fn new(layers: Vec<Layer>) -> Self { // конструктор
        Self { layers }
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self { // рандомайзер
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> { // распространение
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> Vec<f32> {
        todo!()
    }
}

#[derive(Clone, Debug)]
struct Layer {  // Слой сети
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());
        assert!(neurons
            .iter()
            .all(|neuron| neuron.weights.len() == neurons[0].weights.len()));

        Self { neurons }
    }

    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Self::new(neurons)
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[derive(Clone, Debug)]
struct Neuron {         // Нейрон
    bias: f32,          // Смещение
    weights: Vec<f32>   // Весы
}

impl Neuron {
    pub fn new(bias: f32, weights: Vec<f32>) -> Self { // конструктор
        assert!(!weights.is_empty());
        Self { bias, weights }
    }

    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();

        Self::new(bias, weights)
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

// Тесты
#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_relative_eq, assert_relative_ne};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() { // рандомайзер
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        //---------------------------------------------------------------------

        // Нейрон
        let neuron = Neuron::random(&mut rng, 4);
        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(neuron.weights.as_slice(), &[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_slice());

        //---------------------------------------------------------------------

        // Нейронная сеть
        let network = Network::random(
            &mut rng,
            &[
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 2 },
                LayerTopology { neurons: 1 },
            ],
        );
        assert_eq!(network.layers.len(), 2);
        assert_eq!(network.layers[0].neurons.len(), 2);
        assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);
        assert_relative_eq!(
            network.layers[0].neurons[0].weights.as_slice(),
            &[0.67383957, 0.8181262, 0.26284897].as_slice()
        );
        assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238807);
        assert_relative_eq!(
            network.layers[0].neurons[1].weights.as_slice(),
            &[-0.5351684, 0.069369555, -0.7648182].as_slice()
        );
        assert_eq!(network.layers[1].neurons.len(), 1);
        assert_relative_eq!(
            network.layers[1].neurons[0].weights.as_slice(),
            &[-0.48879623, -0.19277143].as_slice()
        );

        //---------------------------------------------------------------------

        // Слой
        let layer = Layer::random(&mut rng, 3, 2);
        let actual_biases: Vec<_> = layer.neurons.iter().map(|neuron| neuron.bias).collect();
        let expected_biases = vec![-0.6255188, 0.5238807];
        let actual_weights: Vec<_> = layer
            .neurons
            .iter()
            .map(|neuron| neuron.weights.as_slice())
            .collect();
        let expected_weights: Vec<&[f32]> = vec![
            &[0.67383957, 0.8181262, 0.26284897],
            &[-0.53516835, 0.069369674, -0.7648182],
        ];
        assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());
        assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());

    }

    #[test]
    fn propagate() {
        // Нейрон
        #[test]
        fn returns_propagated_input() { // распространяемое значение
            let actual = Neuron::new(0.1, vec![-0.3, 0.6, 0.9]).propagate(&[0.5, -0.6, 0.7]);
            let expected: f32 = 0.1 + (0.5 * -0.3) + (-0.6 * 0.6) + (0.7 * 0.9);
            approx::assert_relative_eq!(actual, expected.max(0.0));
        }

        #[test]
        fn restricts_output() { // ограничение выходного значения
            let neuron = Neuron::new(0.0, vec![0.5]);
            let v1 = neuron.propagate(&[-1.0]);
            let v2 = neuron.propagate(&[-0.5]);
            let v3 = neuron.propagate(&[0.0]);
            let v4 = neuron.propagate(&[0.5]);
            let v5 = neuron.propagate(&[1.0]);
            assert_relative_eq!(v1, v2);
            assert_relative_eq!(v2, v3);
            assert_relative_ne!(v3, v4);
            assert_relative_ne!(v4, v5);
        }

        //---------------------------------------------------------------------

        // Нейронная сеть
        let layers = (
            Layer::new(vec![
                Neuron::new(0.0, vec![-0.5, -0.4, -0.3]),
                Neuron::new(0.0, vec![-0.2, -0.1, 0.0]),
            ]),
            Layer::new(vec![Neuron::new(0.0, vec![-0.5, 0.5])]),
        );
        let network = Network::new(vec![layers.0.clone(), layers.1.clone()]);
        let actual = network.propagate(vec![0.5, 0.6, 0.7]);
        let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));
        assert_relative_eq!(actual.as_slice(), expected.as_slice());

        //---------------------------------------------------------------------

        // Слой
        let neurons = (
            Neuron::new(0.0, vec![0.1, 0.2, 0.3]),
            Neuron::new(0.0, vec![0.4, 0.5, 0.6]),
        );

        let layer = Layer::new(vec![neurons.0.clone(), neurons.1.clone()]);
        let inputs = &[-0.5, 0.0, 0.5];

        let actual = layer.propagate(inputs.to_vec());
        let expected = vec![neurons.0.propagate(inputs), neurons.1.propagate(inputs)];

        assert_relative_eq!(actual.as_slice(), expected.as_slice());

    }
}