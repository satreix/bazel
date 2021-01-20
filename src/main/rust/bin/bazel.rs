extern crate bazel;

extern crate slog;
use slog::{o, Drain};
extern crate slog_async;
extern crate slog_term;

fn logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

fn main() {
    let log = logger();
    let start_time = std::time::SystemTime::now();
    let args: Vec<String> = std::env::args().collect();
    let option_processor = bazel::OptionProcessor::new(
        log.new(o!("component" => "option_parser")),
        bazel::StartupOptions::new(String::from("Bazel")),
    );
    std::process::exit(bazel::main(log, args, option_processor, start_time) as i32);
}
