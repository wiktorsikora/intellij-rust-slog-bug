use slog::{o, info};

pub fn foo() {
    let logger = slog::Logger::root(slog::Discard, o!());

    let ctrl_click_me = 0;

    info!(logger, "Arg: {arg}", arg="some arg"; "key" => "value");

    let _x = ctrl_click_me;
}