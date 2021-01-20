extern crate dirs;
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::{error, info, o, Drain};

extern crate bazel;

fn logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

fn run() -> i32 {
    let log = logger();

    let cwd = std::env::current_dir().unwrap();
    let w = match bazel::workspace_layout::find_workspace(cwd.to_str().unwrap()) {
        Ok(w) => w,
        Err(e) => {
            error!(log, "get_workspace"; "error" => format!("{:#?}", e));
            return 1;
        }
    };
    info!(log, "get_workspace"; "dir" => format!("{:#?}", w));

    let pretty_name = bazel::workspace_layout::pretty_workspace_name(w);
    info!(log, "pretty_workspace_name"; "name" => format!("{:#?}", pretty_name));

    let p = dirs::home_dir().unwrap().join(".bazelrc");
    let filename = p.to_str().unwrap();

    let rc = match bazel::rc_file::RcFile::new(
        log.new(o!("component" => "rc_parser")),
        filename,
        "dummy-workspace",
    ) {
        Ok(rc) => rc,
        Err(e) => {
            error!(log, "{:#?}", e);
            return 1;
        }
    };
    info!(log, "success"; "rc" => format!("{:#?}", rc));

    0
}

fn main() {
    let exit_code = run();
    if exit_code != 0 {
        std::process::exit(exit_code);
    }

    unimplemented!();
}
