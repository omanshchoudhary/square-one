use crate::parser::Argv;

use std::ffi::CString;
use nix::unistd::{fork, execvp, ForkResult};
use nix::sys::wait::waitpid;

pub fn run(unit: Argv) {
    let file = CString::new(unit.program).unwrap();
    let mut argv = vec![file.clone()];
    argv.extend(
        unit.args
            .into_iter()
            .map(|arg| CString::new(arg).unwrap()),
    );

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            let _ = execvp(&file, &argv);
            unsafe { nix::libc::_exit(1) };
        }
        Err(e) => eprintln!("fork: {e}"),
    }
}