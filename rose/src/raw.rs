use libc::{
    __errno_location, c_int, c_long, c_ulong, kill as unsafe_kill, pid_t, ptrace,
    raise as unsafe_raise, strerror_r, user_regs_struct, waitpid as unsafe_waitpid, PTRACE_ATTACH,
    PTRACE_CONT, PTRACE_DETACH, PTRACE_EVENT_CLONE, PTRACE_EVENT_EXEC, PTRACE_EVENT_EXIT,
    PTRACE_EVENT_FORK, PTRACE_EVENT_STOP, PTRACE_EVENT_VFORK, PTRACE_EVENT_VFORK_DONE,
    PTRACE_GETEVENTMSG, PTRACE_GETREGS, PTRACE_INTERRUPT, PTRACE_LISTEN, PTRACE_O_EXITKILL,
    PTRACE_O_SUSPEND_SECCOMP, PTRACE_O_TRACECLONE, PTRACE_O_TRACEEXEC, PTRACE_O_TRACEEXIT,
    PTRACE_O_TRACEFORK, PTRACE_O_TRACESECCOMP, PTRACE_O_TRACESYSGOOD, PTRACE_O_TRACEVFORK,
    PTRACE_O_TRACEVFORKDONE, PTRACE_PEEKTEXT, PTRACE_POKETEXT, PTRACE_SEIZE, PTRACE_SETOPTIONS,
    PTRACE_SYSCALL, PTRACE_TRACEME, SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGFPE, SIGHUP,
    SIGILL, SIGINT, SIGIO, SIGKILL, SIGPIPE, SIGPROF, SIGPWR, SIGQUIT, SIGSEGV, SIGSTKFLT, SIGSTOP,
    SIGSYS, SIGTERM, SIGTRAP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGUSR1, SIGUSR2, SIGVTALRM,
    SIGWINCH, SIGXCPU, SIGXFSZ, WEXITSTATUS, WIFEXITED, WIFSIGNALED, WIFSTOPPED, WSTOPSIG,
    WTERMSIG, __WALL,
};
use std::{borrow::BorrowMut, error::Error, ffi::CStr, fmt::Display, mem, ptr};

#[derive(Debug)]
pub struct SyscallError {
    command: String,
    text: String,
}

impl SyscallError {
    #[must_use]
    pub fn new(command: &str, text: &str) -> Self {
        Self {
            command: command.to_string(),
            text: text.to_string(),
        }
    }
}

impl Display for SyscallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -- {}", self.command, self.text)
    }
}
impl Error for SyscallError {}

#[derive(Debug)]
pub enum ProcessState {
    Dead(Death),
    Event(Event),
    Group(Group),
    Signal(Signal),
    Syscall,
}

#[derive(Debug)]
pub enum Group {
    Stop,
    TermStop,
    TermIn,
    TermOut,
}

#[derive(Debug, Clone, Copy)]
pub enum Death {
    Exited(i32),
    Signaled(Signal),
}

#[derive(Debug)]
pub enum Event {
    Clone(pid_t),
    Exec(pid_t),
    Exit(i32),
    Fork(pid_t),
    VFork(pid_t),
    VForkDone(pid_t),
}

pub fn waitpid(pid: i32) -> Result<ProcessState, SyscallError> {
    reset_error();
    let mut status: c_int = 0;
    let ret = unsafe { unsafe_waitpid(pid, status.borrow_mut(), __WALL) };
    if ret == -1 {
        return Err(strerror("waitpid"));
    }
    if WIFSTOPPED(status) {
        let sig = WSTOPSIG(status);
        if sig == SIGTRAP {
            if status >> 8 == PTRACE_EVENT_CLONE << 8 | SIGTRAP {
                let msg = ptrace_geteventmsg(pid)?;
                Ok(ProcessState::Event(Event::Clone(msg)))
            } else if status >> 8 == PTRACE_EVENT_EXEC << 8 | SIGTRAP {
                let msg = ptrace_geteventmsg(pid)?;
                Ok(ProcessState::Event(Event::Exec(msg)))
            } else if status >> 8 == PTRACE_EVENT_EXIT << 8 | SIGTRAP {
                let msg = ptrace_geteventmsg(pid)?;
                Ok(ProcessState::Event(Event::Exit(msg)))
            } else if status >> 8 == PTRACE_EVENT_FORK << 8 | SIGTRAP {
                let msg = ptrace_geteventmsg(pid)?;
                Ok(ProcessState::Event(Event::Fork(msg)))
            } else if status >> 8 == PTRACE_EVENT_VFORK << 8 | SIGTRAP {
                let msg = ptrace_geteventmsg(pid)?;
                Ok(ProcessState::Event(Event::VFork(msg)))
            } else if status >> 8 == PTRACE_EVENT_VFORK_DONE << 8 | SIGTRAP {
                let msg = ptrace_geteventmsg(pid)?;
                Ok(ProcessState::Event(Event::VForkDone(msg)))
            } else {
                Ok(ProcessState::Signal(sig.into()))
            }
        } else if sig == SIGTRAP | 0x80 {
            Ok(ProcessState::Syscall)
        } else if status >> 16 == PTRACE_EVENT_STOP {
            if sig == SIGSTOP {
                Ok(ProcessState::Group(Group::Stop))
            } else if sig == SIGTSTP {
                Ok(ProcessState::Group(Group::TermStop))
            } else if sig == SIGTTIN {
                Ok(ProcessState::Group(Group::TermIn))
            } else if sig == SIGTTOU {
                Ok(ProcessState::Group(Group::TermOut))
            } else {
                Err(SyscallError::new("waitpid", "Unknown event stop type"))
            }
        } else {
            Ok(ProcessState::Signal(sig.into()))
        }
    } else if WIFEXITED(status) {
        let exitcode = WEXITSTATUS(status);
        Ok(ProcessState::Dead(Death::Exited(exitcode)))
    } else if WIFSIGNALED(status) {
        let signo = WTERMSIG(status);
        Ok(ProcessState::Dead(Death::Signaled(signo.into())))
    } else {
        Err(SyscallError::new("waitpid", "Unknown wait type"))
    }
}

fn reset_error() {
    unsafe { *__errno_location() = 0 };
}

fn strerror<T>(command: &T) -> SyscallError
where
    T: AsRef<str> + ?Sized,
{
    let errno = unsafe { *__errno_location() };
    unsafe { *__errno_location() = 0 };
    let mut buf = [0i8; 64];
    let ret = unsafe { strerror_r(errno, buf.as_mut_ptr(), buf.len()) };
    if ret == 0 {
        SyscallError::new(
            command.as_ref(),
            unsafe { CStr::from_ptr(buf.as_ptr()) }
                .to_string_lossy()
                .as_ref(),
        )
    } else {
        SyscallError::new("unknown", "Failed to get error string")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Signal {
    Hup,
    Int,
    Quit,
    Ill,
    Trap,
    Abrt,
    Bus,
    Fpe,
    Kill,
    Usr1,
    Segv,
    Usr2,
    Pipe,
    Alrm,
    Term,
    Stkflt,
    Chld,
    Cont,
    Stop,
    Tstp,
    Ttin,
    Ttou,
    Urg,
    Xcpu,
    Xfsz,
    Vtalrm,
    Prof,
    Winch,
    Io,
    Pwr,
    Sys,
    Uknown(i32),
}

impl Signal {
    const fn get_val(self) -> i32 {
        match self {
            Self::Hup => SIGHUP,
            Self::Int => SIGINT,
            Self::Quit => SIGQUIT,
            Self::Ill => SIGILL,
            Self::Trap => SIGTRAP,
            Self::Abrt => SIGABRT,
            Self::Bus => SIGBUS,
            Self::Fpe => SIGFPE,
            Self::Kill => SIGKILL,
            Self::Usr1 => SIGUSR1,
            Self::Segv => SIGSEGV,
            Self::Usr2 => SIGUSR2,
            Self::Pipe => SIGPIPE,
            Self::Alrm => SIGALRM,
            Self::Term => SIGTERM,
            Self::Stkflt => SIGSTKFLT,
            Self::Chld => SIGCHLD,
            Self::Cont => SIGCONT,
            Self::Stop => SIGSTOP,
            Self::Tstp => SIGTSTP,
            Self::Ttin => SIGTTIN,
            Self::Ttou => SIGTTOU,
            Self::Urg => SIGURG,
            Self::Xcpu => SIGXCPU,
            Self::Xfsz => SIGXFSZ,
            Self::Vtalrm => SIGVTALRM,
            Self::Prof => SIGPROF,
            Self::Winch => SIGWINCH,
            Self::Io => SIGIO,
            Self::Pwr => SIGPWR,
            Self::Sys => SIGSYS,
            Self::Uknown(unknown) => unknown,
        }
    }
}

impl From<i32> for Signal {
    fn from(value: i32) -> Self {
        match value {
            SIGHUP => Self::Hup,
            SIGINT => Self::Int,
            SIGQUIT => Self::Quit,
            SIGILL => Self::Ill,
            SIGTRAP => Self::Trap,
            SIGABRT => Self::Abrt,
            SIGBUS => Self::Bus,
            SIGFPE => Self::Fpe,
            SIGKILL => Self::Kill,
            SIGUSR1 => Self::Usr1,
            SIGSEGV => Self::Segv,
            SIGUSR2 => Self::Usr2,
            SIGPIPE => Self::Pipe,
            SIGALRM => Self::Alrm,
            SIGTERM => Self::Term,
            SIGSTKFLT => Self::Stkflt,
            SIGCHLD => Self::Chld,
            SIGCONT => Self::Cont,
            SIGSTOP => Self::Stop,
            SIGTSTP => Self::Tstp,
            SIGTTIN => Self::Ttin,
            SIGTTOU => Self::Ttou,
            SIGURG => Self::Urg,
            SIGXCPU => Self::Xcpu,
            SIGXFSZ => Self::Xfsz,
            SIGVTALRM => Self::Vtalrm,
            SIGPROF => Self::Prof,
            SIGWINCH => Self::Winch,
            SIGIO => Self::Io,
            SIGPWR => Self::Pwr,
            SIGSYS => Self::Sys,
            unknown => Self::Uknown(unknown),
        }
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hup => write!(f, "Hup"),
            Self::Int => write!(f, "Int"),
            Self::Quit => write!(f, "Quit"),
            Self::Ill => write!(f, "Ill"),
            Self::Trap => write!(f, "Trap"),
            Self::Abrt => write!(f, "Abrt"),
            Self::Bus => write!(f, "Bus"),
            Self::Fpe => write!(f, "Fpe"),
            Self::Kill => write!(f, "Kill"),
            Self::Usr1 => write!(f, "Usr1"),
            Self::Segv => write!(f, "Segv"),
            Self::Usr2 => write!(f, "Usr2"),
            Self::Pipe => write!(f, "Pipe"),
            Self::Alrm => write!(f, "Alrm"),
            Self::Term => write!(f, "Term"),
            Self::Stkflt => write!(f, "Stkflt"),
            Self::Chld => write!(f, "Chld"),
            Self::Cont => write!(f, "Cont"),
            Self::Stop => write!(f, "Stop"),
            Self::Tstp => write!(f, "Tstp"),
            Self::Ttin => write!(f, "Ttin"),
            Self::Ttou => write!(f, "Ttou"),
            Self::Urg => write!(f, "Urg"),
            Self::Xcpu => write!(f, "Xcpu"),
            Self::Xfsz => write!(f, "Xfsz"),
            Self::Vtalrm => write!(f, "Vtalrm"),
            Self::Prof => write!(f, "Prof"),
            Self::Winch => write!(f, "Winch"),
            Self::Io => write!(f, "Io"),
            Self::Pwr => write!(f, "Pwr"),
            Self::Sys => write!(f, "Sys"),
            Self::Uknown(sig) => write!(f, "Uknown({sig})"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SignalInject {
    Suppress,
    Inject(Signal),
}

impl SignalInject {
    const fn get_val(self) -> i32 {
        match self {
            Self::Suppress => 0,
            Self::Inject(sig) => sig.get_val(),
        }
    }
}

impl From<Signal> for SignalInject {
    fn from(value: Signal) -> Self {
        Self::Inject(value)
    }
}

pub fn kill(pid: pid_t, sig: Signal) -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { unsafe_kill(pid, sig.get_val()) };
    if ret == -1 {
        Err(strerror("kill"))
    } else {
        Ok(())
    }
}

#[allow(unused)]
pub fn raise(sig: Signal) -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { unsafe_raise(sig.get_val()) };
    if ret == -1 {
        Err(strerror("raise"))
    } else {
        Ok(())
    }
}

pub fn ptrace_geteventmsg(pid: pid_t) -> Result<c_int, SyscallError> {
    reset_error();
    let mut msg: c_ulong = 0;
    let ret = unsafe { ptrace(PTRACE_GETEVENTMSG, pid, 0, ptr::from_mut(&mut msg)) };
    if ret == -1 {
        Err(strerror("geteventmsg"))
    } else {
        let msg: i32 = msg
            .try_into()
            .map_err(|_| SyscallError::new("geteventmsg", "msg was not a i32"))?;
        Ok(msg)
    }
}
#[allow(unused)]
pub fn ptrace_trace_me() -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { ptrace(PTRACE_TRACEME, 0, 0, 0) };
    if ret == -1 {
        Err(strerror("trace_me"))
    } else {
        Ok(())
    }
}

pub fn ptrace_seize(pid: pid_t, options: PtraceOptionList) -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { ptrace(PTRACE_SEIZE, pid, 0, options.inner) };
    if ret != 0 {
        Err(strerror("seize"))
    } else {
        Ok(())
    }
}

#[allow(unused)]
pub fn ptrace_attach(pid: pid_t) -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { ptrace(PTRACE_ATTACH, pid, 0, 0) };
    if ret != 0 {
        Err(strerror("attach"))
    } else {
        Ok(())
    }
}

pub fn ptrace_inturrupt(pid: pid_t) -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { ptrace(PTRACE_INTERRUPT, pid, 0, 0) };
    if ret != 0 {
        Err(strerror("inturrupt"))
    } else {
        Ok(())
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum PtraceOption {
    EXITKILL,
    TRACECLONE,
    TRACEEXEC,
    TRACEEXIT,
    TRACEFORK,
    TRACESYSGOOD,
    TRACEVFORK,
    TRACEVFORKDONE,
    TRACESECCOMP,
    SUSPEND_SECCOMP,
}

impl PtraceOption {
    const fn get_val(&self) -> c_int {
        match self {
            Self::EXITKILL => PTRACE_O_EXITKILL,
            Self::TRACECLONE => PTRACE_O_TRACECLONE,
            Self::TRACEEXEC => PTRACE_O_TRACEEXEC,
            Self::TRACEEXIT => PTRACE_O_TRACEEXIT,
            Self::TRACEFORK => PTRACE_O_TRACEFORK,
            Self::TRACESYSGOOD => PTRACE_O_TRACESYSGOOD,
            Self::TRACEVFORK => PTRACE_O_TRACEVFORK,
            Self::TRACEVFORKDONE => PTRACE_O_TRACEVFORKDONE,
            Self::TRACESECCOMP => PTRACE_O_TRACESECCOMP,
            Self::SUSPEND_SECCOMP => PTRACE_O_SUSPEND_SECCOMP,
        }
    }
}
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct PtraceOptionList {
    inner: i32,
}

impl Default for PtraceOptionList {
    fn default() -> Self {
        Self {
            inner: PTRACE_O_EXITKILL
                | PTRACE_O_TRACECLONE
                | PTRACE_O_TRACEEXEC
                | PTRACE_O_TRACEEXIT
                | PTRACE_O_TRACEFORK
                | PTRACE_O_TRACESYSGOOD
                | PTRACE_O_TRACEVFORK
                | PTRACE_O_TRACEVFORKDONE,
        }
    }
}

impl Display for PtraceOptionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = Vec::new();
        let inner = self.inner;
        if (inner & PTRACE_O_EXITKILL) > 0 {
            out.push("Exit Kill");
        }
        if (inner & PTRACE_O_TRACECLONE) > 0 {
            out.push("Clone");
        }
        if (inner & PTRACE_O_TRACEEXEC) > 0 {
            out.push("Exec");
        }
        if (inner & PTRACE_O_TRACEEXIT) > 0 {
            out.push("Exit");
        }
        if (inner & PTRACE_O_TRACESYSGOOD) > 0 {
            out.push("Sys Good");
        }
        if (inner & PTRACE_O_TRACEFORK) > 0 {
            out.push("Fork");
        }
        if (inner & PTRACE_O_TRACEVFORK) > 0 {
            out.push("VFork");
        }
        if (inner & PTRACE_O_TRACEVFORKDONE) > 0 {
            out.push("VFork Done");
        }
        if (inner & PTRACE_O_TRACESECCOMP) > 0 {
            out.push("Seccomp");
        }
        if (inner & PTRACE_O_SUSPEND_SECCOMP) > 0 {
            out.push("Suspend Seccomp");
        }
        let joined = out.join(", ");
        write!(f, "{joined}")
    }
}

impl PtraceOptionList {
    #[allow(clippy::needless_pass_by_value)]
    pub fn set(&mut self, option: PtraceOption) -> &mut Self {
        self.inner |= option.get_val();
        self
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn unset(&mut self, option: PtraceOption) -> &mut Self {
        self.inner &= !option.get_val();
        self
    }
}
pub fn ptrace_set_options(pid: pid_t, options: PtraceOptionList) -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { ptrace(PTRACE_SETOPTIONS, pid, 0, options.inner) };
    if ret != 0 {
        Err(strerror("set_options"))
    } else {
        Ok(())
    }
}
#[derive(Debug, Clone, Copy)]
pub enum RestartCommand {
    Continue,
    Listen,
    Detach,
    Syscall,
}

impl RestartCommand {
    pub(crate) fn restart(self, pid: pid_t, sig: SignalInject) -> Result<(), SyscallError> {
        match self {
            Self::Continue => ptrace_continue(pid, sig),
            Self::Listen => ptrace_listen(pid),
            Self::Detach => ptrace_detach(pid, sig),
            Self::Syscall => ptrace_syscall(pid, sig),
        }
    }
}

pub fn ptrace_continue(pid: pid_t, sig: SignalInject) -> Result<(), SyscallError> {
    let sig = sig.get_val();
    reset_error();
    let ret = unsafe { ptrace(PTRACE_CONT, pid, 0, sig) };
    if ret != 0 {
        Err(strerror("continue"))
    } else {
        Ok(())
    }
}
pub fn ptrace_syscall(pid: pid_t, sig: SignalInject) -> Result<(), SyscallError> {
    let sig = sig.get_val();
    reset_error();
    let ret = unsafe { ptrace(PTRACE_SYSCALL, pid, 0, sig) };
    if ret != 0 {
        Err(strerror("syscall"))
    } else {
        Ok(())
    }
}
pub fn ptrace_listen(pid: pid_t) -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { ptrace(PTRACE_LISTEN, pid, 0, 0) };
    if ret != 0 {
        Err(strerror("listen"))
    } else {
        Ok(())
    }
}
pub fn ptrace_detach(pid: pid_t, sig: SignalInject) -> Result<(), SyscallError> {
    let sig = sig.get_val();
    reset_error();
    let ret = unsafe { ptrace(PTRACE_DETACH, pid, 0, sig) };
    if ret != 0 {
        Err(strerror("continue"))
    } else {
        Ok(())
    }
}

pub fn ptrace_getregs(pid: pid_t) -> Result<user_regs_struct, SyscallError> {
    reset_error();
    let mut regs: user_regs_struct = unsafe { mem::zeroed() };
    let ret = unsafe { ptrace(PTRACE_GETREGS, pid, 0, &mut regs) };
    if ret != 0 {
        Err(strerror("get_regs"))
    } else {
        Ok(regs)
    }
}

pub fn ptrace_peektext(pid: pid_t, addr: c_long) -> Result<c_long, SyscallError> {
    reset_error();
    let val = unsafe { ptrace(PTRACE_PEEKTEXT, pid, addr, 0) };
    let errno = unsafe { *__errno_location() };
    if errno != 0 {
        Err(strerror("peek_text"))
    } else {
        Ok(val)
    }
}
pub fn ptrace_poketext(pid: pid_t, addr: c_long, data: c_long) -> Result<(), SyscallError> {
    reset_error();
    let ret = unsafe { ptrace(PTRACE_POKETEXT, pid, addr, data) };
    if ret != 0 {
        Err(strerror("poke_text"))
    } else {
        Ok(())
    }
}
