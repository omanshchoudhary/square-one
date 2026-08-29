use crate::parser::Argv;

use std::ffi::CString;
use std::os::fd::AsRawFd;

use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::signal::{signal, SigHandler, Signal};
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::{dup2_stdin, dup2_stdout, execvp, fork, pipe, ForkResult};

pub fn run(commands: Vec<Argv>) {
    let n = commands.len();
    let mut pipes = Vec::new();
    for _ in 0..n.saturating_sub(1) {
        match pipe() {
            Ok(pair) => pipes.push(pair),
            Err(e) => {
                eprintln!("pipe: {e}");
                return;
            }
        }
    }

    let mut children = Vec::new();
    for (i, unit) in commands.iter().enumerate() {
        let (file, argv) = match argv_of(unit) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => children.push(child),
            Ok(ForkResult::Child) => {
                let _ = unsafe { signal(Signal::SIGINT, SigHandler::SigDfl) };
                child_run(i, n, unit, &pipes, &file, &argv);
            }
            Err(e) => {
                eprintln!("fork: {e}");
                break;
            }
        }
    }

    drop(pipes);
    for child in children {
        wait_child(child);
    }
}

fn child_run(
    i: usize,
    n: usize,
    unit: &Argv,
    pipes: &[(std::os::fd::OwnedFd, std::os::fd::OwnedFd)],
    file: &CString,
    argv: &[CString],
) -> ! {
    if i > 0 {
        if let Err(e) = dup2_stdin(&pipes[i - 1].0) {
            child_exit(&format!("dup2: {e}"));
        }
    }
    if i + 1 < n {
        if let Err(e) = dup2_stdout(&pipes[i].1) {
            child_exit(&format!("dup2: {e}"));
        }
    }
    close_pipe_fds(pipes);

    if let Err(e) = setup_redirects(unit) {
        child_exit(&format!("{e}"));
    }

    let _ = execvp(file.as_c_str(), argv);
    child_exit(&format!("{}: command not found", unit.program));
}

fn argv_of(unit: &Argv) -> Result<(CString, Vec<CString>), String> {
    let file = CString::new(unit.program.as_str())
        .map_err(|_| format!("{}: invalid argument", unit.program))?;
    let mut argv = vec![file.clone()];
    for arg in &unit.args {
        argv.push(
            CString::new(arg.as_str()).map_err(|_| format!("{arg}: invalid argument"))?,
        );
    }
    Ok((file, argv))
}

fn wait_child(child: nix::unistd::Pid) {
    loop {
        match waitpid(child, None) {
            Ok(_) | Err(Errno::ECHILD) => return,
            Err(Errno::EINTR) => continue,
            Err(e) => {
                eprintln!("wait: {e}");
                return;
            }
        }
    }
}

fn child_exit(msg: &str) -> ! {
    eprintln!("{msg}");
    unsafe { nix::libc::_exit(1) };
}

fn close_pipe_fds(pipes: &[(std::os::fd::OwnedFd, std::os::fd::OwnedFd)]) {
    for (read, write) in pipes {
        unsafe {
            nix::libc::close(read.as_raw_fd());
            nix::libc::close(write.as_raw_fd());
        }
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
