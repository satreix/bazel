const WORKSPACE_DOT_BAZEL_MARKER: &str = "WORKSPACE.bazel";
const WORKSPACE_MARKER: &str = "WORKSPACE";
const WORKSPACE_PREFIX: &str = "%workspace%/";

use std::ffi::OsStr;
use std::path::Path;

// FIXME implement for other platforms from blaze::GetOutputRoot
#[cfg(target_os = "macos")]
pub fn get_output_root() -> String {
    "/var/tmp".to_string()
}

fn in_workspace(workspace: &str) -> bool {
    vec![WORKSPACE_DOT_BAZEL_MARKER, WORKSPACE_MARKER]
        .iter()
        .map(|x| Path::new(workspace).join(x))
        .any(|x| x.exists() && !x.is_dir())
}

pub fn find_workspace(cwd: &str) -> Result<&str, &str> {
    let mut workspace = Path::new(cwd);
    loop {
        if in_workspace(workspace.to_str().unwrap()) {
            return Ok(workspace.to_str().unwrap());
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
pub fn pretty_workspace_name(workspace: &str) -> String {
    Path::new(workspace)
        .file_name()
        .and_then(OsStr::to_str)
        .map_or("".to_string(), str::to_owned)
}

pub fn workspace_rc_path(workspace: &str, _startup_args: Vec<String>) -> String {
    Path::new(workspace)
        .join("tools/bazel.rc")
        .to_str()
        .unwrap()
        .to_owned()
}

// Strip off the "%workspace%/" prefix and prepend the true workspace path.
// In theory this could use alternate search paths for rc files.
pub fn workspace_relativize_rc_file_path(workspace: &str, path_fragment: &str) -> String {
    String::from(path_fragment).replace(WORKSPACE_PREFIX, &(workspace.to_owned() + "/"))
}
