use std::collections::{HashMap, VecDeque};
use std::fs;

const COMMAND_IMPORT: &str = "import";
const COMMAND_TRY_IMPORT: &str = "try-import";

/// Single option in an rc file.
#[derive(Debug, PartialEq)]
struct RcOption {
    option: String,
    /// Only keep the index of the source file to avoid copying the paths all over.
    /// This index points into the RcFile's canonical_source_paths() vector.
    source_index: usize,
}

#[derive(Debug)]
pub enum ParseError {
    UnreadableFile(String),
    InvalidFormat(String),
    ImportLoop(String),
}

/// Reads and parses a single rc file with all its imports.
///
/// The syntax for the Bazel RC files is documented in
/// https://docs.bazel.build/versions/master/guide.html#bazelrc-syntax-and-semantics.
#[derive(Debug)]
pub struct RcFile {
    // // Command -> all options for that command (in order of appearance).
    // using OptionMap = std::unordered_map<std::string, std::vector<RcOption>>;
    // const OptionMap& options() const { return options_; }
    //
    // private:
    // RcFile(std::string filename, const WorkspaceLayout* workspace_layout, std::string workspace);
    //
    // // Recursive call to parse a file and its imports.
    // ParseError ParseFile(const std::string& filename,
    // std::deque<std::string>* import_stack,
    // std::string* error_text);
    filename_: String,

    /// Workspace definition.
    // const WorkspaceLayout* workspace_layout_;
    workspace_: String,

    /// Full closure of rcfile paths imported from this file (including itself).
    /// These are all canonical paths, created with blaze_util::MakeCanonical.
    /// This also means all of these paths should exist.
    canonical_rcfile_paths_: Vec<String>,

    /// All options parsed from the file.
    options_: HashMap<String, Vec<RcOption>>,
}

// RcFile::RcFile(string filename, const WorkspaceLayout* workspace_layout, string workspace)
// : filename_(std::move(filename)),
// workspace_layout_(workspace_layout),
// workspace_(std::move(workspace)) {}

impl RcFile {
    /// Constructs a parsed rc file object, or returns an error.
    // /*static*/ std::unique_ptr<RcFile> RcFile::Parse(
    // std::string filename, const WorkspaceLayout* workspace_layout,
    // std::string workspace, ParseError* error, std::string* error_text) {
    pub fn parse(filename: &str, workspace: &str) -> Result<Self, ParseError> {
        let mut rc = RcFile {
            filename_: filename.to_string(),
            workspace_: workspace.to_string(),
            canonical_rcfile_paths_: Vec::new(),
            options_: Default::default(),
        };
        let mut import_stack = VecDeque::new();
        rc.parse_file(filename, &mut import_stack)?;
        Ok(rc)
    }

    /// Returns all relevant rc sources for this file (including itself).
    pub fn canonical_source_paths(self) -> Vec<String> {
        self.canonical_rcfile_paths_
    }

    fn parse_file(
        &mut self,
        filename: &str,
        import_stack: &mut VecDeque<String>,
    ) -> Result<(), ParseError> {
        // BAZEL_LOG(INFO) << "Parsing the RcFile " << filename;
        println!("Parsing {}", filename);

        let contents = match std::fs::read_to_string(&filename) {
            Ok(x) => x,
            // FIXME how to propagate _e in the returned error?
            Err(e) => {
                return Err(ParseError::UnreadableFile(format!(
                    "Unexpected error reading file '{}': {:?}",
                    filename, e
                )))
            }
        };

        let canonical_filename = match fs::canonicalize(filename) {
            Ok(p) => p,
            // FIXME how to propagate _e in the returned error?
            Err(e) => {
                return Err(ParseError::UnreadableFile(format!(
                    "canonicalize error: {:?}",
                    e
                )));
            }
        };

        let rcfile_index = self.canonical_rcfile_paths_.len();
        if let Some(p) = canonical_filename.to_str() {
            self.canonical_rcfile_paths_.push(String::from(p));
        } else {
            return Err(ParseError::UnreadableFile(
                "canonical_filename.to_str error".to_string(),
            ));
        }

        // A '\' at the end of a line continues the line.
        let contents = str::replace(&contents, "\\\r\n", "");
        let contents = str::replace(&contents, "\\\n", "");

        for l in contents
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .filter(|l| !l.starts_with('#'))
        {
            let words = l.split_whitespace().collect::<Vec<&str>>();
            if words.is_empty() {
                continue;
            }

            let command = words[0];

            if command == COMMAND_IMPORT || command == COMMAND_TRY_IMPORT {
                // FIXME
                // if (words.size() != 2 ||
                // (words[1].compare(0, workspace_layout_->WorkspacePrefixLength,
                // workspace_layout_->WorkspacePrefix) == 0 &&
                // !workspace_layout_->WorkspaceRelativizeRcFilePath(workspace_,
                // &words[1]))) {
                if words.len() != 2 {
                    return Err(ParseError::InvalidFormat(format!("Invalid import declaration in '{:?}': '{}' (are you in your source checkout/WORKSPACE?)", canonical_filename.to_str(), l)));
                }

                if import_stack.contains(&words[1].to_string()) {
                    return Err(ParseError::ImportLoop(format!(
                        "Import loop detected:\n{} {}",
                        import_stack.as_slices().0.join(" "),
                        words[1],
                    )));
                }

                import_stack.push_back(words[1].to_string());

                match self.parse_file(words[1], import_stack) {
                    Ok(_) => {}
                    Err(ParseError::UnreadableFile(_)) if command == COMMAND_TRY_IMPORT => {
                        // For try-import, we ignore it if we couldn't find a file.
                        println!("Skipped optional import of {}, the specified rc file either does not exist or is not readable.", words[1]);
                    }
                    e => {
                        // Files that are there but are malformed or introduce a loop are
                        // still a problem, though, so perpetuate those errors as we would
                        // for a normal import statement.
                        return e;
                    }
                };

                import_stack.pop_back();
            } else {
                self.options_
                    .entry(command.to_string())
                    .or_insert_with(Vec::new)
                    .push(RcOption {
                        option: words[1..].join(" "),
                        source_index: rcfile_index,
                    });
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_split_rc_file() {
        let mut expected = HashMap::new();
        expected.insert(
            "test".to_string(),
            vec![
                RcOption {
                    option: "--test_output errors".to_string(),
                    source_index: 0,
                },
            ],
        );

        let got = RcFile::parse("src/main/rust/testdata/rc_file/line_split.rc", "dummy-workspace");
        assert!(got.is_ok());
        if let Ok(rc) = got {
            assert_eq!(rc.options_, expected);
        }
    }

    #[test]
    fn parse_user_rc_file() {
        let mut expected = HashMap::new();
        expected.insert(
            "build".to_string(),
            vec![
                RcOption {
                    option: "--disk_cache=~/.cache/bazel/".to_string(),
                    source_index: 0,
                },
                RcOption {
                    option: "--repository_cache=~/.cache/bazel-repositories/".to_string(),
                    source_index: 0,
                },
            ],
        );
        expected.insert(
            "test".to_string(),
            vec![
                RcOption {
                    option: "--test_output errors".to_string(),
                    source_index: 0,
                },
                RcOption {
                    option: "--test_summary terse".to_string(),
                    source_index: 0,
                },
                RcOption {
                    option: "--test_verbose_timeout_warnings".to_string(),
                    source_index: 0,
                },
            ],
        );

        let got = RcFile::parse("src/main/rust/testdata/rc_file/user.rc", "dummy-workspace");
        assert!(got.is_ok());
        if let Ok(rc) = got {
            assert_eq!(rc.options_, expected);
        }
    }

    #[test]
    fn parse_tensorflow_rc_file() {
        assert!(RcFile::parse(
            "src/main/rust/testdata/rc_file/tensorflow.rc",
            "dummy-workspace",
        )
        .is_ok());
    }

    #[test]
    fn parse_import_loop_self_rc_file() {
        match RcFile::parse(
            "src/main/rust/testdata/rc_file/import_loop_self.rc",
            "dummy-workspace",
        ) {
            Err(ParseError::ImportLoop(msg)) => {
                assert_eq!(msg, "Import loop detected:\nsrc/main/rust/testdata/rc_file/import_loop_self.rc src/main/rust/testdata/rc_file/import_loop_self.rc")
            }
            _ => {
                assert!(false)
            }
        };
    }

    #[test]
    fn parse_import_loop1_rc_file() {
        match RcFile::parse(
            "src/main/rust/testdata/rc_file/import_loop1.rc",
            "dummy-workspace",
        ) {
            Err(ParseError::ImportLoop(msg)) => {
                assert_eq!(msg, "Import loop detected:\nsrc/main/rust/testdata/rc_file/import_loop2.rc src/main/rust/testdata/rc_file/import_loop3.rc src/main/rust/testdata/rc_file/import_loop1.rc src/main/rust/testdata/rc_file/import_loop2.rc")
            }
            _ => {
                assert!(false)
            }
        };
    }
}
