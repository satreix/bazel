// #include <sys/types.h>
//
// #include <string>
// #include <vector>
//
// #include "src/main/cpp/util/path.h"
// #include "src/main/cpp/util/port.h"  // pid_t on Windows/MSVC
// #include "src/main/cpp/util/path.h"

use std::path::PathBuf;

/// Encapsulates information around the blaze server process and its
/// configuration.
pub struct ServerProcessInfo {
    /// When running as a daemon, where the deamonized server's stdout and
    /// stderr should be written.
    jvm_log_file: PathBuf,

    /// Whether or not the jvm_log_file should be opened with O_APPEND.
    jvm_log_file_append: bool,

    //   // TODO(laszlocsomor) 2016-11-28: move pid_t usage out of here and wherever
    //   // else it appears. Find some way to not have to declare a pid_t here, either
    //   // by making PID handling platform-independent or some other idea.
    //   pid_t server_pid_;
    pub server_pid: i32,
}

impl ServerProcessInfo {
    pub fn new(output_base: PathBuf, server_jvm_out: Option<PathBuf>) -> Self {
        // ServerProcessInfo::ServerProcessInfo(const blaze_util::Path &output_base, const blaze_util::Path &server_jvm_out)
        //     : jvm_log_file_(get_jvm_out_file(output_base, server_jvm_out)),
        //       jvm_log_file_append_(!server_jvm_out.IsEmpty()),
        //       // TODO(b/69972303): Formalize the "no server" magic value or rm it.
        //       server_pid_(-1) {}

        Self {
            jvm_log_file: Self::get_jvm_out_file(output_base, server_jvm_out.clone()),
            jvm_log_file_append: server_jvm_out.is_some(),

            // FIXME hardcoded value
            server_pid: 55020,
        }
    }

    fn get_jvm_out_file(output_base: PathBuf, server_jvm_out: Option<PathBuf>) -> PathBuf {
        // static blaze_util::Path get_jvm_out_file(const blaze_util::Path &output_base, const blaze_util::Path &server_jvm_out) {

        match server_jvm_out {
            Some(p) => p,
            None => {
                // output_base.GetRelative("server/jvm.out");

                output_base.join("server/jvm.out")

                // FIXME
                //.join("server/jvm.out").to_str().unwrap().to_owned()
            }
        }
    }
}
