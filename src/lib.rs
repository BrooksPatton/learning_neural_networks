use ggez::{
    event::EventHandler,
    graphics::{self, Color},
};
use neural_network::perceptron::Perceptron;
use rand::{rngs::ThreadRng, thread_rng};

#[derive(Debug)]
pub struct MainState {
    rng: ThreadRng,
}

impl MainState {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let inputs = [-1.0, 0.5];
        let perceptron = Perceptron::new(&mut rng);
        let guess = perceptron.guess(&inputs);

        dbg!(guess);
        Self { rng }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let canvas = graphics::Canvas::from_frame(context, Color::WHITE);

        canvas.finish(context)
    }
}
