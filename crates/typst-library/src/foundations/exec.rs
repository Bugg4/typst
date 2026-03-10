use std::process::Command;

use typst_syntax::Spanned;

use crate::World;
use crate::diag::{SourceResult, bail, warning};
use crate::engine::Engine;
use crate::foundations::{Content, Str, cast, func};
use crate::text::RawElem;
use crate::Feature;

/// A command source: either a plain string or raw content.
pub enum ExecCommand {
    String(String),
    Raw(String),
}

impl ExecCommand {
    fn as_str(&self) -> &str {
        match self {
            Self::String(s) | Self::Raw(s) => s,
        }
    }
}

cast! {
    ExecCommand,
    self => match self {
        Self::String(s) => s.into_value(),
        Self::Raw(s) => s.into_value(),
    },
    v: String => Self::String(v),
    v: Content => {
        if !v.is::<RawElem>() {
            return Err(ecow::eco_format!(
                "expected string or raw content, found {}",
                v.elem().name()
            ).into());
        }
        Self::Raw(v.plain_text().into())
    },
}

/// Executes an external shell script and returns its standard output.
///
/// This function runs the given string as a command in a shell
/// (`sh -c "..."`). It waits for the command to finish. If the command 
/// succeeds (exit code 0), its standard output is returned as a string. If 
/// the command writes anything to standard error, it will emit compiler 
/// warnings. If the command fails (non-zero exit code or other errors), 
/// compilation will fail with an error.
///
/// You can pass either a plain string or a raw text block (enclosed in
/// triple backticks). The latter is useful for multi-line shell scripts.
///
/// Note: This function allows executing arbitrary code on the system the 
/// compiler is running on. It requires the `--allow-exec` flag to be set.
///
/// # Example
/// ```example
/// #let res = exec("echo Hello, World!")
/// #res
/// ```
#[func]
pub fn exec(
    engine: &mut Engine,
    /// The string or raw content to execute as a shell command.
    command: Spanned<ExecCommand>,
) -> SourceResult<Str> {
    if !engine.world.library().features.is_enabled(Feature::Exec) {
        bail!(command.span, "exec() requires the --allow-exec flag or --features=exec");
    }

    let result = Command::new("sh").arg("-c").arg(command.v.as_str()).output();

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
