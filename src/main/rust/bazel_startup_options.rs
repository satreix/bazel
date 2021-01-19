// #include <cassert>
// #include <string>
//
// #include "src/main/cpp/blaze_util.h"
// #include "src/main/cpp/startup_options.h"
// #include "src/main/cpp/util/exit_code.h"
// #include "src/main/cpp/util/logging.h"
// #include "src/main/cpp/workspace_layout.h"

use std::path::PathBuf;

use crate::exit_code::ExitCode;
use crate::startup_options::{RcStartupFlag, StartupOptionsTrait};

/// BazelStartupOptions contains the startup options that are Bazel-specific.
struct BazelStartupOptions {
    // class BazelStartupOptions : public StartupOptions {
    // FIXME there is not inheritance in Rust so lets look at traits to lint this to StartupOptions

    //  public:
    //   explicit BazelStartupOptions(const WorkspaceLayout *workspace_layout);
    //
    //   void AddExtraOptions(std::vector<std::string> *result) const override;
    //
    //   void MaybeLogStartupOptionWarnings() const override;
    //
    //  protected:
    //   std::string GetRcFileBaseName() const override { return ".bazelrc"; }
    user_bazelrc: String,
    use_system_rc: bool,
    use_workspace_rc: bool,
    use_home_rc: bool,
    // TODO(b/36168162): Remove the master rc flag.
    use_master_bazelrc: bool,
}

impl BazelStartupOptions {
    fn new() -> Self {
        // BazelStartupOptions::BazelStartupOptions(const WorkspaceLayout *workspace_layout)
        //     : StartupOptions("Bazel", workspace_layout) {
        // FIXME there is not inheritance in Rust

        //   RegisterNullaryStartupFlagNoRc("home_rc", &use_home_rc);
        //   RegisterNullaryStartupFlagNoRc("master_bazelrc", &use_master_bazelrc_);
        //   OverrideOptionSourcesKey("master_bazelrc", "blazerc");
        //   RegisterNullaryStartupFlagNoRc("system_rc", &use_system_rc);
        //   RegisterNullaryStartupFlagNoRc("workspace_rc", &use_workspace_rc);
        //   RegisterUnaryStartupFlag("bazelrc");
        Self {
            user_bazelrc: "".to_string(),
            use_system_rc: true,
            use_workspace_rc: true,
            use_home_rc: true,
            use_master_bazelrc: true,
        }
    }

    pub fn process_arg_extra() -> ExitCode {
        // blaze_exit_code::ExitCode BazelStartupOptions::process_arg_extra(
        //     const char *arg, const char *next_arg, const std::string &rcfile,
        //     const char **value, bool *is_processed, std::string *error) {

        unimplemented!();
        //   assert(value);
        //   assert(is_processed);
        //
        //   if ((*value = GetUnaryOption(arg, next_arg, "--bazelrc")) != nullptr) {
        //     if (!rcfile.empty()) {
        //       *error = "Can't specify --bazelrc in the RC file.";
        //       return blaze_exit_code::BAD_ARGV;
        //     }
        //     user_bazelrc_ = *value;
        //   } else {
        //     *is_processed = false;
        //     return blaze_exit_code::SUCCESS;
        //   }
        //
        //   *is_processed = true;
        //   return blaze_exit_code::SUCCESS;
    }

    // void BazelStartupOptions::MaybeLogStartupOptionWarnings() const {
    //   if (ignore_all_rc_files) {
    //     if (!user_bazelrc_.empty()) {
    //       BAZEL_LOG(WARNING) << "Value of --bazelrc is ignored, since --ignore_all_rc_files is on.";
    //     }
    //     if ((use_home_rc) &&
    //         option_sources.find("home_rc") != option_sources.end()) {
    //       BAZEL_LOG(WARNING) << "Explicit value of --home_rc is ignored, since --ignore_all_rc_files is on.";
    //     }
    //     if ((use_system_rc) &&
    //         option_sources.find("system_rc") != option_sources.end()) {
    //       BAZEL_LOG(WARNING) << "Explicit value of --system_rc is ignored, since --ignore_all_rc_files is on.";
    //     }
    //     if ((use_workspace_rc) &&
    //         option_sources.find("workspace_rc") != option_sources.end()) {
    //       BAZEL_LOG(WARNING) << "Explicit value of --workspace_rc is ignored, since --ignore_all_rc_files is on.";
    //     }
    //   }
    //   bool output_user_root_has_space =
    //       output_user_root.find_first_of(' ') != std::string::npos;
    //   if (output_user_root_has_space) {
    //     BAZEL_LOG(WARNING)
    //         << "Output user root \"" << output_user_root
    //         << "\" contains a space. This will probably break the build. "
    //            "You should set a different --output_user_root.";
    //   } else if (output_base.Contains(' ')) {
    //     // output_base is computed from output_user_root by default.
    //     // If output_user_root was bad, don't check output_base: while output_base
    //     // may also be bad, we already warned about output_user_root so there's no
    //     // point in another warning.
    //     BAZEL_LOG(WARNING)
    //         << "Output base \"" << output_base.AsPrintablePath()
    //         << "\" contains a space. This will probably break the build. "
    //            "You should not set --output_base and let Bazel use the default, or "
    //            "set --output_base to a path without space.";
    //   }
    // }
    //
    // void BazelStartupOptions::AddExtraOptions(std::vector<std::string> *result) const {
    //   StartupOptions::AddExtraOptions(result);
    // }
}

impl StartupOptionsTrait for BazelStartupOptions {
    fn process_args(rcstartup_flags: Vec<RcStartupFlag>) -> Result<(), (String, ExitCode)> {
        unimplemented!()
    }

    fn add_extra_options(result: Vec<String>) {
        unimplemented!()
    }

    fn get_jvm() -> PathBuf {
        unimplemented!()
    }

    fn get_exe(jvm: PathBuf, jar_path: &str) -> PathBuf {
        unimplemented!()
    }

    fn add_jvm_argument_prefix(javabase: PathBuf, result: Vec<String>) {
        unimplemented!()
    }

    fn add_jvm_argument_suffix(real_install_dir: PathBuf, jar_path: String, result: Vec<String>) {
        unimplemented!()
    }
}
