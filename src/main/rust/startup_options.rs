// @@@ Header from src/main/cpp/startup_options.h @@@
// #if defined(__APPLE__)
// #include <sys/qos.h>
// #endif
// 
// #include <functional>
// #include <map>
// #include <string>
// #include <unordered_map>
// #include <unordered_set>
// #include <utility>
// #include <vector>
// 
// #include "src/main/cpp/util/exit_code.h"
// #include "src/main/cpp/util/path.h"
// 
// class WorkspaceLayout;

use std::collections::{HashMap, HashSet};

// FIXME //   typedef std::function<void(bool)> SpecialNullaryFlagHandler;

/// A startup flag tagged with its origin, either an rc file or the empty
/// string for the ones specified in the command line.
/// For instance, RcStartupFlag("somepath/.bazelrc", "--foo") is used to
/// represent that the line "startup --foo" was found when parsing
/// "somepath/.bazelrc".
struct RcStartupFlag {
    source: String,
    value: String,
}

impl RcStartupFlag {
    fn new(source: String, value: String) -> Self {
        Self{ source, value }
    }
}

/// This class defines the startup options accepted by all versions Bazel, and
/// holds the parsed values. These options and their defaults must be kept in
/// sync with those in
/// src/main/java/com/google/devtools/build/lib/runtime/BlazeServerStartupOptions.java.
/// The latter are (usually) purely decorative (they affect the help message,
/// which displays the defaults).  The actual defaults are defined
/// in the constructor.
///
/// Note that this class is not thread-safe.
///
/// TODO(bazel-team): The encapsulation is not quite right -- there are some
/// places in blaze.cc where some of these fields are explicitly modified. Their
/// names also don't conform to the style guide.
struct StartupOptions {
    //  public:
    //   virtual ~StartupOptions();
    //
    //   // Process an ordered list of RcStartupFlags using ProcessArg.
    //   blaze_exit_code::ExitCode ProcessArgs(
    //       const std::vector<RcStartupFlag>& rcstartup_flags,
    //       std::string *error);
    //
    //   // Adds any other options needed to result.
    //   //
    //   // TODO(jmmv): Now that we support site-specific options via subclasses of
    //   // StartupOptions, the "ExtraOptions" concept makes no sense; remove it.
    //   virtual void AddExtraOptions(std::vector<std::string> *result) const;
    //
    //   // Once startup options have been parsed, warn the user if certain options
    //   // might combine in surprising ways.
    //   virtual void MaybeLogStartupOptionWarnings() const = 0;
    //
    //   // Returns the path to the JVM. This should be called after parsing
    //   // the startup options.
    //   virtual blaze_util::Path GetJvm() const;
    //
    //   // Returns the executable used to start the Blaze server, typically the given
    //   // JVM.
    //   virtual blaze_util::Path GetExe(const blaze_util::Path &jvm,
    //                                   const std::string &jar_path) const;
    //
    //   // Adds JVM prefix flags to be set. These will be added before all other
    //   // JVM flags.
    //   virtual void AddJVMArgumentPrefix(const blaze_util::Path &javabase,
    //                                     std::vector<std::string> *result) const;
    //
    //   // Adds JVM suffix flags. These will be added after all other JVM flags, and
    //   // just before the Blaze server startup flags.
    //   virtual void AddJVMArgumentSuffix(const blaze_util::Path &real_install_dir,
    //                                     const std::string &jar_path,
    //                                     std::vector<std::string> *result) const;
    //
    //   // Adds JVM tuning flags for Blaze.
    //   //
    //   // Returns the exit code after this operation. "error" will be set to a
    //   // descriptive string for any value other than blaze_exit_code::SUCCESS.
    //   blaze_exit_code::ExitCode AddJVMArguments(
    //       const blaze_util::Path &server_javabase, std::vector<std::string> *result,
    //       const std::vector<std::string> &user_options, std::string *error) const;
    //
    //   // Checks whether "arg" is a valid nullary option (e.g. "--master_bazelrc" or
    //   // "--nomaster_bazelrc").
    //   //
    //   // Returns true, if "arg" looks like either a valid nullary option or a
    //   // potentially valid unary option. In this case, "result" will be populated
    //   // with true iff "arg" is definitely a valid nullary option.
    //   //
    //   // Returns false, if "arg" looks like an attempt to pass a value to nullary
    //   // option (e.g. "--nullary_option=idontknowwhatimdoing"). In this case,
    //   // "error" will be populated with a user-friendly error message.
    //   //
    //   // Therefore, callers of this function should look at the return value and
    //   // then either look at "result" (on true) or "error" (on false).
    //   bool MaybeCheckValidNullary(const std::string &arg, bool *result,
    //                               std::string *error) const;
    //
    //   // Checks whether the argument is a valid unary option.
    //   // E.g. --blazerc=foo, --blazerc foo.
    //   bool IsUnary(const std::string& arg) const;
    //
    //   std::string GetLowercaseProductName() const;
    //
    //   // The capitalized name of this binary.
    //   const std::string product_name;
    //
    //   // If supplied, alternate location to write the blaze server's jvm's stdout.
    //   // Otherwise a default path in the output base is used.
    //   blaze_util::Path server_jvm_out;
    //
    //   // If supplied, alternate location to write a serialized failure_detail proto.
    //   // Otherwise a default path in the output base is used.
    //   blaze_util::Path failure_detail_out;
    //
    //   // Blaze's output base.  Everything is relative to this.  See
    //   // the BlazeDirectories Java class for details.
    //   blaze_util::Path output_base;
    //
    //   // Installation base for a specific release installation.
    //   std::string install_base;
    //
    //   // The toplevel directory containing Blaze's output.  When Blaze is
    //   // run by a test, we use TEST_TMPDIR, simplifying the correct
    //   // hermetic invocation of Blaze from tests.
    //   std::string output_root;
    //
    //   // Blaze's output_user_root. Used only for computing install_base and
    //   // output_base.
    //   std::string output_user_root;
    //
    //   // Override more finegrained rc file flags and ignore them all.
    //   bool ignore_all_rc_files;
    //
    //   // Block for the Blaze server lock. Otherwise,
    //   // quit with non-0 exit code if lock can't
    //   // be acquired immediately.
    //   bool block_for_lock;
    //
    //   bool host_jvm_debug;
    //
    //   bool autodetect_server_javabase;
    //
    //   std::string host_jvm_profile;
    //
    //   std::vector<std::string> host_jvm_args;
    //
    //   bool batch;
    //
    //   // From the man page: "This policy is useful for workloads that are
    //   // non-interactive, but do not want to lower their nice value, and for
    //   // workloads that want a deterministic scheduling policy without
    //   // interactivity causing extra preemptions (between the workload's tasks)."
    //   bool batch_cpu_scheduling;
    //
    //   // If negative, don't mess with ionice. Otherwise, set a level from 0-7
    //   // for best-effort scheduling. 0 is highest priority, 7 is lowest.
    //   int io_nice_level;
    //
    //   int max_idle_secs;
    //
    //   bool shutdown_on_low_sys_mem;
    //
    //   bool oom_more_eagerly;
    //
    //   int oom_more_eagerly_threshold;
    //
    //   bool write_command_log;
    //
    //   // If true, Blaze will listen to OS-level file change notifications.
    //   bool watchfs;
    //
    //   // Temporary flag for enabling EventBus exceptions to be fatal.
    //   bool fatal_event_bus_exceptions;
    //
    //   // A string to string map specifying where each option comes from. If the
    //   // value is empty, it was on the command line, if it is a string, it comes
    //   // from a blazerc file, if a key is not present, it is the default.
    //   std::map<std::string, std::string> option_sources;
    //
    //   // Returns the embedded JDK, or an empty string.
    //   blaze_util::Path GetEmbeddedJavabase() const;
    //
    //   // The source of truth for the server javabase.
    //   enum class JavabaseType {
    //     UNKNOWN,
    //     // An explicit --server_javabase startup option.
    //     EXPLICIT,
    //     // The embedded JDK.
    //     EMBEDDED,
    //     // The default system JVM.
    //     SYSTEM
    //   };
    //
    //   // Returns the server javabase and its source of truth. This should be called
    //   // after parsing the --server_javabase option.
    //   std::pair<blaze_util::Path, JavabaseType> GetServerJavabaseAndType() const;
    //
    //   // Returns the server javabase. This should be called after parsing the
    //   // --server_javabase option.
    //   blaze_util::Path GetServerJavabase() const;
    //
    //   // Returns the explicit value of the --server_javabase startup option or the
    //   // empty string if it was not specified on the command line.
    //   blaze_util::Path GetExplicitServerJavabase() const;
    //
    //   // Port to start up the gRPC command server on. If 0, let the kernel choose.
    //   int command_port;
    //
    //   // Connection timeout for each gRPC connection attempt.
    //   int connect_timeout_secs;
    //
    //   // Local server startup timeout duration.
    //   int local_startup_timeout_secs;
    //
    //   // Invocation policy proto, or an empty string.
    //   std::string invocation_policy;
    //   // Invocation policy can only be specified once.
    //   bool have_invocation_policy_;
    //
    //   // Whether to output addition debugging information in the client.
    //   bool client_debug;
    //
    //   // Whether the resulting command will be preempted if a subsequent command is
    //   // run.
    //   bool preemptible;
    //
    //   // Value of the java.util.logging.FileHandler.formatter Java property.
    //   std::string java_logging_formatter;
    //
    //   bool expand_configs_in_place;
    //
    //   // The hash function to use when computing file digests.
    //   std::string digest_function;
    //
    //   std::string unix_digest_hash_attribute_name;
    //
    //   bool idle_server_tasks;
    //
    //   // The startup options as received from the user and rc files, tagged with
    //   // their origin. This is populated by ProcessArgs.
    //   std::vector<RcStartupFlag> original_startup_options_;
    //
    // #if defined(__APPLE__)
    //   // The QoS class to apply to the Bazel server process.
    //   qos_class_t macos_qos_class;
    // #endif
    //
    //   // Whether to raise the soft coredump limit to the hard one or not.
    //   bool unlimit_coredumps;
    //
    //   // Whether the execution transition is enabled, or behaves like a host
    //   // transition. This must be set before rule classes are constructed.
    //   // See https://github.com/bazelbuild/bazel/issues/7935
    //   bool incompatible_enable_execution_transition;
    //
    //   // Whether to create symbolic links on Windows for files. Requires
    //   // developer mode to be enabled.
    //   bool windows_enable_symlinks;
    //
    //  protected:
    //   // Constructor for subclasses only so that site-specific extensions of this
    //   // class can override the product name.  The product_name must be the
    //   // capitalized version of the name, as in "Bazel".
    //   StartupOptions(const std::string &product_name,
    //                  const WorkspaceLayout *workspace_layout);
    //
    //   // Checks extra fields when processing arg.
    //   //
    //   // Returns the exit code after processing the argument. "error" will contain
    //   // a descriptive string for any return value other than
    //   // blaze_exit_code::SUCCESS.
    //   //
    //   // TODO(jmmv): Now that we support site-specific options via subclasses of
    //   // StartupOptions, the "ExtraOptions" concept makes no sense; remove it.
    //   virtual blaze_exit_code::ExitCode process_arg_extra(
    //       const char *arg, const char *next_arg, const std::string &rcfile,
    //       const char **value, bool *is_processed, std::string *error) = 0;
    //
    //   // Checks whether the given javabase contains a java executable and runtime.
    //   // On success, returns blaze_exit_code::SUCCESS. On error, prints an error
    //   // message and returns an appropriate exit code with which the client should
    //   // terminate.
    //   blaze_exit_code::ExitCode SanityCheckJavabase(
    //       const blaze_util::Path &javabase,
    //       StartupOptions::JavabaseType javabase_type) const;
    //
    //   // Returns the absolute path to the user's local JDK install, to be used as
    //   // the default target javabase and as a fall-back host_javabase. This is not
    //   // the embedded JDK.
    //   virtual blaze_util::Path GetSystemJavabase() const;
    //
    //   // Adds JVM logging-related flags for Bazel.
    //   //
    //   // This is called by StartupOptions::AddJVMArguments and is a separate method
    //   // so that subclasses of StartupOptions can override it.
    //   virtual void AddJVMLoggingArguments(std::vector<std::string> *result) const;
    //
    //   // Adds JVM memory tuning flags for Bazel.
    //   //
    //   // This is called by StartupOptions::AddJVMArguments and is a separate method
    //   // so that subclasses of StartupOptions can override it.
    //   virtual blaze_exit_code::ExitCode AddJVMMemoryArguments(
    //       const blaze_util::Path &server_javabase, std::vector<std::string> *result,
    //       const std::vector<std::string> &user_options, std::string *error) const;
    //
    //   virtual std::string GetRcFileBaseName() const = 0;
    //
    //   void RegisterUnaryStartupFlag(const std::string& flag_name);
    //
    //   // Register a nullary startup flag.
    //   // Both '--flag_name' and '--noflag_name' will be registered as valid nullary
    //   // flags. 'value' is the pointer to the boolean that will receive the flag's
    //   // value.
    //   void RegisterNullaryStartupFlag(const std::string &flag_name, bool *value);
    //
    //   // Same as RegisterNullaryStartupFlag, but these flags are forbidden in
    //   // .bazelrc files.
    //   void RegisterNullaryStartupFlagNoRc(const std::string &flag_name,
    //                                       bool *value);
    //
    //   typedef std::function<void(bool)> SpecialNullaryFlagHandler;
    //
    //   void RegisterSpecialNullaryStartupFlag(const std::string &flag_name, SpecialNullaryFlagHandler handler);
    //
    //   // Override the flag name to use in the 'option_sources' map.
    //   void OverrideOptionSourcesKey(const std::string &flag_name,
    //                                 const std::string &new_name);
    //
    //  private:
    //   // Prevent copying and moving the object to avoid invalidating pointers to
    //   // members (in all_nullary_startup_flags_ for example).
    //   StartupOptions() = delete;
    //   StartupOptions(const StartupOptions&) = delete;
    //   StartupOptions& operator=(const StartupOptions&) = delete;
    //   StartupOptions(StartupOptions&&) = delete;
    //   StartupOptions& operator=(StartupOptions&&) = delete;
    //
    //   // Parses a single argument, either from the command line or from the .blazerc
    //   // "startup" options.
    //   //
    //   // rcfile should be an empty string if the option being parsed does not come
    //   // from a blazerc.
    //   //
    //   // Sets "is_space_separated" true if arg is unary and uses the "--foo bar"
    //   // style, so its value is in next_arg.
    //   //
    //   // Sets "is_space_separated" false if arg is either nullary
    //   // (e.g. "--[no]batch") or is unary but uses the "--foo=bar" style.
    //   //
    //   // Returns the exit code after processing the argument. "error" will contain
    //   // a descriptive string for any return value other than
    //   // blaze_exit_code::SUCCESS.
    //   blaze_exit_code::ExitCode ProcessArg(const std::string &arg,
    //                                        const std::string &next_arg,
    //                                        const std::string &rcfile,
    //                                        bool *is_space_separated,
    //                                        std::string *error);
    //
    //   // The server javabase as provided on the commandline.
    //   blaze_util::Path explicit_server_javabase_;
    //
    //   // The default server javabase to be used and its source of truth (computed
    //   // lazily). Not guarded by a mutex - StartupOptions is not thread-safe.
    //   mutable std::pair<blaze_util::Path, JavabaseType> default_server_javabase_;

    //   // Startup flags that don't expect a value, e.g. "master_bazelrc".
    //   // Valid uses are "--master_bazelrc" are "--nomaster_bazelrc".
    //   // Keys are positive and negative flag names (e.g. "--master_bazelrc" and
    //   // "--nomaster_bazelrc"), values are pointers to the boolean to mutate.
    //   std::unordered_map<std::string, bool *> all_nullary_startup_flags_;

    //   // Subset of 'all_nullary_startup_flags_'.
    //   // Contains positive and negative names (e.g. "--master_bazelrc" and
    //   // "--nomaster_bazelrc") of flags that must not appear in .bazelrc files.
    //   std::unordered_set<std::string> no_rc_nullary_startup_flags_;

    //   // Subset of 'all_nullary_startup_flags_'.
    //   // Contains positive and negative names (e.g. "--master_bazelrc" and
    //   // "--nomaster_bazelrc") of flags that have a special handler.
    //   // Can be used for tri-state flags where omitting the flag completely means
    //   // leaving the tri-state as "auto".
    //   std::unordered_map<std::string, SpecialNullaryFlagHandler>
    //       special_nullary_startup_flags_;
    special_nullary_startup_flags: HashMap<String, >

    /// Startup flags that expect a value, e.g. "bazelrc".
    /// Valid uses are "--bazelrc=foo" and "--bazelrc foo".
    /// Keys are flag names (e.g. "--bazelrc"), values are pointers to the string
    /// to mutate.
    valid_unary_startup_flags: HashSet<String>,

    /// Startup flags that use an alternative key name in the 'option_sources' map.
    /// For example, "--[no]master_bazelrc" uses "blazerc" as the map key.
    option_sources_key_override: HashMap<string, String>,
}

impl StartupOptions {}

// @@@ Implementation from src/main/cpp/startup_options.cc @@@
// // Copyright 2014 The Bazel Authors. All rights reserved.
// //
// // Licensed under the Apache License, Version 2.0 (the "License");
// // you may not use this file except in compliance with the License.
// // You may obtain a copy of the License at
// //
// //    http://www.apache.org/licenses/LICENSE-2.0
// //
// // Unless required by applicable law or agreed to in writing, software
// // distributed under the License is distributed on an "AS IS" BASIS,
// // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// // See the License for the specific language governing permissions and
// // limitations under the License.
// #include "src/main/cpp/startup_options.h"
// 
// #include <assert.h>
// 
// #include <cstdio>
// #include <cstdlib>
// #include <cstring>
// 
// #include "src/main/cpp/blaze_util.h"
// #include "src/main/cpp/blaze_util_platform.h"
// #include "src/main/cpp/util/errors.h"
// #include "src/main/cpp/util/exit_code.h"
// #include "src/main/cpp/util/file.h"
// #include "src/main/cpp/util/logging.h"
// #include "src/main/cpp/util/numbers.h"
// #include "src/main/cpp/util/path.h"
// #include "src/main/cpp/util/path_platform.h"
// #include "src/main/cpp/util/strings.h"
// #include "src/main/cpp/workspace_layout.h"
// 
// using std::string;
// using std::vector;
// 
// void StartupOptions::RegisterNullaryStartupFlag(const std::string &flag_name,
//                                                 bool *flag_value) {
//   all_nullary_startup_flags_[std::string("--") + flag_name] = flag_value;
//   all_nullary_startup_flags_[std::string("--no") + flag_name] = flag_value;
// }
// 
// void StartupOptions::RegisterNullaryStartupFlagNoRc(
//     const std::string &flag_name, bool *flag_value) {
//   RegisterNullaryStartupFlag(flag_name, flag_value);
//   no_rc_nullary_startup_flags_.insert(std::string("--") + flag_name);
//   no_rc_nullary_startup_flags_.insert(std::string("--no") + flag_name);
// }
// 
// void StartupOptions::RegisterSpecialNullaryStartupFlag(
//     const std::string &flag_name, SpecialNullaryFlagHandler handler) {
//   RegisterNullaryStartupFlag(flag_name, nullptr);
//   special_nullary_startup_flags_[std::string("--") + flag_name] = handler;
//   special_nullary_startup_flags_[std::string("--no") + flag_name] = handler;
// }
// 
// void StartupOptions::RegisterUnaryStartupFlag(const std::string &flag_name) {
//   valid_unary_startup_flags_.insert(std::string("--") + flag_name);
// }
// 
// void StartupOptions::OverrideOptionSourcesKey(const std::string &flag_name,
//                                               const std::string &new_name) {
//   option_sources_key_override_[flag_name] = new_name;
// }
// 
// StartupOptions::StartupOptions(const string &product_name,
//                                const WorkspaceLayout *workspace_layout)
//     : product_name(product_name),
//       ignore_all_rc_files(false),
//       block_for_lock(true),
//       host_jvm_debug(false),
//       autodetect_server_javabase(true),
//       batch(false),
//       batch_cpu_scheduling(false),
//       io_nice_level(-1),
//       shutdown_on_low_sys_mem(false),
//       oom_more_eagerly(false),
//       oom_more_eagerly_threshold(100),
//       write_command_log(true),
//       watchfs(false),
//       fatal_event_bus_exceptions(false),
//       command_port(0),
//       connect_timeout_secs(30),
//       local_startup_timeout_secs(120),
//       have_invocation_policy_(false),
//       client_debug(false),
//       preemptible(false),
//       java_logging_formatter(
//           "com.google.devtools.build.lib.util.SingleLineFormatter"),
//       expand_configs_in_place(true),
//       digest_function(),
//       idle_server_tasks(true),
//       original_startup_options_(std::vector<RcStartupFlag>()),
// #if defined(__APPLE__)
//       macos_qos_class(QOS_CLASS_DEFAULT),
// #endif
//       unlimit_coredumps(false),
//       incompatible_enable_execution_transition(false),
//       windows_enable_symlinks(false) {
//   if (blaze::IsRunningWithinTest()) {
//     output_root = blaze_util::MakeAbsolute(blaze::GetPathEnv("TEST_TMPDIR"));
//     max_idle_secs = 15;
//     BAZEL_LOG(USER) << "$TEST_TMPDIR defined: output root default is '"
//                     << output_root << "' and max_idle_secs default is '"
//                     << max_idle_secs << "'.";
//   } else {
//     output_root = workspace_layout->GetOutputRoot();
//     max_idle_secs = 3 * 3600;
//     BAZEL_LOG(INFO) << "output root is '" << output_root
//                     << "' and max_idle_secs default is '" << max_idle_secs
//                     << "'.";
//   }
// 
// #if defined(_WIN32) || defined(__CYGWIN__)
//   string windows_unix_root = DetectBashAndExportBazelSh();
//   if (!windows_unix_root.empty()) {
//     host_jvm_args.push_back(string("-Dbazel.windows_unix_root=") +
//                             windows_unix_root);
//   }
// #endif  // defined(_WIN32) || defined(__CYGWIN__)
// 
//   const string product_name_lower = GetLowercaseProductName();
//   output_user_root = blaze_util::JoinPath(
//       output_root, "_" + product_name_lower + "_" + GetUserName());
// 
//   // IMPORTANT: Before modifying the statements below please contact a Bazel
//   // core team member that knows the internal procedure for adding/deprecating
//   // startup flags.
//   RegisterNullaryStartupFlag("batch", &batch);
//   RegisterNullaryStartupFlag("batch_cpu_scheduling", &batch_cpu_scheduling);
//   RegisterNullaryStartupFlag("block_for_lock", &block_for_lock);
//   RegisterNullaryStartupFlag("client_debug", &client_debug);
//   RegisterNullaryStartupFlag("preemptible", &preemptible);
//   RegisterNullaryStartupFlag("expand_configs_in_place",
//                              &expand_configs_in_place);
//   RegisterNullaryStartupFlag("fatal_event_bus_exceptions",
//                              &fatal_event_bus_exceptions);
//   RegisterNullaryStartupFlag("host_jvm_debug", &host_jvm_debug);
//   RegisterNullaryStartupFlag("autodetect_server_javabase",
//                              &autodetect_server_javabase);
//   RegisterNullaryStartupFlag("idle_server_tasks", &idle_server_tasks);
//   RegisterNullaryStartupFlag("incompatible_enable_execution_transition",
//                              &incompatible_enable_execution_transition);
//   RegisterNullaryStartupFlag("shutdown_on_low_sys_mem",
//                              &shutdown_on_low_sys_mem);
//   RegisterNullaryStartupFlagNoRc("ignore_all_rc_files", &ignore_all_rc_files);
//   RegisterNullaryStartupFlag("unlimit_coredumps", &unlimit_coredumps);
//   RegisterNullaryStartupFlag("watchfs", &watchfs);
//   RegisterNullaryStartupFlag("write_command_log", &write_command_log);
//   RegisterNullaryStartupFlag("windows_enable_symlinks",
//                              &windows_enable_symlinks);
//   RegisterUnaryStartupFlag("command_port");
//   RegisterUnaryStartupFlag("connect_timeout_secs");
//   RegisterUnaryStartupFlag("local_startup_timeout_secs");
//   RegisterUnaryStartupFlag("digest_function");
//   RegisterUnaryStartupFlag("unix_digest_hash_attribute_name");
//   RegisterUnaryStartupFlag("server_javabase");
//   RegisterUnaryStartupFlag("host_jvm_args");
//   RegisterUnaryStartupFlag("host_jvm_profile");
//   RegisterUnaryStartupFlag("invocation_policy");
//   RegisterUnaryStartupFlag("io_nice_level");
//   RegisterUnaryStartupFlag("install_base");
//   RegisterUnaryStartupFlag("macos_qos_class");
//   RegisterUnaryStartupFlag("max_idle_secs");
//   RegisterUnaryStartupFlag("output_base");
//   RegisterUnaryStartupFlag("output_user_root");
//   RegisterUnaryStartupFlag("server_jvm_out");
//   RegisterUnaryStartupFlag("failure_detail_out");
// }
// 
// StartupOptions::~StartupOptions() {}
// 
// string StartupOptions::GetLowercaseProductName() const {
//   string lowercase_product_name = product_name;
//   blaze_util::ToLower(&lowercase_product_name);
//   return lowercase_product_name;
// }
// 
// bool StartupOptions::IsUnary(const string &arg) const {
//   std::string::size_type i = arg.find_first_of('=');
//   if (i == std::string::npos) {
//     return valid_unary_startup_flags_.find(arg) !=
//            valid_unary_startup_flags_.end();
//   } else {
//     return valid_unary_startup_flags_.find(arg.substr(0, i)) !=
//            valid_unary_startup_flags_.end();
//   }
// }
// 
// bool StartupOptions::MaybeCheckValidNullary(const string &arg, bool *result,
//                                             std::string *error) const {
//   std::string::size_type i = arg.find_first_of('=');
//   if (i == std::string::npos) {
//     *result = all_nullary_startup_flags_.find(arg) !=
//               all_nullary_startup_flags_.end();
//     return true;
//   }
//   std::string f = arg.substr(0, i);
//   if (all_nullary_startup_flags_.find(f) == all_nullary_startup_flags_.end()) {
//     *result = false;
//     return true;
//   }
// 
//   blaze_util::StringPrintf(
//       error, "In argument '%s': option '%s' does not take a value.",
//       arg.c_str(), f.c_str());
//   return false;
// }
// 
// void StartupOptions::AddExtraOptions(vector<string> *result) const {
//   if (incompatible_enable_execution_transition) {
//     result->push_back("--incompatible_enable_execution_transition");
//   } else {
//     result->push_back("--noincompatible_enable_execution_transition");
//   }
// }
// 
// blaze_exit_code::ExitCode StartupOptions::ProcessArg(
//       const string &argstr, const string &next_argstr, const string &rcfile,
//       bool *is_space_separated, string *error) {
//   // We have to parse a specific option syntax, so GNU getopts won't do.  All
//   // options begin with "--" or "-". Values are given together with the option
//   // delimited by '=' or in the next option.
//   const char* arg = argstr.c_str();
//   const char* next_arg = next_argstr.empty() ? NULL : next_argstr.c_str();
//   const char* value = NULL;
// 
//   bool is_nullary;
//   if (!MaybeCheckValidNullary(argstr, &is_nullary, error)) {
//     *is_space_separated = false;
//     return blaze_exit_code::BAD_ARGV;
//   }
// 
//   if (is_nullary) {
//     // 'enabled' is true if 'argstr' is "--foo", and false if it's "--nofoo".
//     bool enabled = (argstr.compare(0, 4, "--no") != 0);
//     if (no_rc_nullary_startup_flags_.find(argstr) !=
//         no_rc_nullary_startup_flags_.end()) {
//       // no_rc_nullary_startup_flags_ are forbidden in .bazelrc files.
//       if (!rcfile.empty()) {
//         *error = std::string("Can't specify ") + argstr + " in the " +
//                  GetRcFileBaseName() + " file.";
//         return blaze_exit_code::BAD_ARGV;
//       }
//     }
//     if (special_nullary_startup_flags_.find(argstr) !=
//         special_nullary_startup_flags_.end()) {
//       // 'argstr' is either "--foo" or "--nofoo", and the map entry is the
//       // lambda that handles setting the flag's value.
//       special_nullary_startup_flags_[argstr](enabled);
//     } else {
//       // 'argstr' is either "--foo" or "--nofoo", and the map entry is the
//       // pointer to the bool storing the flag's value.
//       *all_nullary_startup_flags_[argstr] = enabled;
//     }
//     // Use the key "foo" for 'argstr' of "--foo" / "--nofoo", unless there's an
//     // overridden name we must use.
//     std::string key = argstr.substr(enabled ? 2 : 4);
//     if (option_sources_key_override_.find(key) !=
//         option_sources_key_override_.end()) {
//       key = option_sources_key_override_[key];
//     }
//     option_sources[key] = rcfile;
//     *is_space_separated = false;
//     return blaze_exit_code::SUCCESS;
//   }
// 
//   if ((value = GetUnaryOption(arg, next_arg, "--output_base")) != NULL) {
//     output_base = blaze_util::Path(blaze::AbsolutePathFromFlag(value));
//     option_sources["output_base"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg,
//                                      "--install_base")) != NULL) {
//     install_base = blaze::AbsolutePathFromFlag(value);
//     option_sources["install_base"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg,
//                                      "--output_user_root")) != NULL) {
//     output_user_root = blaze::AbsolutePathFromFlag(value);
//     option_sources["output_user_root"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg,
//                                      "--server_jvm_out")) != NULL) {
//     server_jvm_out = blaze_util::Path(blaze::AbsolutePathFromFlag(value));
//     option_sources["server_jvm_out"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--failure_detail_out")) !=
//              NULL) {
//     failure_detail_out = blaze_util::Path(blaze::AbsolutePathFromFlag(value));
//     option_sources["failure_detail_out"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--host_jvm_profile")) !=
//              NULL) {
//     host_jvm_profile = value;
//     option_sources["host_jvm_profile"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--server_javabase")) !=
//              NULL) {
//     // TODO(bazel-team): Consider examining the javabase and re-execing in case
//     // of architecture mismatch.
//     explicit_server_javabase_ =
//         blaze_util::Path(blaze::AbsolutePathFromFlag(value));
//     option_sources["server_javabase"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--host_jvm_args")) !=
//              NULL) {
//     host_jvm_args.push_back(value);
//     option_sources["host_jvm_args"] = rcfile;  // NB: This is incorrect
//   } else if ((value = GetUnaryOption(arg, next_arg, "--io_nice_level")) !=
//              NULL) {
//     if (!blaze_util::safe_strto32(value, &io_nice_level) ||
//         io_nice_level > 7) {
//       blaze_util::StringPrintf(error,
//           "Invalid argument to --io_nice_level: '%s'. Must not exceed 7.",
//           value);
//       return blaze_exit_code::BAD_ARGV;
//     }
//     option_sources["io_nice_level"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--max_idle_secs")) !=
//              NULL) {
//     if (!blaze_util::safe_strto32(value, &max_idle_secs) ||
//         max_idle_secs < 0) {
//       blaze_util::StringPrintf(error,
//           "Invalid argument to --max_idle_secs: '%s'.", value);
//       return blaze_exit_code::BAD_ARGV;
//     }
//     option_sources["max_idle_secs"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--macos_qos_class")) !=
//              NULL) {
//     // We parse the value of this flag on all platforms even if it is
//     // macOS-specific to ensure that rc files mentioning it are valid.
//     if (strcmp(value, "user-interactive") == 0) {
// #if defined(__APPLE__)
//       macos_qos_class = QOS_CLASS_USER_INTERACTIVE;
// #endif
//     } else if (strcmp(value, "user-initiated") == 0) {
// #if defined(__APPLE__)
//       macos_qos_class = QOS_CLASS_USER_INITIATED;
// #endif
//     } else if (strcmp(value, "default") == 0) {
// #if defined(__APPLE__)
//       macos_qos_class = QOS_CLASS_DEFAULT;
// #endif
//     } else if (strcmp(value, "utility") == 0) {
// #if defined(__APPLE__)
//       macos_qos_class = QOS_CLASS_UTILITY;
// #endif
//     } else if (strcmp(value, "background") == 0) {
// #if defined(__APPLE__)
//       macos_qos_class = QOS_CLASS_BACKGROUND;
// #endif
//     } else {
//       blaze_util::StringPrintf(
//           error, "Invalid argument to --macos_qos_class: '%s'.", value);
//       return blaze_exit_code::BAD_ARGV;
//     }
//     option_sources["macos_qos_class"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg,
//                                      "--connect_timeout_secs")) != NULL) {
//     if (!blaze_util::safe_strto32(value, &connect_timeout_secs) ||
//         connect_timeout_secs < 1 || connect_timeout_secs > 120) {
//       blaze_util::StringPrintf(error,
//           "Invalid argument to --connect_timeout_secs: '%s'.\n"
//           "Must be an integer between 1 and 120.\n",
//           value);
//       return blaze_exit_code::BAD_ARGV;
//     }
//     option_sources["connect_timeout_secs"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg,
//                                      "--local_startup_timeout_secs")) != NULL) {
//     if (!blaze_util::safe_strto32(value, &local_startup_timeout_secs) ||
//         local_startup_timeout_secs < 1) {
//       blaze_util::StringPrintf(
//           error,
//           "Invalid argument to --local_startup_timeout_secs: '%s'.\n"
//           "Must be a positive integer.\n",
//           value);
//       return blaze_exit_code::BAD_ARGV;
//     }
//     option_sources["local_startup_timeout_secs"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--digest_function")) !=
//              NULL) {
//     digest_function = value;
//     option_sources["digest_function"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg,
//                                      "--unix_digest_hash_attribute_name")) !=
//              NULL) {
//     unix_digest_hash_attribute_name = value;
//     option_sources["unix_digest_hash_attribute_name"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--command_port")) !=
//              NULL) {
//     if (!blaze_util::safe_strto32(value, &command_port) ||
//         command_port < 0 || command_port > 65535) {
//       blaze_util::StringPrintf(error,
//           "Invalid argument to --command_port: '%s'.\n"
//           "Must be a valid port number or 0.\n",
//           value);
//       return blaze_exit_code::BAD_ARGV;
//     }
//     option_sources["command_port"] = rcfile;
//   } else if ((value = GetUnaryOption(arg, next_arg, "--invocation_policy")) !=
//              NULL) {
//     if (!have_invocation_policy_) {
//       have_invocation_policy_ = true;
//       invocation_policy = value;
//       option_sources["invocation_policy"] = rcfile;
//     } else {
//       *error = "The startup flag --invocation_policy cannot be specified "
//           "multiple times.";
//       return blaze_exit_code::BAD_ARGV;
//     }
//   } else {
//     bool extra_argument_processed;
//     blaze_exit_code::ExitCode process_extra_arg_exit_code = process_arg_extra(
//         arg, next_arg, rcfile, &value, &extra_argument_processed, error);
//     if (process_extra_arg_exit_code != blaze_exit_code::SUCCESS) {
//       return process_extra_arg_exit_code;
//     }
//     if (!extra_argument_processed) {
//       blaze_util::StringPrintf(
//           error,
//           "Unknown startup option: '%s'.\n"
//           "  For more info, run '%s help startup_options'.",
//           arg, GetLowercaseProductName().c_str());
//       return blaze_exit_code::BAD_ARGV;
//     }
//   }
// 
//   *is_space_separated = ((value == next_arg) && (value != NULL));
//   return blaze_exit_code::SUCCESS;
// }
// 
// blaze_exit_code::ExitCode StartupOptions::ProcessArgs(
//     const std::vector<RcStartupFlag>& rcstartup_flags,
//     std::string *error) {
//   std::vector<RcStartupFlag>::size_type i = 0;
//   while (i < rcstartup_flags.size()) {
//     bool is_space_separated = false;
//     const std::string next_value =
//         (i == rcstartup_flags.size() - 1) ? "" : rcstartup_flags[i + 1].value;
//     const blaze_exit_code::ExitCode process_arg_exit_code =
//         ProcessArg(rcstartup_flags[i].value, next_value,
//                    rcstartup_flags[i].source, &is_space_separated, error);
//     // Store the provided option in --flag(=value)? form. Store these before
//     // propagating any error code, since we want to have the correct
//     // information for the output. The fact that the options aren't parseable
//     // doesn't matter for this step.
//     if (is_space_separated) {
//       const std::string combined_value =
//           rcstartup_flags[i].value + "=" + next_value;
//       original_startup_options_.push_back(
//           RcStartupFlag(rcstartup_flags[i].source, combined_value));
//       i += 2;
//     } else {
//       original_startup_options_.push_back(
//           RcStartupFlag(rcstartup_flags[i].source, rcstartup_flags[i].value));
//       i++;
//     }
// 
//     if (process_arg_exit_code != blaze_exit_code::SUCCESS) {
//       return process_arg_exit_code;
//     }
//   }
//   return blaze_exit_code::SUCCESS;
// }
// 
// blaze_util::Path StartupOptions::GetSystemJavabase() const {
//   return blaze_util::Path(blaze::GetSystemJavabase());
// }
// 
// blaze_util::Path StartupOptions::GetEmbeddedJavabase() const {
//   blaze_util::Path bundled_jre_path = blaze_util::Path(
//       blaze_util::JoinPath(install_base, "embedded_tools/jdk"));
//   if (blaze_util::CanExecuteFile(
//           bundled_jre_path.GetRelative(GetJavaBinaryUnderJavabase()))) {
//     return bundled_jre_path;
//   }
//   return blaze_util::Path();
// }
// 
// std::pair<blaze_util::Path, StartupOptions::JavabaseType>
// StartupOptions::GetServerJavabaseAndType() const {
//   // 1) Allow overriding the server_javabase via --server_javabase.
//   if (!explicit_server_javabase_.IsEmpty()) {
//     return std::pair<blaze_util::Path, JavabaseType>(explicit_server_javabase_,
//                                                      JavabaseType::EXPLICIT);
//   }
//   if (default_server_javabase_.first.IsEmpty()) {
//     blaze_util::Path bundled_jre_path = GetEmbeddedJavabase();
//     if (!bundled_jre_path.IsEmpty()) {
//       // 2) Use a bundled JVM if we have one.
//       default_server_javabase_ = std::pair<blaze_util::Path, JavabaseType>(
//           bundled_jre_path, JavabaseType::EMBEDDED);
//     } else if (!autodetect_server_javabase) {
//       BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR)
//           << "Could not find embedded or explicit server javabase, and "
//              "--noautodetect_server_javabase is set.";
//     } else {
//       // 3) Otherwise fall back to using the default system JVM.
//       blaze_util::Path system_javabase = GetSystemJavabase();
//       if (system_javabase.IsEmpty()) {
//         BAZEL_DIE(blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR)
//             << "Could not find system javabase. Ensure JAVA_HOME is set, or "
//                "javac is on your PATH.";
//       }
//       default_server_javabase_ = std::pair<blaze_util::Path, JavabaseType>(
//           system_javabase, JavabaseType::SYSTEM);
//     }
//   }
//   return default_server_javabase_;
// }
// 
// blaze_util::Path StartupOptions::GetServerJavabase() const {
//   return GetServerJavabaseAndType().first;
// }
// 
// blaze_util::Path StartupOptions::GetExplicitServerJavabase() const {
//   return explicit_server_javabase_;
// }
// 
// blaze_util::Path StartupOptions::GetJvm() const {
//   auto javabase_and_type = GetServerJavabaseAndType();
//   blaze_exit_code::ExitCode sanity_check_code =
//       SanityCheckJavabase(javabase_and_type.first, javabase_and_type.second);
//   if (sanity_check_code != blaze_exit_code::SUCCESS) {
//     exit(sanity_check_code);
//   }
//   return javabase_and_type.first.GetRelative(GetJavaBinaryUnderJavabase());
// }
// 
// // Prints an appropriate error message and returns an appropriate error exit
// // code for a server javabase which failed sanity checks.
// static blaze_exit_code::ExitCode BadServerJavabaseError(
//     StartupOptions::JavabaseType javabase_type,
//     const std::map<string, string> &option_sources) {
//   switch (javabase_type) {
//     case StartupOptions::JavabaseType::EXPLICIT: {
//       auto source = option_sources.find("server_javabase");
//       string rc_file;
//       if (source != option_sources.end() && !source->second.empty()) {
//         rc_file = source->second;
//       }
//       BAZEL_LOG(ERROR)
//           << "  The java path was specified by a '--server_javabase' option " +
//                  (rc_file.empty() ? "on the command line" : "in " + rc_file);
//       return blaze_exit_code::BAD_ARGV;
//     }
//     case StartupOptions::JavabaseType::EMBEDDED:
//       BAZEL_LOG(ERROR) << "  Internal error: embedded JDK fails sanity check.";
//       return blaze_exit_code::INTERNAL_ERROR;
//     case StartupOptions::JavabaseType::SYSTEM:
//       return blaze_exit_code::LOCAL_ENVIRONMENTAL_ERROR;
//     default:
//       BAZEL_LOG(ERROR)
//           << "  Internal error: server javabase type was not initialized.";
//       // Fall through.
//   }
//   return blaze_exit_code::INTERNAL_ERROR;
// }
// 
// blaze_exit_code::ExitCode StartupOptions::SanityCheckJavabase(
//     const blaze_util::Path &javabase,
//     StartupOptions::JavabaseType javabase_type) const {
//   blaze_util::Path java_program =
//       javabase.GetRelative(GetJavaBinaryUnderJavabase());
//   if (!blaze_util::CanExecuteFile(java_program)) {
//     if (!blaze_util::PathExists(java_program)) {
//       BAZEL_LOG(ERROR) << "Couldn't find java at '"
//                        << java_program.AsPrintablePath() << "'.";
//     } else {
//       string err = blaze_util::GetLastErrorString();
//       BAZEL_LOG(ERROR) << "Java at '" << java_program.AsPrintablePath()
//                        << "' exists but is not executable: " << err;
//     }
//     return BadServerJavabaseError(javabase_type, option_sources);
//   }
//   if (  // If the full JDK is installed
//       blaze_util::CanReadFile(javabase.GetRelative("jre/lib/rt.jar")) ||
//       // If just the JRE is installed
//       blaze_util::CanReadFile(javabase.GetRelative("lib/rt.jar")) ||
//       // rt.jar does not exist in java 9+ so check for java instead
//       blaze_util::CanReadFile(javabase.GetRelative("bin/java")) ||
//       blaze_util::CanReadFile(javabase.GetRelative("bin/java.exe"))) {
//     return blaze_exit_code::SUCCESS;
//   }
//   BAZEL_LOG(ERROR) << "Problem with java installation: couldn't find/access "
//                       "rt.jar or java in "
//                    << javabase.AsPrintablePath();
//   return BadServerJavabaseError(javabase_type, option_sources);
// }
// 
// blaze_util::Path StartupOptions::GetExe(const blaze_util::Path &jvm,
//                                         const string &jar_path) const {
//   return jvm;
// }
// 
// void StartupOptions::AddJVMArgumentPrefix(const blaze_util::Path &javabase,
//                                           std::vector<string> *result) const {}
// 
// void StartupOptions::AddJVMArgumentSuffix(
//     const blaze_util::Path &real_install_dir, const string &jar_path,
//     std::vector<string> *result) const {
//   result->push_back("-jar");
//   result->push_back(real_install_dir.GetRelative(jar_path).AsJvmArgument());
// }
// 
// blaze_exit_code::ExitCode StartupOptions::AddJVMArguments(
//     const blaze_util::Path &server_javabase, std::vector<string> *result,
//     const vector<string> &user_options, string *error) const {
//   AddJVMLoggingArguments(result);
// 
//   // Disable the JVM's own unlimiting of file descriptors.  We do this
//   // ourselves in blaze.cc so we want our setting to propagate to the JVM.
//   //
//   // The reason to do this is that the JVM's unlimiting is suboptimal on
//   // macOS.  Under that platform, the JVM limits the open file descriptors
//   // to the OPEN_MAX constant... which is much lower than the per-process
//   // kernel allowed limit of kern.maxfilesperproc (which is what we set
//   // ourselves to).
//   result->push_back("-XX:-MaxFDLimit");
// 
//   return AddJVMMemoryArguments(server_javabase, result, user_options, error);
// }
// 
// static std::string GetSimpleLogHandlerProps(
//     const blaze_util::Path &java_log,
//     const std::string &java_logging_formatter) {
//   return "handlers=com.google.devtools.build.lib.util.SimpleLogHandler\n"
//          ".level=INFO\n"
//          "com.google.devtools.build.lib.util.SimpleLogHandler.level=INFO\n"
//          "com.google.devtools.build.lib.util.SimpleLogHandler.prefix=" +
//          java_log.AsJvmArgument() +
//          "\n"
//          "com.google.devtools.build.lib.util.SimpleLogHandler.limit=1024000\n"
//          "com.google.devtools.build.lib.util.SimpleLogHandler.total_limit="
//          "20971520\n"  // 20 MB.
//          "com.google.devtools.build.lib.util.SimpleLogHandler.formatter=" +
//          java_logging_formatter + "\n";
// }
// 
// void StartupOptions::AddJVMLoggingArguments(std::vector<string> *result) const {
//   // Configure logging
//   const blaze_util::Path propFile =
//       output_base.GetRelative("javalog.properties");
//   const blaze_util::Path java_log = output_base.GetRelative("java.log");
//   const std::string loggingProps =
//       GetSimpleLogHandlerProps(java_log, java_logging_formatter);
// 
//   if (!blaze_util::WriteFile(loggingProps, propFile)) {
//     perror(
//         ("Couldn't write logging file " + propFile.AsPrintablePath()).c_str());
//   } else {
//     result->push_back("-Djava.util.logging.config.file=" +
//                       propFile.AsJvmArgument());
//     result->push_back(
//         "-Dcom.google.devtools.build.lib.util.LogHandlerQuerier.class="
//         "com.google.devtools.build.lib.util.SimpleLogHandler$HandlerQuerier");
//   }
// }
// 
// blaze_exit_code::ExitCode StartupOptions::AddJVMMemoryArguments(
//     const blaze_util::Path &, std::vector<string> *, const vector<string> &,
//     string *) const {
//   return blaze_exit_code::SUCCESS;
// }
