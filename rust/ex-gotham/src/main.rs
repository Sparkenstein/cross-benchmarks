
use gotham::state::State;

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello World!")
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    gotham::start("127.0.0.1:3000", || Ok(say_hello))
}