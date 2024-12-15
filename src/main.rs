use mod_loader_gui;

fn main() {
    tracing_subscriber::fmt::init();
    let _ = mod_loader_gui::run();
}
