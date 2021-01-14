extern crate bazel;

fn main() {
    let start_time = std::time::SystemTime::now();
    let args: Vec<String> = std::env::args().collect();
    let option_processor = bazel::OptionProcessor::new(bazel::StartupOptions::new());
    std::process::exit(bazel::main(args, option_processor, start_time) as i32);
}
