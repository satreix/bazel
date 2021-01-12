use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::workspace_layout::workspace_relativize_rc_file_path;
use slog::info;
use std::path::PathBuf;

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

impl RcOption {
    pub fn new(option: String, source_index: usize) -> Self {
        Self {
            option,
            source_index,
        }
    }
}

#[derive(Debug, PartialEq)]
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

        let contents = fs::read_to_string(&filename).map_err(|e| {
            ParseError::UnreadableFile(format!("error reading file '{}': {:?}", filename, e))
        })?;

        let canonical_filename = fs::canonicalize(filename)
            .map_err(|e| ParseError::UnreadableFile(format!("canonicalize error: {:?}", e)))?;

        let rcfile_index = self.canonical_rcfile_paths.len();
        let canonical_filename_str = canonical_filename.to_str().ok_or_else(|| {
            ParseError::UnreadableFile("canonical_filename.to_str error".to_string())
        })?;
        self.canonical_rcfile_paths
            .push(canonical_filename_str.to_string());

        // A '\' at the end of a line continues the line.
        let contents = str::replace(&contents, "\\\r\n", "");
        let contents = str::replace(&contents, "\\\n", "");

        contents
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .filter(|l| !l.starts_with('#'))
            .try_for_each(|l| self.parse_line(l, import_stack, &canonical_filename, rcfile_index))
    }

    fn parse_line(
        &mut self,
        line: &str,
        import_stack: &mut VecDeque<String>,
        canonical_filename: &PathBuf,
        rcfile_index: usize,
    ) -> Result<(), ParseError> {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        if words.is_empty() {
            return Ok(());
        }

        let command = words[0];

        if command == COMMAND_IMPORT || command == COMMAND_TRY_IMPORT {
            if words.len() != 2 {
                return Err(ParseError::InvalidFormat(format!(
                    "Invalid import declaration in '{:?}': '{}' (are you in your source checkout/WORKSPACE?)",
                    canonical_filename.to_str(),
                    line,
                )));
            }

            let import = workspace_relativize_rc_file_path(&self.workspace, words[1]);

            if import_stack.contains(&import) {
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

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use slog::{o, Drain};

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

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
        let got = RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/line_split.rc",
            "src/main/rust/testdata/rc_file",
        );

        let want = map! {
            "test".to_string() => vec![
                RcOption::new("--test_output errors".to_string(), 0),
            ]
        };

        assert!(got.is_ok());
        assert_eq!(want, got.unwrap().options());
    }

    #[test]
    fn parse_user_rc_file() {
        let got = RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/user.rc",
            "src/main/rust/testdata/rc_file",
        );

        let want = map! {
            "build".to_string() => vec![
                RcOption::new("--disk_cache=~/.cache/bazel/".to_string(), 0),
                RcOption::new("--repository_cache=~/.cache/bazel-repositories/".to_string(), 0),
            ],
            "test".to_string() => vec![
                RcOption::new("--test_output errors".to_string(), 0),
                RcOption::new("--test_summary terse".to_string(), 0),
                RcOption::new("--test_verbose_timeout_warnings".to_string(), 0),
            ]
        };

        assert!(got.is_ok());
        assert_eq!(want, got.unwrap().options());
    }

    #[test]
    fn parse_import_workspace_prefix_rc_file() {
        let got = RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/import_workspace_prefix.rc",
            "src/main/rust/testdata/rc_file",
        );
        assert!(got.is_ok());
    }

    #[test]
    fn parse_tensorflow_rc_file() {
        let got = RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/tensorflow.rc",
            "src/main/rust/testdata/rc_file",
        );
        assert!(got.is_ok());
    }

    #[test]
    fn parse_import_loop_self_rc_file() {
        let got = RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/import_loop_self.rc",
            "src/main/rust/testdata/rc_file",
        );
        let want = ParseError::ImportLoop("Import loop detected:\nsrc/main/rust/testdata/rc_file/import_loop_self.rc src/main/rust/testdata/rc_file/import_loop_self.rc".to_string());
        assert!(got.is_err());
        assert_eq!(want, got.err().unwrap());
    }

    #[test]
    fn parse_import_loop1_rc_file() {
        let got = RcFile::new(
            logger(),
            "src/main/rust/testdata/rc_file/import_loop1.rc",
            "src/main/rust/testdata/rc_file",
        );
        let want = ParseError::ImportLoop("Import loop detected:\nsrc/main/rust/testdata/rc_file/import_loop2.rc src/main/rust/testdata/rc_file/import_loop3.rc src/main/rust/testdata/rc_file/import_loop1.rc src/main/rust/testdata/rc_file/import_loop2.rc".to_string());
        assert!(got.is_err());
        assert_eq!(want, got.err().unwrap());
    }
}
