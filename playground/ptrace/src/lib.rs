#![feature(iter_array_chunks)]
use nix::{
    libc::user_regs_struct,
    sys::{
        ptrace::{self, Options},
        wait,
    },
    unistd::Pid,
    Result,
};
use std::ffi::c_void;

struct TracedProcess {
    pid: Pid,
    open_guards: usize,
}

impl TracedProcess {
    pub fn seize(pid: Pid) -> Result<TracedProcess> {
        ptrace::seize(pid, Options::empty())?;
        Ok(TracedProcess {
            pid,
            open_guards: 0,
        })
    }
    pub fn interrupt(&mut self) -> Result<TracedProcessGuard> {
        self.open_guards += 1;
        if self.open_guards == 1 {
            ptrace::interrupt(self.pid)?;
            wait::waitpid(Some(self.pid), None)?;
        }
        Ok(TracedProcessGuard {
            pid: self.pid,
            parent: self,
        })
    }
}

impl Drop for TracedProcess {
    fn drop(&mut self) {
        if let Err(e) = ptrace::interrupt(self.pid) {
            println!("In Drop cant interrupt {e}");
        } else if let Err(e) = wait::waitpid(Some(self.pid), None) {
            println!("In Drop cant wait {e}");
        } else if let Err(e) = ptrace::detach(self.pid, None) {
            println!("In Drop: {e}");
        };
    }
}

struct TracedProcessGuard<'a> {
    pid: Pid,
    parent: &'a mut TracedProcess,
}

impl Drop for TracedProcessGuard<'_> {
    fn drop(&mut self) {
        self.parent.open_guards -= 1;
        if self.parent.open_guards == 0 {
            if let Err(e) = ptrace::cont(self.pid, None) {
                println!("Also In Drop: {e}");
            };
        }
    }
}

impl TracedProcessGuard<'_> {
    fn get_regs(&self) -> Result<user_regs_struct> {
        ptrace::getregs(self.pid)
    }
    fn set_regs(&self, regs: user_regs_struct) -> Result<()> {
        ptrace::setregs(self.pid, regs)
    }

    fn write_memory(&self, addr: u64, data: &[u8]) -> Result<()> {
        let addr = addr as *mut c_void;

        let data_iter = data.iter().copied();
        let mut missing = 8 - (data.len() % 8);
        if missing == 8 {
            missing = 0;
        }
        let combined = data_iter
            .chain((0..missing).map(|_| 0u8))
            .array_chunks::<8>()
            .map(|i| {
                println!("{i:0>2x?}");
                u64::from_le_bytes(i)
            });

        for (word, offset) in combined.zip(std::iter::successors(Some(0isize), |i| Some(*i + 8))) {
            unsafe {
                let addr = addr.offset(offset);
                println!("offset: {addr:?}");
                println!("word: {word:x}");
                ptrace::write(self.pid, addr, word as *mut c_void)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::TracedProcess;
    use nix::{errno::Errno, unistd::Pid, Result};

    #[test]
    fn test() -> Result<()> {
        let args = std::env::args().collect::<Vec<_>>();
        let pid = args
            .get(1)
            .ok_or_else(|| {
                println!("Not enough args");
                Errno::UnknownErrno
            })?
            .parse()
            .or(Err(Errno::UnknownErrno))?;
        let shellcode = std::fs::read("src/shellcode").unwrap();
        let mut tracee = TracedProcess::seize(Pid::from_raw(pid))?;
        {
            let tracee_session = tracee.interrupt()?;
            let regs = tracee_session.get_regs()?;
            // regs.rax = 0x3c;
            // regs.rdi = 42;
            // let data = [0x0f, 0x05];
            // tracee_session.set_regs(regs)?;
            tracee_session.write_memory(regs.rip, &shellcode)?;
        }
        Ok(())
    }
}
