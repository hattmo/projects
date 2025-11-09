use std::{
    ops::Deref,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicUsize, Ordering::SeqCst},
        mpsc::{self, Receiver, Sender},
    },
    thread,
    time::Duration,
};

pub struct Semephore {
    inner: Arc<(Mutex<usize>, Condvar)>,
    max: Option<usize>,
}

pub struct Ticket<'a> {
    source: &'a Semephore,
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        let (mutex, cond) = self.source.inner.deref();
        let mut count = mutex.lock().unwrap();
        *count -= 1;
        cond.notify_one();
    }
}

impl Semephore {
    pub fn take<'a>(&'a self) -> Ticket<'a> {
        let (mutex, cond) = self.inner.deref();
        let lock = mutex.lock().unwrap();
        let mut lock = if let Some(max) = self.max {
            cond.wait_while(lock, |&mut v| v >= max).unwrap()
        } else {
            lock
        };
        *lock += 1;
        Ticket { source: self }
    }
}

pub struct Pool<T> {
    min: usize,
    max: Option<usize>,
    timeout: Option<Duration>,
    tx: Sender<T>,
    rx: Arc<Mutex<Receiver<T>>>,
    waiting: Arc<AtomicUsize>,
    threads: Arc<AtomicUsize>,
}

pub struct PoolBuilder {
    min: usize,
    max: Option<usize>,
    timeout: Option<Duration>,
}

impl<T> Pool<T> {
    pub fn task(&self, task: T) {
        self.tx.send(task).unwrap();
    }
}

impl PoolBuilder {
    pub fn new() -> Self {
        let builder = Self {
            min: 0,
            max: None,
            timeout: None,
        };
        builder
    }
    pub fn start<T, F>(self, spawner: F) -> Pool<T>
    where
        F: FnMut(T) + Clone + Send + Sync + 'static,
        T: Send + 'static,
    {
        let PoolBuilder { min, max, timeout } = self;

        let waiting = Arc::new(AtomicUsize::new(0));
        let threads = Arc::new(AtomicUsize::new(min));

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..min {
            spawn_thread(
                rx.clone(),
                spawner.clone(),
                waiting.clone(),
                threads.clone(),
                timeout,
            );
        }
        Pool {
            min,
            max,
            tx,
            rx,
            waiting,
            threads,
            timeout,
        }
    }
}

fn spawn_thread<F, T>(
    rx: Arc<Mutex<Receiver<T>>>,
    mut spawner: F,
    waiting: Arc<AtomicUsize>,
    thread: Arc<AtomicUsize>,
    timeout: Option<Duration>,
) where
    F: FnMut(T) + Clone + Send + Sync + 'static,
    T: Send + 'static,
{
    thread::spawn(move || {
        loop {
            waiting.fetch_add(1, SeqCst);
            let Ok(rx) = rx.lock() else { break };
            let task = if let Some(timeout) = timeout {
                rx.recv_timeout(timeout).unwrap()
            } else {
                rx.recv().unwrap()
            };
            waiting.fetch_sub(1, SeqCst);
            spawner(task);
        }
        thread.fetch_sub(1, SeqCst);
    });
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let pool = PoolBuilder::new().start(|foo| println!("{foo}"));
        pool.task("Foo");
    }
}
