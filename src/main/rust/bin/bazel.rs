extern crate bazel;

fn main() {
    let start_time = std::time::SystemTime::now();
    let args: Vec<String> = std::env::args().collect();

    // std::unique_ptr<blaze::WorkspaceLayout> workspace_layout(new blaze::WorkspaceLayout());
    // std::unique_ptr<blaze::StartupOptions> startup_options(new blaze::BazelStartupOptions(workspace_layout.get()));

    let exit_code = bazel::main(
        args,
        // workspace_layout.get(),
        // new blaze::OptionProcessor(workspace_layout.get(), std::move(startup_options)),
        start_time,
    );

    std::process::exit(exit_code as i32);
}
