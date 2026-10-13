use redust::command::InteractiveMode;

fn main() {
    let interactive = InteractiveMode::new();

    let _ = interactive.read_cmd();
}
