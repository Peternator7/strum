use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr)]
enum State<'a> {
    Initial(&'a str),
    Finished,
}

fn print_state<'a>(s: &'a str) {
    let mut state = State::Initial(s);
    // The following won't work because the lifetime is incorrect:
    // let wrong: &'static str = state.as_ref();
    // using the trait implemented by the derive works however:
    let right: &'static str = state.into();
    println!("{}", right);
    state = State::Finished;
    let done: &'static str = state.into();
    println!("{}", done);
}

fn main() {
    print_state(&"hello world".to_string())
}
