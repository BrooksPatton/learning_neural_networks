use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder,
};
use learning_neural_networks::MainState;

fn main() {
    let window_mode = WindowMode::default().dimensions(200.0, 200.0);
    let (context, event_loop) = ContextBuilder::new("Learning Neural Networks", "Brooks")
        .window_mode(window_mode)
        .build()
        .unwrap();
    let main_state = MainState::new();

    event::run(context, event_loop, main_state);
}
