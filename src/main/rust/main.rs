extern crate dirs;
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use slog::{error, info, o, Drain};

mod rc_file;

fn logger() -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

fn run() -> i32 {
    let log = logger();

    let p = dirs::home_dir().unwrap().join(".bazelrc");
    let filename = p.to_str().unwrap();

    match rc_file::RcFile::new(
        log.new(o!("component" => "rc_parser")),
        filename,
        "dummy-workspace",
    ) {
        Ok(rc) => {
            info!(log, "success"; "rc" => format!("{:#?}", rc))
        }
        Err(e) => {
            error!(log, "{:#?}", e);
            return 1;
        }
    }

    0
}

fn main() {
    let exit_code = run();
    if exit_code != 0 {
        std::process::exit(exit_code);
    }

    unimplemented!();
}
