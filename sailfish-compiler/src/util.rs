use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn read_to_string(path: &Path) -> io::Result<String> {
    let mut content = std::fs::read_to_string(path)?;

    // strip break line at file end
    if content.ends_with('\n') {
        content.truncate(content.len() - 1);
        if content.ends_with('\r') {
            content.truncate(content.len() - 1);
        }
    }

    Ok(content)
}

/// Format block expression using `rustfmt` command
pub fn rustfmt_block(source: &str) -> io::Result<String> {
    let mut new_source = String::with_capacity(source.len() + 11);
    new_source.push_str("fn render()");
    new_source.push_str(source);

    let mut child = Command::new("rustfmt")
        .args(&["--emit", "stdout", "--color", "never", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let stdin = child
        .stdin
        .as_mut()
        .ok_or_else(|| io::Error::from(io::ErrorKind::BrokenPipe))?;
    stdin.write_all(new_source.as_bytes())?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let mut s = unsafe { String::from_utf8_unchecked(output.stdout) };
        let brace_offset = s.find('{').unwrap();
        s.replace_range(..brace_offset, "");
        Ok(s)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "rustfmt command failed",
        ))
    }
}
