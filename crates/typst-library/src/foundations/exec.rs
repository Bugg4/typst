use std::process::Command;

use typst_syntax::Spanned;

use crate::diag::{SourceResult, bail, warning};
use crate::engine::Engine;
use crate::foundations::{Str, func};

/// Executes an external shell script and returns its standard output.
///
/// This function runs the given string as a command in a shell
/// (`sh -c "..."`). It waits for the command to finish. If the command 
/// succeeds (exit code 0), its standard output is returned as a string. If 
/// the command writes anything to standard error, it will emit compiler 
/// warnings. If the command fails (non-zero exit code or other errors), 
/// compilation will fail with an error.
///
/// Note: This function allows executing arbitrary code on the system the 
/// compiler is running on.
///
/// # Example
/// ```example
/// #let res = exec("echo Hello, World!")
/// #res
/// ```
#[func]
pub fn exec(
    engine: &mut Engine,
    /// The string to execute as a shell command.
    command: Spanned<String>,
) -> SourceResult<Str> {
    let result = Command::new("sh").arg("-c").arg(&command.v).output();

    match result {
        Ok(output) => {
            if !output.stderr.is_empty() {
                let stderr_str = String::from_utf8_lossy(&output.stderr);
                engine.sink.warn(warning!(command.span, "script produced stderr: {}", stderr_str));
            }

            if !output.status.success() {
                bail!(command.span, "script failed with exit status {}", output.status);
            }

            let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();
            Ok(Str::from(stdout_str))
        }
        Err(e) => {
            bail!(command.span, "failed to execute script: {}", e);
        }
    }
}
