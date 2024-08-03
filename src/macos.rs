use std::{ffi::OsStr, process::Command};

pub fn commands<T: AsRef<OsStr>>(path: T) -> Vec<Command> {
    let mut cmd = Command::new("/usr/bin/open");
    cmd.arg(path.as_ref());
    vec![cmd]
}

pub fn with_command<T: AsRef<OsStr>>(path: T, app: impl AsRef<OsStr>) -> Command {
    let mut cmd = Command::new("/usr/bin/open");
    cmd.arg(path.as_ref()).arg("-a").arg(app.as_ref());
    cmd
}
