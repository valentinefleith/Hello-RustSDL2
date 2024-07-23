pub enum State {
    Running,
    Paused,
}

pub struct Point(pub i32, pub i32);

pub struct Context {
    pub food: Point,
    pub state: State,
}

impl Context {
    pub fn new() -> Context {
        Context {
            food: Point(3, 3),
            state: State::Paused,
        }
    }
}
