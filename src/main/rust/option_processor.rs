// #include <algorithm>
// #include <cassert>
// #include <iterator>
// #include <list>
// #include <memory>
// #include <set>
// #include <sstream>
// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>
// #include <string>
// #include <utility>
// #include <utility>
// #include <vector>

// #include "src/main/cpp/blaze_util.h"
// #include "src/main/cpp/blaze_util_platform.h"
// #include "src/main/cpp/rc_file.h"
// #include "src/main/cpp/startup_options.h"
// #include "src/main/cpp/util/exit_code.h"
// #include "src/main/cpp/util/file.h"
// #include "src/main/cpp/util/logging.h"
// #include "src/main/cpp/util/path.h"
// #include "src/main/cpp/util/path_platform.h"
// #include "src/main/cpp/util/strings.h"
// #include "src/main/cpp/workspace_layout.h"

// // On OSX, there apparently is no header that defines this.
// #ifndef environ
// extern char **environ;
// #endif

// using std::map;
// using std::set;
// using std::string;
// using std::vector;

use crate::bazel_util::is_arg;
use crate::exit_code::{self, ExitCode};
use crate::rc_file::RcFile;
use crate::StartupOptions;
use slog::o;
use std::collections::HashSet;

// constexpr char WorkspaceLayout::WorkspacePrefix[];
const RC_BASENAME: &str = ".bazelrc";
// static std::vector<std::string> GetProcessedEnv();

/// Broken down structure of the command line into logical components. The raw
/// arguments should not be referenced after this structure exists. This
/// breakdown should suffice to access the parts of the command line that the
/// client cares about, notably the binary and startup startup options.
#[derive(Debug, PartialEq)]
pub struct CommandLine {
    path_to_binary: String,
    startup_args: Vec<String>,
    command: String,
    command_args: Vec<String>,
}

impl CommandLine {
    fn new(
        path_to_binary: String,
        startup_args: Vec<String>,
        command: String,
        command_args: Vec<String>,
    ) -> Self {
        Self {
            path_to_binary,
            startup_args,
            command,
            command_args,
        }
    }
}

/// This class is responsible for parsing the command line of the Blaze binary,
/// parsing blazerc files, and putting together the command that should be sent
/// to the server.
pub struct OptionProcessor {
    logger: slog::Logger,

    //   // Parse a command line and the appropriate blazerc files and stores the
    //   // results. This should be invoked only once per OptionProcessor object.
    //   blaze_exit_code::ExitCode parse_options(const std::vector<std::string>& args, const std::string& workspace, const std::string& cwd, std::string* error);
    //
    //   // Get the Blaze command to be executed.
    //   // Returns an empty string if no command was found on the command line.
    //   std::string get_command() const;
    //
    //   // Returns the underlying StartupOptions object with parsed values. Must
    //   // only be called after parse_options.
    //   virtual StartupOptions* get_parsed_startup_options() const;
    //
    //   // Prints a message about the origin of startup options. This should be called
    //   // if the server is not started or called, in case the options are related to
    //   // the failure. Otherwise, the server will handle any required logging.
    //   void print_startup_options_provenance_message() const;
    //
    //   // Constructs all synthetic command args that should be passed to the
    //   // server to configure blazerc options and client environment.
    //   static std::vector<std::string> get_blazerc_and_env_command_args(
    //       const std::string& cwd,
    //       const std::vector<RcFile*>& blazercs,
    //       const std::vector<std::string>& env);
    //
    //   // Finds and parses the appropriate RcFiles:
    //   //   - system rc (unless --nosystem_rc)
    //   //   - workspace, %workspace%/.bazelrc (unless --noworkspace_rc, or we aren't
    //   //     in a workspace directory, indicated by an empty workspace parameter)
    //   //   - user, $HOME/.bazelrc (unless --nohome_rc)
    //   //   - command-line provided, if a value is passed with --bazelrc.
    //   virtual blaze_exit_code::ExitCode get_rc_files(
    //       const WorkspaceLayout* workspace_layout, const std::string& workspace,
    //       const std::string& cwd, const CommandLine* cmd_line,
    //       std::vector<std::unique_ptr<RcFile>>* result_rc_files,
    //       std::string* error) const;
    /// An ordered list of command args that contain information about the
    /// execution environment and the flags passed via the bazelrc files.
    blazerc_and_env_command_args: Vec<String>,

    /// The command line constructed after calling parse_options.
    cmd_line: Option<CommandLine>,

    //   const WorkspaceLayout* workspace_layout_;
    /// The StartupOptions object defining the startup options which are accepted,
    /// and, after parse_options has been called, their values.
    startup_options: StartupOptions,

    /// Whether or not parse_options has been called.
    parse_options_called: bool,

    /// Path to the system-wide bazelrc configuration file.
    /// This is configurable for testing purposes only.
    system_bazelrc_path: &'static str,
}

impl OptionProcessor {
    pub fn new(
        logger: slog::Logger,

        //     const WorkspaceLayout* workspace_layout, FIXME do we need this? it looks like a namespace and not a class
        default_startup_options: StartupOptions, // std::unique_ptr<StartupOptions> default_startup_options
    ) -> Self {
        Self {
            logger,
            // workspace_layout_(workspace_layout),
            blazerc_and_env_command_args: Default::default(),
            cmd_line: None,

            // startup_options_(std::move(default_startup_options))
            startup_options: default_startup_options,

            parse_options_called: false,

            // FIXME see src/main/cpp/BUILD for bazel injected value
            // system_bazelrc_path_(BAZEL_SYSTEM_BAZELRC_PATH)
            system_bazelrc_path: "/etc/bazel.bazelrc",
        }
    }

    //   OptionProcessor(const WorkspaceLayout* workspace_layout,
    //                   std::unique_ptr<StartupOptions> default_startup_options,
    //                   const std::string& system_bazelrc_path);

    // OptionProcessor::OptionProcessor(
    //     const WorkspaceLayout* workspace_layout,
    //     std::unique_ptr<StartupOptions> default_startup_options,
    //     const std::string& system_bazelrc_path)
    //     : workspace_layout_(workspace_layout),
    //       startup_options_(std::move(default_startup_options)),
    //       parse_options_called_(false),
    //       system_bazelrc_path_(system_bazelrc_path) {}

    // blaze_exit_code::ExitCode ParseStartupOptions(const std::vector<RcFile*>& rc_files, std::string* error);

    /// Splits the arguments of a command line invocation.
    ///
    /// For instance:
    /// output/bazel --foo --bar=42 --bar blah build --myflag value :mytarget
    ///
    /// returns a CommandLine structure with the following values:
    /// result.path_to_binary = "output/bazel"
    /// result.startup_args = {"--foo", "--bar=42", "--bar=blah"}
    /// result.command = "build"
    /// result.command_args = {"--myflag", "value", ":mytarget"}
    ///
    /// Note that result.startup_args is guaranteed to contain only valid
    /// startup options (w.r.t. StartupOptions::is_unary and
    /// StartupOptions::IsNullary) and unary startup args of the form '--bar blah'
    /// are rewritten as '--bar=blah' for uniformity.
    /// In turn, the command and command args are not rewritten nor validated.
    ///
    /// If the method fails then error will contain the cause, otherwise error
    /// remains untouched.
    pub fn split_command_line(&self, args: Vec<String>) -> Result<CommandLine, String> {
        if args.is_empty() {
            return Err(String::from("Unable to split command line, args is empty"));
        }

        let path_to_binary = args[0].clone();

        // Process the startup options.
        let mut startup_args = Vec::<String>::new();
        let mut i = 1;
        while i < args.len() && is_arg(&args[i]) {
            let current_arg = args[i].clone();

            // If the current argument is a valid nullary startup option such as
            // --master_bazelrc or --nomaster_bazelrc proceed to examine the next
            // argument.
            if self.startup_options.check_valid_nullary(&current_arg)? {
                startup_args.push(current_arg);
                i += 1;
            } else if self.startup_options.is_unary(&current_arg) {
                // If the current argument is a valid unary startup option such as
                // --bazelrc there are two cases to consider.

                // The option is of the form '--bazelrc=value', hence proceed to
                // examine the next argument.
                match current_arg.find('=') {
                    Some(_) => {
                        startup_args.push(current_arg.clone());
                        i += 1;
                    }
                    None => {
                        // Otherwise, the option is of the form '--bazelrc value', hence
                        // skip the next argument and proceed to examine the argument located
                        // two places after.
                        if i + 1 >= args.len() {
                            return Err(format!(
                                "Startup option '{0}' expects a value.\nUsage: '{0}=somevalue' or '{0} somevalue'.\n  For more info, run 'bazel help startup_options'.",
                                current_arg,
                            ));
                        }
                        // In this case we transform it to the form '--bazelrc=value'.
                        startup_args.push(format!("{}={}", current_arg, args[i + 1]));
                        i += 2;
                    }
                };
            } else {
                // If the current argument is not a valid unary or nullary startup option then fail.
                return Err(format!("Unknown startup option: '{}'.\n  For more info, run 'bazel help startup_options'.", current_arg));
            }
        }

        // The command is the arg right after the startup options.
        if i == args.len() {
            return Ok(CommandLine {
                path_to_binary,
                startup_args,
                command: "".to_string(),
                command_args: Default::default(),
            });
        }

        let command = args[i].clone();

        // The rest are the command arguments.
        let mut command_args = Vec::<String>::new();
        // FIXME there must be a oneline to do this
        for a in args[i + 1..].iter() {
            command_args.push(a.to_string());
        }

        Ok(CommandLine {
            path_to_binary,
            startup_args,
            command,
            command_args,
        })
    }

    // // TODO(#4502) Consider simplifying result_rc_files to a vector of RcFiles, no
    // // unique_ptrs.
    // blaze_exit_code::ExitCode OptionProcessor::get_rc_files(@@ARGS@@) const {
    fn get_rc_files(
        &self,
        //     const WorkspaceLayout* workspace_layout,
        workspace: &str,       // const std::string& workspace,
        cwd: &str,             //     const std::string& cwd,
        cmd_line: CommandLine, // const CommandLine* cmd_line,
    ) -> Result<Vec<RcFile>, exit_code::Error> {
        let mut rc_files = Vec::<String>::new();
        //   std::vector<std::string> rc_files;

        //   // Get the system rc (unless --nosystem_rc).
        //   if (search_nullary_option(cmd_line->startup_args, "system_rc", true)) {
        //     // MakeAbsoluteAndResolveEnvvars will standardize the form of the
        //     // provided path. This also means we accept relative paths, which is
        //     // is convenient for testing.
        //     const std::string system_rc = blaze_util::MakeAbsoluteAndResolveEnvvars(system_bazelrc_path_);
        //     rc_files.push_back(system_rc);
        //   }

        //   // Get the workspace rc: %workspace%/.bazelrc (unless --noworkspace_rc), but
        //   // only if we are in a workspace: invoking commands like "help" from outside
        //   // a workspace should work.
        //   if (!workspace.empty() && search_nullary_option(cmd_line->startup_args, "workspace_rc", true)) {
        //     const std::string workspaceRcFile = blaze_util::JoinPath(workspace, K_RC_BASENAME);
        //     rc_files.push_back(workspaceRcFile);
        //   }
        //
        //   // Get the user rc: $HOME/.bazelrc (unless --nohome_rc)
        //   if (search_nullary_option(cmd_line->startup_args, "home_rc", true)) {
        //     const std::string home = blaze::GetHomeDir();
        //     if (!home.empty()) {
        //       rc_files.push_back(blaze_util::JoinPath(home, K_RC_BASENAME));
        let p = dirs::home_dir().unwrap().join(".bazelrc");
        rc_files.push(String::from(p.to_str().unwrap()));
        //     }
        //   }

        //   // Get the command-line provided rc, passed as --bazelrc or nothing if the
        //   // flag is absent.
        //   const char* cmd_line_rc_file = search_unary_option(cmd_line->startup_args, "--bazelrc", true);
        //   if (cmd_line_rc_file != nullptr) {
        //     string absolute_cmd_line_rc = blaze::absolute_path_from_flag(cmd_line_rc_file);
        //     // Unlike the previous 3 paths, where we ignore it if the file does not
        //     // exist or is unreadable, since this path is explicitly passed, this is an
        //     // error. Check this condition here.
        //     if (!blaze_util::CanReadFile(absolute_cmd_line_rc)) {
        //       BAZEL_LOG(ERROR) << "Error: Unable to read .bazelrc file '" << absolute_cmd_line_rc << "'.";
        //       return blaze_exit_code::BAD_ARGV;
        //     }
        //     rc_files.push_back(absolute_cmd_line_rc);
        //   }

        //   // Log which files we're looking for before removing duplicates and
        //   // non-existent files, so that this can serve to debug why a certain file is
        //   // not being read. The final files which are read will be logged as they are
        //   // parsed, and can be found using --announce_rc.
        //   std::string joined_rcs;
        //   blaze_util::JoinStrings(rc_files, ',', &joined_rcs);
        //   BAZEL_LOG(INFO) << "Looking for the following rc files: " << joined_rcs;

        //   // It's possible that workspace == home, that files are symlinks for each
        //   // other, or that the --bazelrc flag is a duplicate. Dedupe them to minimize
        //   // the likelihood of repeated options. Since bazelrcs can include one another,
        //   // this isn't sufficient to prevent duplicate options, so we also warn if we
        //   // discover duplicate loads later. This also has the effect of removing paths
        //   // that don't point to real files.
        //   rc_files = internal::DedupeBlazercPaths(rc_files);

        let mut result_rc_files = Vec::<RcFile>::new();

        //   std::set<std::string> read_files_canonical_paths;
        let read_files_canonical_paths = HashSet::<String>::new();

        //   // Parse these potential files, in priority order;
        for top_level_bazelrc_path in rc_files {
            //   for (const std::string& top_level_bazelrc_path : rc_files) {

            let parsed_rc = RcFile::new(
                self.logger.new(o!("component" => "rc_parser")),
                top_level_bazelrc_path.as_str(),
                workspace,
            )
            .map_err(|e| exit_code::Error::new(ExitCode::BadArgv, String::from("parse error")))?;

            //     // Check that none of the rc files loaded this time are duplicate.
            //     const auto& sources = parsed_rc->canonical_source_paths();
            //     internal::WarnAboutDuplicateRcFiles(read_files_canonical_paths, sources);
            //     read_files_canonical_paths.insert(sources.begin(), sources.end());

            result_rc_files.push(parsed_rc);
        }

        //   // Provide a warning for any old file that might have been missed with the new
        //   // expectations. This compares "canonical" paths to one another, so should not
        //   // require additional transformation.
        //   // TODO(b/36168162): Remove this warning along with
        //   // internal::GetOldRcPaths and internal::FindLegacyUserBazelrc after
        //   // the transition period has passed.
        //   const std::set<std::string> old_files = internal::GetOldRcPaths(
        //       workspace_layout, workspace, cwd, cmd_line->path_to_binary,
        //       cmd_line->startup_args, internal::FindSystemWideRc(system_bazelrc_path_));

        //   std::vector<std::string> lost_files = internal::GetLostFiles(old_files, read_files_canonical_paths);
        //   if (!lost_files.empty()) {
        //     std::string joined_lost_rcs;
        //     blaze_util::JoinStrings(lost_files, '\n', &joined_lost_rcs);
        //     BAZEL_LOG(WARNING)
        //         << "The following rc files are no longer being read, please transfer "
        //            "their contents or import their path into one of the standard rc "
        //            "files:\n"
        //         << joined_lost_rcs;
        //   }

        Ok(result_rc_files)
    }

    // blaze_exit_code::ExitCode OptionProcessor::parse_options(@@ARGS@@, string* error) {
    pub fn parse_options(
        &mut self,
        args: Vec<String>, // const vector<string>& args
        workspace: &str,   // const string& workspace
        cwd: &str,         // const string& cwd
    ) -> Result<(), exit_code::Error> {
        assert!(!self.parse_options_called);
        self.parse_options_called = true;

        let cmd_line = match self.split_command_line(args) {
            Ok(x) => x,
            Err(e) => return Err(exit_code::Error::new(exit_code::ExitCode::BadArgv, e)),
        };

        //   // Read the rc files, unless --ignore_all_rc_files was provided on the command
        //   // line. This depends on the startup options in argv since these may contain
        //   // other rc-modifying options. For all other options, the precedence of
        //   // options will be rc first, then command line options, though, despite this
        //   // exception.
        //   std::vector<std::unique_ptr<RcFile>> rc_files;
        //   if (!search_nullary_option(cmd_line_->startup_args, "ignore_all_rc_files", false)) {
        //     const blaze_exit_code::ExitCode rc_parsing_exit_code = get_rc_files(workspace_layout_, workspace, cwd, cmd_line_.get(), &rc_files, error);
        //     if (rc_parsing_exit_code != blaze_exit_code::SUCCESS) {
        //       return rc_parsing_exit_code;
        //     }
        //   }

        //   // The helpers expect regular pointers, not unique_ptrs.
        //   std::vector<RcFile*> rc_file_ptrs;
        //   rc_file_ptrs.reserve(rc_files.size());
        //   for (auto& rc_file : rc_files) { rc_file_ptrs.push_back(rc_file.get()); }

        //   // Parse the startup options in the correct priority order.
        //   const blaze_exit_code::ExitCode parse_startup_options_exit_code = ParseStartupOptions(rc_file_ptrs, error);
        //   if (parse_startup_options_exit_code != blaze_exit_code::SUCCESS) {
        //     return parse_startup_options_exit_code;
        //   }

        //   blazerc_and_env_command_args_ = get_blazerc_and_env_command_args(cwd, rc_file_ptrs, GetProcessedEnv());

        Ok(())
    }

    // static void PrintStartupOptions(const std::string& source, const std::vector<std::string>& options) {
    //   if (!source.empty()) {
    //     std::string startup_args;
    //     blaze_util::JoinStrings(options, ' ', &startup_args);
    //     fprintf(stderr, "INFO: Reading 'startup' options from %s: %s\n", source.c_str(), startup_args.c_str());
    //   }
    // }

    pub fn print_startup_options_provenance_message(&self) {
        // void OptionProcessor::print_startup_options_provenance_message() const {
        //   StartupOptions* parsed_startup_options = get_parsed_startup_options();
        //
        //   // Print the startup flags in the order they are parsed, to keep the
        //   // precedence clear. In order to minimize the number of lines of output in
        //   // the terminal, group sequential flags by origin. Note that an rc file may
        //   // turn up multiple times in this list, if, for example, it imports another
        //   // rc file and contains startup options on either side of the import
        //   // statement. This is done intentionally to make option priority clear.
        //   std::string command_line_source;
        //   std::string& most_recent_blazerc = command_line_source;
        //   std::vector<std::string> accumulated_options;
        //   for (const auto& flag : parsed_startup_options->original_startup_options_) {
        //     if (flag.source == most_recent_blazerc) {
        //       accumulated_options.push_back(flag.value);
        //     } else {
        //       PrintStartupOptions(most_recent_blazerc, accumulated_options);
        //       // Start accumulating again.
        //       accumulated_options.clear();
        //       accumulated_options.push_back(flag.value);
        //       most_recent_blazerc = flag.source;
        //     }
        //   }
        //   // Don't forget to print out the last ones.
        //   PrintStartupOptions(most_recent_blazerc, accumulated_options);
    }

    // blaze_exit_code::ExitCode OptionProcessor::ParseStartupOptions(
    //     const std::vector<RcFile*> &rc_files, std::string *error) {
    //   // Rc files can import other files at any point, and these imported rcs are
    //   // expanded in place. Here, we isolate just the startup options but keep the
    //   // file they came from attached for the option_sources tracking and for
    //   // sending to the server.
    //   std::vector<RcStartupFlag> rcstartup_flags;
    //
    //   for (const auto* blazerc : rc_files) {
    //     const auto iter = blazerc->options().find("startup");
    //     if (iter == blazerc->options().end()) continue;
    //
    //     for (const RcOption& option : iter->second) {
    //       const std::string& source_path =
    //           blazerc->canonical_source_paths()[option.source_index];
    //       rcstartup_flags.push_back({source_path, option.option});
    //     }
    //   }
    //
    //   for (const std::string& arg : cmd_line_->startup_args) {
    //     if (!is_arg(arg)) {
    //       break;
    //     }
    //     rcstartup_flags.push_back(RcStartupFlag("", arg));
    //   }
    //
    //   return startup_options_->ProcessArgs(rcstartup_flags, error);
    // }
    //
    // static bool IsValidEnvName(const char* p) {
    // #if defined(_WIN32) || defined(__CYGWIN__)
    //   for (; *p && *p != '='; ++p) {
    //     if (!((*p >= 'a' && *p <= 'z') || (*p >= 'A' && *p <= 'Z') ||
    //           (*p >= '0' && *p <= '9') || *p == '_' || *p == '(' || *p == ')')) {
    //       return false;
    //     }
    //   }
    // #endif
    //   return true;
    // }
    //
    // #if defined(_WIN32)
    // static void PreprocessEnvString(string* env_str) {
    //   static constexpr const char* vars_to_uppercase[] = {"PATH", "SYSTEMROOT",
    //                                                       "SYSTEMDRIVE",
    //                                                       "TEMP", "TEMPDIR", "TMP"};
    //
    //   int pos = env_str->find_first_of('=');
    //   if (pos == string::npos) return;
    //
    //   string name = env_str->substr(0, pos);
    //   // We do not care about locale. All variable names are ASCII.
    //   std::transform(name.begin(), name.end(), name.begin(), ::toupper);
    //   if (std::find(std::begin(vars_to_uppercase), std::end(vars_to_uppercase),
    //                 name) != std::end(vars_to_uppercase)) {
    //     env_str->assign(name + "=" + env_str->substr(pos + 1));
    //   }
    // }
    //
    // #elif defined(__CYGWIN__)  // not defined(_WIN32)
    //
    // static void PreprocessEnvString(string* env_str) {
    //   int pos = env_str->find_first_of('=');
    //   if (pos == string::npos) return;
    //   string name = env_str->substr(0, pos);
    //   if (name == "PATH") {
    //     env_str->assign("PATH=" + env_str->substr(pos + 1));
    //   } else if (name == "TMP") {
    //     // A valid Windows path "c:/foo" is also a valid Unix path list of
    //     // ["c", "/foo"] so must use ConvertPath here. See GitHub issue #1684.
    //     env_str->assign("TMP=" + blaze_util::ConvertPath(env_str->substr(pos + 1)));
    //   }
    // }
    //
    // #else  // Non-Windows platforms.
    //
    // static void PreprocessEnvString(const string* env_str) {
    //   // do nothing.
    // }
    // #endif  // defined(_WIN32)
    //
    // static std::vector<std::string> GetProcessedEnv() {
    //   std::vector<std::string> processed_env;
    //   for (char** env = environ; *env != NULL; env++) {
    //     string env_str(*env);
    //     if (IsValidEnvName(*env)) {
    //       PreprocessEnvString(&env_str);
    //       processed_env.push_back(std::move(env_str));
    //     }
    //   }
    //   return processed_env;
    // }

    /// IMPORTANT: The options added here do not come from the user. In order for
    /// their source to be correctly tracked, the options must either be passed
    /// as --default_override=0, 0 being "client", or must be listed in
    /// BlazeOptionHandler.INTERNAL_COMMAND_OPTIONS!
    fn get_blazerc_and_env_command_args(
        &self,
        cwd: &str,         // const std::string& cwd,
        rcs: Vec<&RcFile>, //     const std::vector<RcFile*>& blazercs,
        env: Vec<String>,  //     const std::vector<std::string>& env
    ) -> Vec<String> {
        //   // Provide terminal options as coming from the least important rc file.
        //   std::vector<std::string> result = {
        //       "--rc_source=client",
        //       "--default_override=0:common=--isatty=" +
        //           blaze_util::ToString(IsStandardTerminal()),
        //       "--default_override=0:common=--terminal_columns=" +
        //           blaze_util::ToString(GetTerminalColumns())};
        //   if (IsEmacsTerminal()) {
        //     result.push_back("--default_override=0:common=--emacs");
        //   }
        //
        //   EnsurePythonPathOption(&result);
        //
        //   // Map .blazerc numbers to filenames. The indexes here start at 1 because #0
        //   // is reserved the "client" options created by this function.
        //   int cur_index = 1;
        //   std::map<std::string, int> rcfile_indexes;
        //   for (const auto* blazerc : blazercs) {
        //     for (const std::string& source_path : blazerc->canonical_source_paths()) {
        //       // Deduplicate the rc_source list because the same file might be included
        //       // from multiple places.
        //       if (rcfile_indexes.find(source_path) != rcfile_indexes.end()) continue;
        //
        //       result.push_back("--rc_source=" + blaze_util::ConvertPath(source_path));
        //       rcfile_indexes[source_path] = cur_index;
        //       cur_index++;
        //     }
        //   }
        //
        //   // Add RcOptions as default_overrides.
        //   for (const auto* blazerc : blazercs) {
        //     for (const auto& command_options : blazerc->options()) {
        //       const string& command = command_options.first;
        //       // Skip startup flags, which are already parsed by the client.
        //       if (command == "startup") continue;
        //
        //       for (const RcOption& rcoption : command_options.second) {
        //         const std::string& source_path =
        //             blazerc->canonical_source_paths()[rcoption.source_index];
        //         std::ostringstream oss;
        //         oss << "--default_override=" << rcfile_indexes[source_path] << ':'
        //             << command << '=' << rcoption.option;
        //         result.push_back(oss.str());
        //       }
        //     }
        //   }
        //
        //   // Pass the client environment to the server.
        //   for (const string& env_var : env) {
        //     result.push_back("--client_env=" + env_var);
        //   }
        //   result.push_back("--client_cwd=" + blaze_util::ConvertPath(cwd));
        //   return result;
        unimplemented!()
    }

    /// Gets the arguments to the command. This is put together from the default
    /// options specified in the blazerc file(s), the command line, and various
    /// bits and pieces of information about the environment the blaze binary is
    /// executed in.
    fn get_command_arguments(&self) -> Vec<String> {
        //   assert(cmd_line_ != nullptr);
        //   // When the user didn't specify a command, the server expects the command
        //   // arguments to be empty in order to display the help message.
        //   if (cmd_line_->command.empty()) {
        //     return {};
        //   }
        //
        //   std::vector<std::string> command_args = blazerc_and_env_command_args_;
        //   command_args.insert(command_args.end(), cmd_line_->command_args.begin(), cmd_line_->command_args.end());
        //   return command_args;
        unimplemented!()
    }

    /// Gets the arguments explicitly provided by the user's command line.
    fn get_explicit_command_arguments(&self) -> Vec<String> {
        assert!(self.cmd_line.is_some());
        self.cmd_line.as_ref().unwrap().command_args.clone()
    }

    pub fn get_command(&self) -> String {
        assert!(self.cmd_line.is_some());
        self.cmd_line.as_ref().unwrap().command.clone()
    }

    pub fn get_parsed_startup_options(&self) -> StartupOptions {
        assert!(self.parse_options_called);
        self.startup_options.clone()
    }
}

mod internal {
    // #include <algorithm>
    // #include <set>
    //
    // #include "src/main/cpp/rc_file.h"
    // #include "src/main/cpp/util/exit_code.h"
    // #include "src/main/cpp/util/file.h"

    // // Returns the deduped set of bazelrc paths (with respect to its canonical form)
    // // preserving the original order.
    // // All paths in the result were verified to exist (otherwise their canonical
    // // form couldn't have been computed). The paths that cannot be resolved are
    // // omitted.
    // std::vector<std::string> DedupeBlazercPaths(
    //     const std::vector<std::string>& paths);
    //
    // // Given the set of already-ready files, warns if any of the newly loaded_rcs
    // // are duplicates. All paths are expected to be canonical.
    // void WarnAboutDuplicateRcFiles(const std::set<std::string>& read_files,
    //                                const std::deque<std::string>& loaded_rcs);
    //
    // // Get the legacy list of rc files that would have been loaded - this is to
    // // provide a useful warning if files are being ignored that were loaded in a
    // // previous version of Bazel.
    // // TODO(b/3616816): Remove this once the warning is no longer useful.
    // std::set<std::string> GetOldRcPaths(
    //     const WorkspaceLayout* workspace_layout, const std::string& workspace,
    //     const std::string& cwd, const std::string& path_to_binary,
    //     const std::vector<std::string>& startup_args);
    //
    // // Returns what the "user bazelrc" would have been in the legacy rc list.
    // std::string FindLegacyUserBazelrc(const char* cmd_line_rc_file,
    //                                   const std::string& workspace);
    //
    // std::string FindSystemWideRc();
    //
    // std::string FindRcAlongsideBinary(const std::string& cwd,
    //                                   const std::string& path_to_binary);
    //
    // blaze_exit_code::ExitCode ParseErrorToExitCode(RcFile::ParseError parse_error);

    // std::string FindLegacyUserBazelrc(const char* cmd_line_rc_file,
    //                                   const std::string& workspace) {
    //   if (cmd_line_rc_file != nullptr) {
    //     string rcFile = blaze::absolute_path_from_flag(cmd_line_rc_file);
    //     if (!blaze_util::CanReadFile(rcFile)) {
    //       // The actual rc file reading will catch this - we ignore this here in the
    //       // legacy version since this is just a warning. Exit eagerly though.
    //       return "";
    //     }
    //     return rcFile;
    //   }
    //
    //   string workspaceRcFile = blaze_util::JoinPath(workspace, K_RC_BASENAME);
    //   if (blaze_util::CanReadFile(workspaceRcFile)) {
    //     return workspaceRcFile;
    //   }
    //
    //   string home = blaze::GetHomeDir();
    //   if (!home.empty()) {
    //     string userRcFile = blaze_util::JoinPath(home, K_RC_BASENAME);
    //     if (blaze_util::CanReadFile(userRcFile)) {
    //       return userRcFile;
    //     }
    //   }
    //   return "";
    // }
    //
    // std::set<std::string> GetOldRcPaths(
    //     const WorkspaceLayout* workspace_layout, const std::string& workspace,
    //     const std::string& cwd, const std::string& path_to_binary,
    //     const std::vector<std::string>& startup_args,
    //     const std::string& system_bazelrc_path) {
    //   // Find the old list of rc files that would have been loaded here, so we can
    //   // provide a useful warning about old rc files that might no longer be read.
    //   std::vector<std::string> candidate_bazelrc_paths;
    //   if (search_nullary_option(startup_args, "master_bazelrc", true)) {
    //     const std::string workspace_rc =
    //         workspace_layout->GetWorkspaceRcPath(workspace, startup_args);
    //     const std::string binary_rc =
    //         internal::FindRcAlongsideBinary(cwd, path_to_binary);
    //     candidate_bazelrc_paths = {workspace_rc, binary_rc, system_bazelrc_path};
    //   }
    //   string user_bazelrc_path = internal::FindLegacyUserBazelrc(
    //       search_unary_option(startup_args, "--bazelrc", /* warn_if_dupe */ true),
    //       workspace);
    //   if (!user_bazelrc_path.empty()) {
    //     candidate_bazelrc_paths.push_back(user_bazelrc_path);
    //   }
    //   // DedupeBlazercPaths returns paths whose canonical path could be computed,
    //   // therefore these paths must exist.
    //   const std::vector<std::string> deduped_existing_blazerc_paths =
    //       internal::DedupeBlazercPaths(candidate_bazelrc_paths);
    //   return std::set<std::string>(deduped_existing_blazerc_paths.begin(),
    //                                deduped_existing_blazerc_paths.end());
    // }
    //
    // // Deduplicates the given paths based on their canonical form.
    // // Computes the canonical form using blaze_util::MakeCanonical.
    // // Returns the unique paths in their original form (not the canonical one).
    // std::vector<std::string> DedupeBlazercPaths(
    //     const std::vector<std::string>& paths) {
    //   std::set<std::string> canonical_paths;
    //   std::vector<std::string> result;
    //   for (const std::string& path : paths) {
    //     const std::string canonical_path = blaze_util::MakeCanonical(path.c_str());
    //     if (canonical_path.empty()) {
    //       // MakeCanonical returns an empty string when it fails. We ignore this
    //       // failure since blazerc paths may point to invalid locations.
    //     } else if (canonical_paths.find(canonical_path) == canonical_paths.end()) {
    //       result.push_back(path);
    //       canonical_paths.insert(canonical_path);
    //     }
    //   }
    //   return result;
    // }
    //
    // std::string FindSystemWideRc(const std::string& system_bazelrc_path) {
    //   const std::string path =
    //       blaze_util::MakeAbsoluteAndResolveEnvvars(system_bazelrc_path);
    //   if (blaze_util::CanReadFile(path)) {
    //     return path;
    //   }
    //   return "";
    // }
    //
    // std::string FindRcAlongsideBinary(const std::string& cwd,
    //                                   const std::string& path_to_binary) {
    //   const std::string path = blaze_util::IsAbsolute(path_to_binary)
    //                                ? path_to_binary
    //                                : blaze_util::JoinPath(cwd, path_to_binary);
    //   const std::string base = blaze_util::Basename(path_to_binary);
    //   const std::string binary_blazerc_path = path + "." + base + "rc";
    //   if (blaze_util::CanReadFile(binary_blazerc_path)) {
    //     return binary_blazerc_path;
    //   }
    //   return "";
    // }
    //
    // blaze_exit_code::ExitCode ParseErrorToExitCode(RcFile::ParseError parse_error) {
    //   switch (parse_error) {
    //     case RcFile::ParseError::NONE:
    //       return blaze_exit_code::SUCCESS;
    //     case RcFile::ParseError::UNREADABLE_FILE:
    //       // We check readability before parsing, so this is unexpected for
    //       // top-level rc files, so is an INTERNAL_ERROR. It can happen for imported
    //       // files, however, which should be BAD_ARGV, but we don't currently
    //       // differentiate.
    //       // TODO(bazel-team): fix RcFile to reclassify unreadable files that were
    //       // read from a recursive call due to a malformed import.
    //       return blaze_exit_code::INTERNAL_ERROR;
    //     case RcFile::ParseError::INVALID_FORMAT:
    //     case RcFile::ParseError::IMPORT_LOOP:
    //       return blaze_exit_code::BAD_ARGV;
    //     default:
    //       return blaze_exit_code::INTERNAL_ERROR;
    //   }
    // }
    //
    // void WarnAboutDuplicateRcFiles(const std::set<std::string>& read_files,
    //                                const std::vector<std::string>& loaded_rcs) {
    //   // The first rc file in the queue is the top-level one, the one that would
    //   // have imported all the others in the queue. The top-level rc is one of the
    //   // default locations (system, workspace, home) or the explicit path passed by
    //   // --bazelrc.
    //   const std::string& top_level_rc = loaded_rcs.front();
    //
    //   const std::set<std::string> unique_loaded_rcs(loaded_rcs.begin(),
    //                                                 loaded_rcs.end());
    //   // First check if each of the newly loaded rc files was already read.
    //   for (const std::string& loaded_rc : unique_loaded_rcs) {
    //     if (read_files.count(loaded_rc) > 0) {
    //       if (loaded_rc == top_level_rc) {
    //         BAZEL_LOG(WARNING)
    //             << "Duplicate rc file: " << loaded_rc
    //             << " is read multiple times, it is a standard rc file location "
    //                "but must have been unnecessarily imported earlier.";
    //       } else {
    //         BAZEL_LOG(WARNING)
    //             << "Duplicate rc file: " << loaded_rc
    //             << " is read multiple times, most recently imported from "
    //             << top_level_rc;
    //       }
    //     }
    //     // Now check if the top-level rc file loads up its own duplicates (it can't
    //     // be a cycle, since that would be an error and we would have already
    //     // exited, but it could have a diamond dependency of some sort.)
    //     if (std::count(loaded_rcs.begin(), loaded_rcs.end(), loaded_rc) > 1) {
    //       BAZEL_LOG(WARNING) << "Duplicate rc file: " << loaded_rc
    //                          << " is imported multiple times from " << top_level_rc;
    //     }
    //   }
    // }
    //
    // std::vector<std::string> GetLostFiles(
    //     const std::set<std::string>& old_files,
    //     const std::set<std::string>& read_files_canon) {
    //   std::vector<std::string> result;
    //   for (const auto& old : old_files) {
    //     std::string old_canon = blaze_util::MakeCanonical(old.c_str());
    //     if (!old_canon.empty() &&
    //         read_files_canon.find(old_canon) == read_files_canon.end()) {
    //       result.push_back(old);
    //     }
    //   }
    //   return result;
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use slog::{o, Drain};

    fn logger() -> slog::Logger {
        let decorator = slog_term::PlainSyncDecorator::new(slog_term::TestStdoutWriter);
        let drain = slog_term::FullFormat::new(decorator)
            .build()
            .filter_level(slog::Level::Debug)
            .fuse();
        slog::Logger::root(drain, o!())
    }

    fn option_processor() -> OptionProcessor {
        OptionProcessor::new(
            logger(),
            StartupOptions::new(String::from("Bazel"), Some(logger())),
        )
    }

    #[test]
    fn split_command_line_with_empty_args() {
        let args = Vec::<String>::new();
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.err().unwrap(),
            String::from("Unable to split command line, args is empty")
        )
    }

    #[test]
    fn split_command_line_with_all_params() {
        let args = vec![
            "bazel".to_string(),
            "--nomaster_bazelrc".to_string(),
            "build".to_string(),
            "--bar".to_string(),
            ":mytarget".to_string(),
        ];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: vec!["--nomaster_bazelrc".to_string()],
                command: "build".to_string(),
                command_args: vec!["--bar".to_string(), ":mytarget".to_string()],
            }
        );
    }

    #[test]
    fn split_command_line_with_absolute_path_to_binary() {
        let args = vec![
            "mybazel".to_string(),
            "build".to_string(),
            ":mytarget".to_string(),
        ];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "mybazel".to_string(),
                startup_args: Default::default(),
                command: "build".to_string(),
                command_args: vec![":mytarget".to_string()],
            }
        );
    }

    #[test]
    fn split_command_line_with_unary_startup_with_equal() {
        let args = vec![
            "bazel".to_string(),
            "--bazelrc=foo".to_string(),
            "build".to_string(),
            ":mytarget".to_string(),
        ];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: vec!["--bazelrc=foo".to_string()],
                command: "build".to_string(),
                command_args: vec![":mytarget".to_string()],
            }
        );
    }

    #[test]
    fn split_command_line_with_unary_startup_without_equal() {
        let args = vec![
            "bazel".to_string(),
            "--bazelrc".to_string(),
            "foo".to_string(),
            "build".to_string(),
            ":mytarget".to_string(),
        ];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: vec!["--bazelrc=foo".to_string()],
                command: "build".to_string(),
                command_args: vec![":mytarget".to_string()],
            }
        );
    }

    #[test]
    fn split_command_line_with_incomplete_unary_option() {
        let args = vec!["bazel".to_string(), "--bazelrc".to_string()];
        let got = option_processor().split_command_line(args);
        assert_eq!(got.err().unwrap(), String::from("Startup option '--bazelrc' expects a value.\nUsage: '--bazelrc=somevalue' or '--bazelrc somevalue'.\n  For more info, run 'bazel help startup_options'."));
    }

    #[test]
    fn split_command_line_with_multiple_startup() {
        let args = vec![
            "bazel".to_string(),
            "--bazelrc".to_string(),
            "foo".to_string(),
            "--nomaster_bazelrc".to_string(),
            "build".to_string(),
            ":mytarget".to_string(),
        ];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: vec![
                    "--bazelrc=foo".to_string(),
                    "--nomaster_bazelrc".to_string()
                ],
                command: "build".to_string(),
                command_args: vec![":mytarget".to_string()],
            }
        );
    }

    #[test]
    fn split_command_line_with_no_startup_args() {
        let args = vec![
            "bazel".to_string(),
            "build".to_string(),
            ":mytarget".to_string(),
        ];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: Default::default(),
                command: "build".to_string(),
                command_args: vec![":mytarget".to_string()],
            }
        );
    }

    #[test]
    fn split_command_line_with_no_command_args() {
        let args = vec!["bazel".to_string(), "build".to_string()];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: Default::default(),
                command: "build".to_string(),
                command_args: Default::default(),
            }
        );
    }

    #[test]
    fn split_command_line_with_bazel_help() {
        let args = vec!["bazel".to_string(), "help".to_string()];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: Default::default(),
                command: "help".to_string(),
                command_args: Default::default(),
            }
        );
    }

    #[test]
    fn split_command_line_with_bazel_version() {
        let args = vec!["bazel".to_string(), "version".to_string()];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: Default::default(),
                command: "version".to_string(),
                command_args: Default::default(),
            }
        );
    }

    #[test]
    fn split_command_line_with_multiple_command_args() {
        let args = vec![
            "bazel".to_string(),
            "build".to_string(),
            "--foo".to_string(),
            "-s".to_string(),
            ":mytarget".to_string(),
        ];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: Default::default(),
                command: "build".to_string(),
                command_args: vec![
                    "--foo".to_string(),
                    "-s".to_string(),
                    ":mytarget".to_string()
                ],
            }
        );
    }

    #[test]
    fn split_command_line_with_dash_in_startup_args() {
        let args = vec!["bazel".to_string(), "--".to_string()];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.err().unwrap(),
            String::from(
                "Unknown startup option: '--'.\n  For more info, run 'bazel help startup_options'."
            )
        );
    }

    #[test]
    fn split_command_line_with_dash_dash() {
        let args = vec![
            "bazel".to_string(),
            "--nomaster_bazelrc".to_string(),
            "build".to_string(),
            "--b".to_string(),
            "--".to_string(),
            ":mytarget".to_string(),
        ];
        let got = option_processor().split_command_line(args);
        assert_eq!(
            got.unwrap(),
            CommandLine {
                path_to_binary: "bazel".to_string(),
                startup_args: vec!["--nomaster_bazelrc".to_string()],
                command: "build".to_string(),
                command_args: vec!["--b".to_string(), "--".to_string(), ":mytarget".to_string()],
            }
        );
    }
}
