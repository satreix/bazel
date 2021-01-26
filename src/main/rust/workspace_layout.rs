const WORKSPACE_DOT_BAZEL_MARKER: &str = "WORKSPACE.bazel";
const WORKSPACE_MARKER: &str = "WORKSPACE";
const WORKSPACE_PREFIX: &str = "%workspace%/";

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

// FIXME implement for other platforms from blaze::GetOutputRoot
#[cfg(target_os = "macos")]
pub fn get_output_root() -> PathBuf {
    Path::new("/var/tmp").to_path_buf()
}

fn in_workspace(workspace: &PathBuf) -> bool {
    vec![WORKSPACE_DOT_BAZEL_MARKER, WORKSPACE_MARKER]
        .iter()
        .map(|x| workspace.join(x))
        .any(|x| x.exists() && !x.is_dir())
}

pub fn find_workspace(cwd: &PathBuf) -> Result<PathBuf, &'static str> {
    let mut workspace = Path::new(cwd.to_str().unwrap());
    loop {
        if in_workspace(&workspace.to_path_buf()) {
            return Ok(workspace.to_path_buf());
        }

        workspace = workspace.parent().ok_or("could not find workspace")?;

        if workspace.to_str().unwrap() == "" {
            break;
        }
    }

    Err("could not find workspace")
}

// e.g. A Bazel server process running in ~/src/myproject (where there's a
// ~/src/myproject/WORKSPACE file) will appear in ps(1) as "bazel(myproject)".
pub fn pretty_workspace_name(workspace: &PathBuf) -> String {
    workspace
        .file_name()
        .and_then(OsStr::to_str)
        .map_or("".to_string(), str::to_owned)
}

pub fn workspace_rc_path(workspace: &PathBuf, _startup_args: Vec<String>) -> String {
    workspace
        .join("tools/bazel.rc")
        .to_str()
        .unwrap()
        .to_owned()
}

// Strip off the "%workspace%/" prefix and prepend the true workspace path.
// In theory this could use alternate search paths for rc files.
pub fn workspace_relativize_rc_file_path(workspace: &PathBuf, path_fragment: &str) -> String {
    String::from(path_fragment).replace(
        WORKSPACE_PREFIX,
        &(workspace.to_str().unwrap().to_owned() + "/"),
    )
}
