use rand::{rngs::ThreadRng, Rng};

// We can think of this Perceptron as a neuron. It's job is to take in data (the inputs) and then make a guess.
#[derive(Debug)]
pub struct Perceptron {
    weights: [f32; 2],
}

impl Perceptron {
    // The weights begin completely random, and changing them can happen either in supervised learning, or with neural evolution.
    pub fn new(rng: &mut ThreadRng) -> Self {
        let weights = [rng.gen_range(-1.0..=1.0); 2];

        Self { weights }
    }

    // Note that the number of inputs match the number of weights. We are using the type system to enforce this.
    pub fn guess(&self, inputs: &[f32; 2]) -> i8 {
        let mut sum = 0.0;
        for (index, weight) in self.weights.iter().enumerate() {
            sum += inputs[index] * weight;
        }

        self.activate(sum)
    }

    // This is a sigmoid function, it is ensuring that no matter what the raw guess is from the neuron we get just a 1 or -1 out of it.
    fn activate(&self, guess: f32) -> i8 {
        if guess >= 0.0 {
            1
        } else {
            -1
        }
    }
}
