use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::workspace_layout::workspace_relativize_rc_file_path;
use slog::info;

const COMMAND_IMPORT: &str = "import";
const COMMAND_TRY_IMPORT: &str = "try-import";

/// Single option in an rc file.
#[derive(Debug, PartialEq)]
pub struct RcOption {
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
    logger: slog::Logger,

    filename: String,

    /// Full closure of rcfile paths imported from this file (including itself).
    /// These are all canonical paths, created with blaze_util::MakeCanonical.
    /// This also means all of these paths should exist.
    canonical_rcfile_paths: Vec<String>,

    /// All options parsed from the file.
    options: HashMap<String, Vec<RcOption>>,

    /// Workspace definition.
    // FIXME const WorkspaceLayout* workspace_layout;
    workspace: String,
}

impl RcFile {
    /// Constructs a parsed rc file object, or returns an error.
    pub fn new(
        logger: slog::Logger,
        filename: &str,
        // FIXME const WorkspaceLayout* workspace_layout
        workspace: &str,
    ) -> Result<Self, ParseError> {
        let mut rc = Self {
            logger,
            filename: filename.to_string(),
            canonical_rcfile_paths: Vec::new(),
            options: Default::default(),
            workspace: workspace.to_string(),
        };
        let mut import_stack = VecDeque::new();
        rc.parse_file(filename, &mut import_stack)?;
        Ok(rc)
    }

    /// Returns all relevant rc sources for this file (including itself).
    pub fn canonical_source_paths(self) -> Vec<String> {
        self.canonical_rcfile_paths
    }

    /// Command -> all options for that command (in order of appearance).
    pub fn options(self) -> HashMap<String, Vec<RcOption>> {
        self.options
    }

    fn parse_file(
        &mut self,
        filename: &str,
        import_stack: &mut VecDeque<String>,
    ) -> Result<(), ParseError> {
        info!(self.logger, "parsing rc file"; "filename" => filename);

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

        let rcfile_index = self.canonical_rcfile_paths.len();
        if let Some(p) = canonical_filename.to_str() {
            self.canonical_rcfile_paths.push(String::from(p));
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
                if words.len() != 2 {
                    return Err(ParseError::InvalidFormat(format!("Invalid import declaration in '{:?}': '{}' (are you in your source checkout/WORKSPACE?)", canonical_filename.to_str(), l)));
                }

                let import = workspace_relativize_rc_file_path(&self.workspace, words[1]);

                if import_stack.contains(&import.to_string()) {
                    return Err(ParseError::ImportLoop(format!(
                        "Import loop detected:\n{} {}",
                        import_stack.as_slices().0.join(" "),
                        words[1],
                    )));
                }

                import_stack.push_back(import.to_string());

                match self.parse_file(&import, import_stack) {
                    Ok(_) => {}
                    Err(ParseError::UnreadableFile(_)) if command == COMMAND_TRY_IMPORT => {
                        // For try-import, we ignore it if we couldn't find a file.
                        info!(self.logger, "Skipped optional import. The specified rc file either does not exist or is not readable."; "import" => words[1]);
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
                self.options
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
    use slog::{o, Drain};

    fn logger() -> slog::Logger {
        let decorator = slog_term::PlainSyncDecorator::new(slog_term::TestStdoutWriter);
        let drain = slog_term::FullFormat::new(decorator)
            .build()
            .filter_level(slog::Level::Debug)
            .fuse();
        slog::Logger::root(drain, o!())
    }

    #[test]
    fn parse_line_split_rc_file() {
        let mut expected = HashMap::new();
        expected.insert(
            "test".to_string(),
            vec![RcOption {
                option: "--test_output errors".to_string(),
                source_index: 0,
            }],
        );

        let got = RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/line_split.rc",
            "dummy-workspace",
        );
        assert!(got.is_ok());
        if let Ok(rc) = got {
            assert_eq!(rc.options(), expected);
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

        let got = RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/user.rc",
            "dummy-workspace",
        );
        assert!(got.is_ok());
        if let Ok(rc) = got {
            assert_eq!(rc.options(), expected);
        }
    }

    #[test]
    fn parse_import_workspace_prefix_rc_file() {
        assert!(RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/import_workspace_prefix.rc",
            "src/main/rust/testdata/rc_file",
        )
        .is_ok());
    }

    #[test]
    fn parse_tensorflow_rc_file() {
        assert!(RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/tensorflow.rc",
            "dummy-workspace",
        )
        .is_ok());
    }

    #[test]
    fn parse_import_loop_self_rc_file() {
        match RcFile::new(
            logger(),
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
        match RcFile::new(
            logger(),
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
