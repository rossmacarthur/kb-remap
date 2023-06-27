use std::process;

use anyhow::{bail, Context, Result};

pub trait CommandExt {
    /// Run the command return the standard output as a UTF-8 string.
    fn output_text(&mut self) -> Result<String>;
}

impl CommandExt for process::Command {
    /// Run the command return the standard output as a UTF-8 string.
    fn output_text(&mut self) -> Result<String> {
        let output = self
            .output()
            .with_context(|| format!("could not execute subprocess: `{:?}`", self))?;
        if !output.status.success() {
            bail!(format_error_msg(self, output));
        }
        String::from_utf8(output.stdout).context("failed to parse stdout")
    }
}

/// Nicely format an error message for when the subprocess didn't exit
/// successfully.
fn format_error_msg(cmd: &process::Command, output: process::Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut msg = format!(
        "subprocess didn't exit successfully `{:?}` ({})",
        cmd, output.status
    );
    if !stdout.trim().is_empty() {
        msg.push_str(&format!("\n--- stdout\n{}", stdout));
    }
    if !stderr.trim().is_empty() {
        msg.push_str(&format!("\n--- stderr\n{}", stderr));
    }
    msg
}
