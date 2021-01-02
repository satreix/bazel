extern crate dirs;

mod rc_file;

fn main() {
    let p = match dirs::home_dir() {
        None => {
            println!("could not get user directory");
            std::process::exit(1);
        }
        Some(x) => x.join(".bazelrc"),
    };

    let filename = match p.to_str() {
        None => {
            println!("could not get filename");
            std::process::exit(1);
        }
        Some(x) => x,
    };

    match rc_file::RcFile::parse(filename, "dummy-workspace") {
        Ok(rc) => {
            println!("rc: {:#?}", rc)
        }
        Err(e) => {
            println!("error: {:#?}", e);
            std::process::exit(1);
        }
    }

    unimplemented!();
}
