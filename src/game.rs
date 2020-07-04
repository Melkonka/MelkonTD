use sfml::window::{Style, Window};

pub struct Game {
  //state_stack: Vec<Box<dyn InputState>>,
}

impl Game {
  pub fn new() -> Self {
    Game {
     // state_stack: vec![Box::new(DefaultInputState)],
    }
  }

  pub fn run(&mut self) {
    let mut window = Window::new((1920, 1080), "MelkonTD", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);

    while window.is_open() {}
  }
}
