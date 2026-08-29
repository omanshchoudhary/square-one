use crate::parser::Argv;

use std::ffi::CString;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::{dup2_stdin, dup2_stdout, execvp, fork, ForkResult};

pub fn run(unit: Argv) {
    let file = CString::new(unit.program.as_str()).unwrap();
    let mut argv = vec![file.clone()];
    argv.extend(
        unit.args
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap()),
    );

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            if setup_redirects(&unit).is_err() {
                unsafe { nix::libc::_exit(1) };
            }
            let _ = execvp(&file, &argv);
            unsafe { nix::libc::_exit(1) };
        }
        Err(e) => eprintln!("fork: {e}"),
    }
}

fn setup_redirects(unit: &Argv) -> nix::Result<()> {
    if let Some(path) = &unit.stdin {
        let fd = open(path.as_str(), OFlag::O_RDONLY, Mode::empty())?;
        dup2_stdin(&fd)?;
    }

    if let Some((path, append)) = &unit.stdout {
        let mut flags = OFlag::O_WRONLY | OFlag::O_CREAT;
        flags |= if *append {
            OFlag::O_APPEND
        } else {
            OFlag::O_TRUNC
        };
        let fd = open(path.as_str(), flags, Mode::from_bits_truncate(0o644))?;
        dup2_stdout(&fd)?;
    }

    Ok(())
}
