use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

const POST_KILL_GRACE_PERIOD: Duration = Duration::from_secs(10);
const POST_SHUTDOWN_GRACE_PERIOD: Duration = Duration::from_secs(60);
const SERVER_PID_FILE: &str = "server.pid.txt";

// extern const char kServerPidFile[];

// // The number of seconds the client will wait for the server process to
// // terminate itself after the client receives the final response from a command
// // that shuts down the server. After waiting this time, if the server process
// // remains, the client will forcibly kill the server.
// extern const unsigned int kPostShutdownGracePeriodSeconds;

// // The number of seconds the client will wait for the server process to
// // terminate after the client forcibly kills the server. After waiting this
// // time, if the server process remains, the client will die.
// extern const unsigned int kPostKillGracePeriodSeconds;

/// What WithEnvVar should do with an environment variable
pub enum EnvVarAction {
    Unset,
    Set,
}

pub struct EnvVarValue {
    /// What should be done with the given variable
    action: EnvVarAction,
    /// The value of the variable; ignored if action == UNSET
    value: String,
}

impl EnvVarValue {
    pub fn new(action: EnvVarAction, value: String) -> Self {
        Self { action, value }
    }
}

/// While this class is in scope, the specified environment variables will be set
/// to a specified value (or unset). When it leaves scope, changed variables will
/// be set to their original values.
struct WithEnvVars {
    old_values: HashMap<String, EnvVarValue>,
}

impl WithEnvVars {
    pub fn new(vars: HashMap<String, EnvVarValue>) -> Self {
        //   for (const auto& v : vars) {
        //     if (ExistsEnv(v.first)) {
        //       _old_values[v.first] = EnvVarValue(EnvVarAction::SET, GetEnv(v.first));
        //     } else {
        //       _old_values[v.first] = EnvVarValue(EnvVarAction::UNSET, "");
        //     }
        //   }
        //
        //   set_env_vars(vars);
        unimplemented!()
    }

    fn set_env_vars(vars: HashMap<String, EnvVarValue>) {
        //   for (const auto& var : vars) {
        //     switch (var.second.action) {
        //       case EnvVarAction::UNSET:
        //         UnsetEnv(var.first);
        //         break;
        //
        //       case EnvVarAction::SET:
        //         SetEnv(var.first, var.second.value);
        //         break;
        //
        //       default:
        //         assert(false);
        //     }
        //   }
        unimplemented!()
    }
}

impl Drop for WithEnvVars {
    fn drop(&mut self) {
        //   set_env_vars(_old_values);
        unimplemented!()
    }
}

/// If 'arg' matches 'key=value', returns address of 'value'.
/// If it matches 'key' alone, returns address of next_arg.
/// Returns NULL otherwise.
pub fn get_unary_option() -> Option<String> {
    // const char* get_unary_option(const char *arg, const char *next_arg, const char *key) {

    //   const char *value = blaze_util::var_strprefix(arg, key);
    //   if (value == NULL) {
    //     return NULL;
    //   } else if (value[0] == '=') {
    //     return value + 1;
    //   } else if (value[0]) {
    //     return NULL;  // trailing garbage in key name
    //   }
    //
    //   return next_arg;
    unimplemented!()
}

/// Returns true if 'arg' equals 'key'.
/// Dies with a syntax error if arg starts with 'key='.
/// Returns false otherwise.
pub fn get_nullary_option(arg: &str, key: &str) -> bool {
    // bool get_nullary_option(const char *arg, const char *key) {

    let value = arg.trim_start_matches(key);
    //   const char *value = blaze_util::var_strprefix(arg, key);

    if value.is_empty() {
        false
    } else if value.starts_with('=') {
        //     BAZEL_DIE(blaze_exit_code::BAD_ARGV)
        //         << "In argument '" << arg << "': option '" << key
        //         << "' does not take a value.";
        panic!("nopnopnop")
    } else {
        value.chars().next().is_none()
    }
}

/// Searches for 'key' in 'args' using get_unary_option. Arguments found after '--'
/// are omitted from the search.
/// When 'warn_if_dupe' is true, the method checks if 'key' is specified more
/// than once and prints a warning if so.
/// Returns the value of the 'key' flag iff it occurs in args.
/// Returns NULL otherwise.
pub fn search_unary_option(arg: &[String], key: String, warn_if_dupe: bool) -> String {
    // const char* search_unary_option(const vector<string>& args, const char *key, bool warn_if_dupe) {

    //   if (args.empty()) {
    //     return NULL;
    //   }
    //
    //   const char* value = nullptr;
    //   bool found_dupe = false;  // true if 'key' was found twice
    //   vector<string>::size_type i = 0;
    //
    //   // Examine the first N-1 arguments. (N-1 because we examine the i'th and
    //   // i+1'th together, in case a flag is defined "--name value" style and not
    //   // "--name=value" style.)
    //   for (; i < args.size() - 1; ++i) {
    //     if (args[i] == "--") {
    //       // If the current argument is "--", all following args are target names.
    //       // If 'key' was not found, 'value' is nullptr and we can return that.
    //       // If 'key' was found exactly once, then 'value' has the value and again
    //       // we can return that.
    //       // If 'key' was found more than once then we could not have reached this
    //       // line, because we would have broken out of the loop when 'key' was found
    //       // the second time.
    //       return value;
    //     }
    //     const char* result = get_unary_option(args[i].c_str(), args[i + 1].c_str(), key);
    //     if (result != NULL) {
    //       // 'key' was found and 'result' has its value.
    //       if (value) {
    //         // 'key' was found once before, because 'value' is not empty.
    //         found_dupe = true;
    //         break;
    //       } else {
    //         // 'key' was not found before, so store the value in 'value'.
    //         value = result;
    //       }
    //     }
    //   }
    //
    //   if (value) {
    //     // 'value' is not empty, so 'key' was found at least once in the first N-1
    //     // arguments.
    //     if (warn_if_dupe) {
    //       if (!found_dupe) {
    //         // We did not find a duplicate in the first N-1 arguments. Examine the
    //         // last argument, it may be a duplicate.
    //         found_dupe = (get_unary_option(args[i].c_str(), NULL, key) != nullptr);
    //       }
    //       if (found_dupe) {
    //         BAZEL_LOG(WARNING) << key << " is given more than once, " << "only the first occurrence is used";
    //       }
    //     }
    //     return value;
    //   } else {
    //     // 'value' is empty, so 'key' was not yet found in the first N-1 arguments.
    //     // If 'key' is in the last argument, we'll parse and return the value from
    //     // that, and if it isn't, we'll return NULL.
    //     return get_unary_option(args[i].c_str(), NULL, key);
    //   }
    unimplemented!()
}

/// Searches for '--flag_name' and '--noflag_name' in 'args' using
/// get_nullary_option. Arguments found after '--' are omitted from the search.
/// Returns true if '--flag_name' is a flag in args and '--noflag_name' does not
/// appear after its last occurrence. If neither '--flag_name' nor
/// '--noflag_name' appear, returns 'default_value'. Otherwise, returns false.
pub fn search_nullary_option(args: &[String], flag_name: &str, default_value: bool) -> bool {
    // bool search_nullary_option(const vector<string>& args, const string& flag_name, const bool default_value) {

    let positive_flag = &format!("--{}", flag_name);
    let negative_flag = &format!("--no{}", flag_name);

    for arg in args.iter() {
        //   for (vector<string>::size_type i = 0; i < args.size(); i++) {

        if arg == "--" {
            break;
        }

        if get_nullary_option(arg, positive_flag) {
            return true;
        } else if get_nullary_option(arg, negative_flag) {
            return false;
        }
    }

    default_value
}

/// Returns true iff arg is a valid command line argument for bazel.
pub fn is_arg(arg: &str) -> bool {
    arg.starts_with('-') && arg != "--help" && arg != "-help" && arg != "-h"
}

/// Returns the flag value as an absolute path. For legacy reasons, it accepts
/// the empty string as cwd.
/// TODO(b/109874628): Assess if removing the empty string case would break
/// legitimate uses, and if not, remove it.
pub fn absolute_path_from_flag() -> String {
    // std::string absolute_path_from_flag(const std::string& value) {

    //   if (value.empty()) {
    //     return blaze_util::GetCwd();
    //   } else if (!value.empty() && value[0] == '~') {
    //     return blaze_util::JoinPath(GetHomeDir(), value.substr(1));
    //   } else {
    //     return blaze_util::MakeAbsolute(value);
    //   }
    unimplemented!()
}

// void LogWait(unsigned int elapsed_seconds, unsigned int wait_seconds) {
//   SigPrintf("WARNING: Waiting for server process to terminate "
//             "(waited %d seconds, waiting at most %d)\n",
//             elapsed_seconds, wait_seconds);
// }

/// Wait to see if the server process terminates. Checks the server's status
/// immediately, and repeats the check every 100ms until approximately
/// wait_seconds elapses or the server process terminates. Returns true if a
/// check sees that the server process terminated. Logs to stderr after 5, 10,
/// and 30 seconds if the wait lasts that long.
pub fn await_server_process_termination(pid: i32, output_base: PathBuf, wait: Duration) -> bool {
    // bool await_server_process_termination(int pid, const blaze_util::Path& output_base, unsigned int wait_seconds) {

    //   uint64_t st = GetMillisecondsMonotonic();
    //   const unsigned int first_seconds = 5;
    //   bool logged_first = false;
    //   const unsigned int second_seconds = 10;
    //   bool logged_second = false;
    //   const unsigned int third_seconds = 30;
    //   bool logged_third = false;
    //
    //   while (VerifyServerProcess(pid, output_base)) {
    //     TrySleep(100);
    //     uint64_t elapsed_millis = GetMillisecondsMonotonic() - st;
    //     if (!logged_first && elapsed_millis > first_seconds * 1000) {
    //       LogWait(first_seconds, wait_seconds);
    //       logged_first = true;
    //     }
    //     if (!logged_second && elapsed_millis > second_seconds * 1000) {
    //       LogWait(second_seconds, wait_seconds);
    //       logged_second = true;
    //     }
    //     if (!logged_third && elapsed_millis > third_seconds * 1000) {
    //       LogWait(third_seconds, wait_seconds);
    //       logged_third = true;
    //     }
    //     if (elapsed_millis > wait_seconds * 1000) {
    //       SigPrintf("INFO: Waited %d seconds for server process (pid=%d) to"
    //                 " terminate.\n",
    //                 wait_seconds, pid);
    //       return false;
    //     }
    //   }
    //   return true;
    unimplemented!()
}

/// Control the output of debug information by debug_log.
/// Revisit once client logging is fixed (b/32939567).
///
/// For now, we don't have the client set up to log to a file. If --client_debug
/// is passed, however, all BAZEL_LOG statements will be output to stderr.
/// If/when we switch to logging these to a file, care will have to be taken to
/// either log to both stderr and the file in the case of --client_debug, or be
/// ok that these log lines will only go to one stream.
pub fn set_debug_log(enabled: bool) {
    // void set_debug_log(bool enabled) {

    //   if (enabled) {
    //     blaze_util::SetLoggingOutputStreamToStderr();
    //   } else {
    //     blaze_util::SetLoggingOutputStream(nullptr);
    //   }
    unimplemented!()
}

/// Returns true if this Bazel instance is running inside of a Bazel test.
/// This method observes the TEST_TMPDIR envvar.
pub fn is_running_within_test() -> bool {
    env::var("TEST_TMPDIR").is_ok()
}

#[cfg(target_family = "unix")]
pub fn hashed_base_dir(root: &PathBuf, hashable: &str) -> PathBuf {
    let digest = md5::compute(hashable);
    root.join(format!("{:x}", digest))
}

#[cfg(target_family = "windows")]
pub fn hashed_base_dir(root: PathBuf, hashable: &str) -> String {
    //   // Builds a shorter output base dir name for Windows.
    //
    //   // We create a path name representing the 128 bits of MD5 digest. To avoid
    //   // platform incompatibilities we restrict the alphabet to ASCII letters and
    //   // numbers. Windows paths are case-insensitive, so use only lower-case
    //   // letters. These constraints yield a 5-bit alphabet.
    //   // Since we only need 6 digits, ignore 0 and 1 because they look like
    //   // upper-case "O" and lower-case "l".
    //   static const char* alphabet = "abcdefghijklmnopqrstuvwxyz234567";
    //
    //   // 128 bits of data in base-32 require 128/5 = 25 digits with 3 bits lost.
    //   // Maximum path length on Windows is only 259 characters, so we'll only use
    //   // a few characters characters (base-32 digits) to represent the digest.
    //   // Using only 8 characters we represent 40 bits of the original 128.
    //   // Since the mapping is lossy and collisions are unlikely in practice, we'll
    //   // keep the mapping simple and just use the lower 5 bits of the first 8 bytes.
    //   static const unsigned char kLower5BitsMask = 0x1F;
    //   static const int filename_length = 8;
    //   unsigned char md5[blaze_util::Md5Digest::kDigestLength];
    //   char coded_name[filename_length + 1];
    //   blaze_util::Md5Digest digest;
    //   digest.Update(hashable.data(), hashable.size());
    //   digest.Finish(md5);
    //   for (int i = 0; i < filename_length; ++i) {
    //     coded_name[i] = alphabet[md5[i] & kLower5BitsMask];
    //   }
    //   coded_name[filename_length] = '\0';
    //   return blaze_util::JoinPath(root, string(coded_name));
    unimplemented!()
}
