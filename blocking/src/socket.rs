use std::env;
use std::path::PathBuf;
use std::process::Command;
use swayipc_types::{Error::SwayFailed, Fallible};

//TODO: try block instead of function
pub fn get_socket_path() -> Fallible<PathBuf> {
    _get_socket_path().map(PathBuf::from)
}

fn _get_socket_path() -> Fallible<String> {
    if let Ok(sockpath) = env::var("I3SOCK") {
        return Ok(sockpath);
    }
    if let Ok(sockpath) = env::var("SWAYSOCK") {
        return Ok(sockpath);
    }
    let output = Command::new("i3").arg("--get-socketpath").output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_owned());
    }
    let output = Command::new("sway").arg("--get-socketpath").output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_owned());
    }
    Err(SwayFailed(output.status.code().unwrap_or(0), output.stderr))
}
