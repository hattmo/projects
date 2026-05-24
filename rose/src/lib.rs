#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
//#![warn(missing_docs)]

mod raw;
use libc::pid_t;
pub use libc::{c_long, user_regs_struct};
use raw::Death;
pub use raw::{
    ProcessState, PtraceOption, PtraceOptionList, RestartCommand, Signal, SignalInject,
    SyscallError,
};
use std::sync::{Mutex, PoisonError};
use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{ChildStderr, ChildStdin, ChildStdout, Command},
};

#[allow(unused)]
use tracing::{error, info, info_span, instrument};

const WORD_SIZE: usize = (c_long::BITS / 8) as usize;

/// An error occured while attempting to spawn a subprocess under trace
#[derive(Debug)]
pub enum SpawnTracedError {
    /// Error occured during a syscall.
    Syscall(SyscallError),
    /// Error spawning the child process.
    Io(io::Error),
    /// Child process died before trace was setup.
    ChildDied,
    /// Panic occured in the setup thread.
    SetupPanicked,
}

impl From<SyscallError> for SpawnTracedError {
    fn from(value: SyscallError) -> Self {
        Self::Syscall(value)
    }
}
impl From<io::Error> for SpawnTracedError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for SpawnTracedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syscall(err) => write!(f, "Error during syscall: ({err})"),
            Self::Io(err) => write!(f, "Io error: ({err})"),
            Self::ChildDied => write!(f, "The child died unexpectedly"),
            Self::SetupPanicked => write!(f, "The setup thread panicked"),
        }
    }
}
impl Error for SpawnTracedError {}
/// Implemented on `std::process::Command` to spawn new child as a the tracee of the current process.
pub trait TracedCommand {
    /// Spawn a child process as a tracee of the current process.
    /// # Errors
    ///
    fn spawn_traced(
        self,
        start_mode: RestartCommand,
        signal: SignalInject,
    ) -> Result<TracedProcess, SpawnTracedError>;
}

impl TracedCommand for &mut Command {
    #[allow(clippy::cast_possible_wrap)]
    #[instrument]
    fn spawn_traced(
        self,
        start_mode: RestartCommand,
        signal: SignalInject,
    ) -> Result<TracedProcess, SpawnTracedError> {
        let mut proc =
            unsafe { self.pre_exec(move || raw::ptrace_trace_me().map_err(io::Error::other)) }
                .spawn()?;
        let pid = proc.id() as pid_t;
        let status = raw::waitpid(pid)?;
        info!(status=?status, "first wait");
        raw::ptrace_detach(pid, Signal::Stop.into())?;
        let options = PtraceOptionList::default();
        raw::ptrace_seize(pid, options)?;
        raw::ptrace_inturrupt(pid)?;
        let status = raw::waitpid(pid)?;
        info!(status=?status, "second wait");
        start_mode.restart(pid, signal)?;
        Ok(TracedProcess {
            tid: pid,
            tgid: pid,
            options,
            restart_command: start_mode,
            stdin: proc.stdin.take(),
            stdout: proc.stdout.take(),
            stderr: proc.stderr.take(),
            is_dead: None,
        })
    }
}

impl Debug for TracedProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TracedCommand")
            .field("tid", &self.tid)
            .field("tgid", &self.tgid)
            .finish_non_exhaustive()
    }
}

/// A single entry of the processes memory maps.
#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct MemMap {
    pub start_addr: u64,
    pub end_addr: u64,
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub shared: bool,
    pub private: bool,
    pub offset: u32,
    pub dev_major: u8,
    pub dev_minor: u8,
    pub inode: u32,
    pub path: Option<String>,
}

/// A event that has occured in the traced process.
#[derive(Debug)]
pub enum TracedEvent<'a, 'b, 'c, O> {
    /// The process was inturrupted by a signal.
    SignalEvent(&'a mut StoppedEvent<'b, O>, &'c mut SignalEvent),
    Clone(&'a mut StoppedEvent<'b, O>, &'c mut CloneEvent),
    Exit(&'a mut StoppedEvent<'b, O>, &'c mut ExitEvent),
    Syscall(&'a mut StoppedEvent<'b, O>, &'c mut SyscallEvent<O>),
}

/// The reason `wait_events` exited.
#[derive(Debug, Clone, Copy)]
pub enum TracedExitStatus<T> {
    Dead(Death),
    /// The callback function stopped the event handler. The process may still be running.
    Break(T),
}

/// An error occured while waiting for an event.
#[derive(Debug)]
pub enum WaitEventsError {
    /// There was an error during a syscall.
    Syscall(SyscallError),
    Io(io::Error),
    Lock,
    Bounds,
}

impl From<SyscallError> for WaitEventsError {
    fn from(value: SyscallError) -> Self {
        Self::Syscall(value)
    }
}

impl From<MemoryProcError> for WaitEventsError {
    fn from(value: MemoryProcError) -> Self {
        match value {
            MemoryProcError::Lock => Self::Lock,
            MemoryProcError::Io(err) => Self::Io(err),
        }
    }
}

impl From<MemoryError> for WaitEventsError {
    fn from(value: MemoryError) -> Self {
        match value {
            MemoryError::Syscall(err) => Self::Syscall(err),
            MemoryError::Bounds => Self::Bounds,
            MemoryError::Io(err) => Self::Io(err),
        }
    }
}

impl From<GetMapsError> for WaitEventsError {
    fn from(GetMapsError(err): GetMapsError) -> Self {
        Self::Io(err)
    }
}

impl<T> From<PoisonError<T>> for WaitEventsError {
    fn from(_value: PoisonError<T>) -> Self {
        Self::Lock
    }
}

impl Display for WaitEventsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syscall(err) => write!(f, "Syscall error: ({err})"),
            Self::Io(err) => write!(f, "Io error: ({err})"),
            Self::Lock => write!(f, "Error locking mutext"),
            Self::Bounds => write!(
                f,
                "Memory buffer was not a multiple of word size ({WORD_SIZE})"
            ),
        }
    }
}
impl Error for WaitEventsError {}
/// An error occured while reading from /proc/${pid}/mem.
#[derive(Debug)]
pub enum MemoryProcError {
    /// The underlying muted was poisoned.
    Lock,
    /// There was an Io error.
    Io(io::Error),
}

impl From<io::Error> for MemoryProcError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
impl<T> From<PoisonError<T>> for MemoryProcError {
    fn from(_value: PoisonError<T>) -> Self {
        Self::Lock
    }
}

impl Display for MemoryProcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lock => write!(f, "Error locking mutext"),
            Self::Io(err) => write!(f, "Io error: ({err})"),
        }
    }
}

impl Error for MemoryProcError {}

/// An error occured while reading from /proc/${pid}/maps.
#[derive(Debug)]
pub struct GetMapsError(io::Error);

impl From<io::Error> for GetMapsError {
    fn from(value: io::Error) -> Self {
        Self(value)
    }
}

impl Display for GetMapsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(err) = self;
        write!(f, "Io error: ({err})")
    }
}

impl Error for GetMapsError {}
/// A handle to the running process under trace.  Behaves
/// similar to a std Child.
pub struct TracedProcess {
    tid: i32,
    tgid: i32,
    is_dead: Option<Death>,
    options: PtraceOptionList,
    restart_command: RestartCommand,
    /// Handle to the traced process stdin.
    pub stdin: Option<ChildStdin>,
    /// Handle to the traced process stdout.
    pub stdout: Option<ChildStdout>,
    /// Handle to the traced process stderr.
    pub stderr: Option<ChildStderr>,
}
type SyscallCB<O> = Box<dyn FnOnce(&mut StoppedEvent<O>) -> Result<(), WaitEventsError>>;
impl TracedProcess {
    /// Wait for incoming ptrace events until the process exits.
    #[instrument(skip(handler, self))]
    pub fn wait_events<O, T>(
        &mut self,
        mut handler: T,
    ) -> Result<TracedExitStatus<O>, WaitEventsError>
    where
        T: FnMut(&mut TracedEvent<O>) -> Result<(), WaitEventsError>,
    {
        let mut syscall_exit_cb: Option<SyscallCB<O>> = None;
        let mut expect_syscall_exit = false;
        let mut stop_wait = None;
        loop {
            let status = raw::waitpid(self.tid)?;
            let old_options = self.options;
            let mut stop_event = StoppedEvent::new(
                self.tid,
                &mut self.options,
                &mut stop_wait,
                &mut self.restart_command,
            );
            let inject = match status {
                ProcessState::Dead(death) => {
                    let exit = TracedExitStatus::Dead(death);
                    self.is_dead = Some(death);
                    return Ok(exit);
                }
                ProcessState::Syscall => {
                    if expect_syscall_exit {
                        if let Some(cb) = syscall_exit_cb.take() {
                            cb(&mut stop_event)?;
                        }
                        expect_syscall_exit = false;
                    } else {
                        let mut syscall_event = SyscallEvent::new();
                        let mut event = TracedEvent::Syscall(&mut stop_event, &mut syscall_event);
                        handler(&mut event)?;
                        syscall_exit_cb = syscall_event.on_return.take();
                        if matches!(self.restart_command, RestartCommand::Syscall) {
                            expect_syscall_exit = true;
                        } else {
                            syscall_exit_cb = None;
                        }
                    }
                    SignalInject::Suppress
                }
                ProcessState::Group(_sig) => {
                    todo!("handle group stop")
                }
                ProcessState::Event(ptrace_event) => {
                    match ptrace_event {
                        raw::Event::Clone(new_tid) => {
                            // TODO: should be able to override restart command
                            let new_trace =
                                Self::from_tid(new_tid, self.tgid, RestartCommand::Continue);
                            let mut clone_event = CloneEvent::new(new_trace);
                            let mut event = TracedEvent::Clone(&mut stop_event, &mut clone_event);
                            handler(&mut event)?;
                            if let Some(trace) = clone_event.new_trace {
                                info!(trace=?trace, "Detaching clone");
                                raw::ptrace_detach(new_tid, SignalInject::Suppress)?;
                                raw::kill(new_tid, Signal::Cont)?;
                            }
                            SignalInject::Suppress
                        }
                        raw::Event::Exec(_) => todo!(),
                        raw::Event::Exit(exit_code) => {
                            let mut exit_event = ExitEvent::new(exit_code);
                            let mut event = TracedEvent::Exit(&mut stop_event, &mut exit_event);
                            handler(&mut event)?;
                            SignalInject::Suppress
                        }
                        raw::Event::Fork(_) => todo!(),
                        raw::Event::VFork(_) => todo!(),
                        raw::Event::VForkDone(_) => todo!(),
                    }
                }
                ProcessState::Signal(signal) => {
                    let mut sig_event = SignalEvent::new(signal);
                    let mut event = TracedEvent::SignalEvent(&mut stop_event, &mut sig_event);
                    handler(&mut event)?;
                    sig_event.sig_inject
                }
            };
            if old_options != self.options {
                raw::ptrace_set_options(self.tid, self.options)?;
            }
            self.restart_command.restart(self.tid, inject)?;
            if let Some(val) = stop_wait {
                return Ok(TracedExitStatus::Break(val));
            }
        }
    }

    #[must_use]
    pub fn get_handle(&self) -> TracedProcessHandle {
        TracedProcessHandle::new(self.tid)
    }

    fn from_tid(tid: pid_t, tgid: pid_t, restart_command: RestartCommand) -> Self {
        Self {
            tid,
            tgid,
            restart_command,
            options: PtraceOptionList::default(),
            stdin: None,
            stdout: None,
            stderr: None,
            is_dead: None,
        }
    }
}

#[derive(Debug)]
pub struct TracedProcessHandle {
    tid: i32,
    maps_path: PathBuf,
    mem_file: Mutex<Option<File>>,
}

impl TracedProcessHandle {
    fn new(tid: i32) -> Self {
        Self {
            tid,
            mem_file: None.into(),
            maps_path: format!("/proc/{tid}/maps").into(),
        }
    }

    ///
    /// # Errors
    pub fn get_memory_proc(&self, addr: u64, buf: &mut [u8]) -> Result<(), MemoryProcError> {
        let mut proc_lock = self.mem_file.lock()?;
        let proc_file = proc_lock.take();
        let mut file = match proc_file {
            Some(file) => file,
            None => OpenOptions::new()
                .read(true)
                .create(false)
                .write(false)
                .create_new(false)
                .append(false)
                .truncate(false)
                .open(format!("/proc/{}/mem", self.tid))?,
        };

        file.seek(io::SeekFrom::Start(addr))?;
        file.read_exact(buf)?;
        *proc_lock = Some(file);
        drop(proc_lock);
        Ok(())
    }
    ///
    /// # Errors
    pub fn get_maps(&self) -> Result<Vec<MemMap>, GetMapsError> {
        let mut maps = String::new();
        OpenOptions::new()
            .read(true)
            .write(false)
            .open(&self.maps_path)?
            .read_to_string(&mut maps)?;
        Ok(maps
            .lines()
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                // address
                let (start, end) = parts.next()?.split_once('-')?;
                let start_addr = u64::from_str_radix(start, 16).ok()?;
                let end_addr = u64::from_str_radix(end, 16).ok()?;
                //perms
                let mut perms = parts.next()?.chars();
                let read = perms.next()? == 'r';
                let write = perms.next()? == 'w';
                let execute = perms.next()? == 'x';
                let scope = perms.next()?;
                let shared = scope == 's';
                let private = scope == 'p';
                //offset
                let offset = u32::from_str_radix(parts.next()?, 16).ok()?;
                // dev
                let (maj, min) = parts.next()?.split_once(':')?;
                let dev_major = maj.parse().ok()?;
                let dev_minor = min.parse().ok()?;
                // inode
                let inode = parts.next()?.parse().ok()?;
                // path
                let path: String = parts.collect();
                let path = (!path.is_empty()).then_some(path);
                Some(MemMap {
                    start_addr,
                    end_addr,
                    read,
                    write,
                    execute,
                    shared,
                    private,
                    offset,
                    dev_major,
                    dev_minor,
                    inode,
                    path,
                })
            })
            .collect())
    }
    ///
    /// # Errors
    pub fn send_signal(&self, sig: Signal) -> Result<(), SyscallError> {
        raw::kill(self.tid, sig)
    }

    ///
    /// # Errors
    pub fn send_inturrupt(&self) -> Result<(), SyscallError> {
        unimplemented!("Must be called from the trace thread");
        //        raw::ptrace_inturrupt(self.tid)
    }
}

#[derive(Debug)]
pub enum MemoryError {
    Syscall(SyscallError),
    Bounds,
    Io(io::Error),
}

impl From<SyscallError> for MemoryError {
    fn from(value: SyscallError) -> Self {
        Self::Syscall(value)
    }
}

impl From<io::Error> for MemoryError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Syscall(err) => write!(f, "Error during syscall: ({err})"),
            Self::Io(err) => write!(f, "Io error: ({err})"),
            Self::Bounds => write!(f, "Buffer was not a multiple of {WORD_SIZE}"),
        }
    }
}

impl Error for MemoryError {}

#[derive(Debug)]
pub struct StoppedEvent<'a, T> {
    tid: i32,
    options: &'a mut PtraceOptionList,
    stop_wait: &'a mut Option<T>,
    restart_command: &'a mut RestartCommand,
}

impl<'a, T> StoppedEvent<'a, T> {
    fn new(
        tid: i32,
        options: &'a mut PtraceOptionList,
        stop_wait: &'a mut Option<T>,
        restart_command: &'a mut RestartCommand,
    ) -> Self {
        Self {
            tid,
            options,
            stop_wait,
            restart_command,
        }
    }

    ///
    /// # Errors
    pub fn get_regs(&self) -> Result<user_regs_struct, MemoryError> {
        raw::ptrace_getregs(self.tid).map_err(MemoryError::Syscall)
    }

    ///
    /// # Errors
    #[allow(clippy::cast_possible_wrap)]
    pub fn get_memory(&self, mut addr: i64, mut buf: &mut [u8]) -> Result<(), MemoryError> {
        if buf.len() % WORD_SIZE != 0 {
            return Err(MemoryError::Bounds);
        }
        loop {
            let mem = raw::ptrace_peektext(self.tid, addr)?.to_le_bytes();
            let ret = buf.write(&mem)?;
            if ret == 0 {
                break Ok(());
            };
            addr += WORD_SIZE as i64;
        }
    }

    ///
    /// # Errors
    #[allow(clippy::cast_possible_wrap)]
    pub fn get_string(&self, mut addr: i64) -> Result<String, MemoryError> {
        let mut buf: Vec<u8> = Vec::new();
        for _ in 0..1024 {
            let mem = raw::ptrace_peektext(self.tid, addr)?.to_le_bytes();
            if mem.contains(&0) {
                buf.extend(mem.into_iter().take_while(|c| *c != 0));
                break;
            }
            buf.extend(mem);
            addr += WORD_SIZE as i64;
        }
        return Ok(String::from_utf8_lossy(&buf).to_string());
    }

    ///
    /// # Errors
    #[allow(clippy::cast_possible_wrap)]
    pub fn put_memory(&self, mut addr: i64, mut buf: &[u8]) -> Result<(), MemoryError> {
        if buf.len() % WORD_SIZE != 0 {
            return Err(MemoryError::Bounds);
        }

        let mut word: [u8; WORD_SIZE] = Default::default();
        loop {
            let ret = buf.read(&mut word)?;
            if ret != WORD_SIZE {
                break Ok(());
            }
            let word = i64::from_le_bytes(word);
            raw::ptrace_poketext(self.tid, addr, word)?;
            addr += WORD_SIZE as i64;
        }
    }
    pub fn stop_wait(&mut self, val: T) {
        *self.stop_wait = Some(val);
    }
    pub fn options(&mut self) -> &mut PtraceOptionList {
        self.options
    }
    pub fn set_restart_command(&mut self, command: RestartCommand) {
        *self.restart_command = command;
    }
}

#[derive(Debug)]
pub struct SignalEvent {
    sig: Signal,
    sig_inject: SignalInject,
}

impl SignalEvent {
    const fn new(sig: Signal) -> Self {
        Self {
            sig_inject: SignalInject::Inject(sig),
            sig,
        }
    }
    #[must_use]
    pub const fn get_sig(&self) -> &Signal {
        &self.sig
    }
    pub fn supress(&mut self) {
        self.sig_inject = SignalInject::Suppress;
    }
}

#[derive(Debug)]
pub struct CloneEvent {
    new_trace: Option<TracedProcess>,
}
impl CloneEvent {
    const fn new(new_trace: TracedProcess) -> Self {
        Self {
            new_trace: Some(new_trace),
        }
    }
    pub fn take_trace(&mut self) -> Option<TracedProcess> {
        self.new_trace.take()
    }
}

#[derive(Debug)]
pub struct ExitEvent {
    pub exit_code: i32,
}

impl ExitEvent {
    const fn new(exit_code: i32) -> Self {
        Self { exit_code }
    }
}

pub struct SyscallEvent<T> {
    on_return: Option<SyscallCB<T>>,
}

impl<T> Debug for SyscallEvent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyscallEvent").finish_non_exhaustive()
    }
}

impl<'a, O> SyscallEvent<O> {
    fn new() -> Self {
        Self { on_return: None }
    }
    pub fn get_return<T>(&'a mut self, cb: T)
    where
        T: FnOnce(&mut StoppedEvent<O>) -> Result<(), WaitEventsError> + 'static,
    {
        self.on_return = Some(Box::new(cb));
    }
}
