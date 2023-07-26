use std::{collections::VecDeque, sync::Arc};

use tokio::sync::{broadcast, Mutex};

pub struct Queue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    notifier: broadcast::Sender<()>,
}

pub struct Producer<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    notifier: broadcast::Sender<()>,
}

pub struct Consumer<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    notifier_receiver: broadcast::Receiver<()>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1);
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            notifier: tx,
        }
    }
    pub fn get_producer(&self) -> Producer<T> {
        Producer {
            queue: self.queue.clone(),
            notifier: self.notifier.clone(),
        }
    }
    pub fn get_consumer(&self) -> Consumer<T> {
        Consumer {
            queue: self.queue.clone(),
            notifier_receiver: self.notifier.subscribe(),
        }
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Producer<T> {
    pub async fn push(&mut self, data: T) {
        self.queue.lock().await.push_back(data);
        let _ = self.notifier.send(());
    }
}

impl<T> Consumer<T> {
    pub async fn pop(&mut self) -> Result<T, ()> {
        loop {
            let task = self.queue.lock().await.pop_front();
            match task {
                Some(val) => return Ok(val),
                None => self.notifier_receiver.recv().await.or(Err(()))?,
            }
        }
    }
}
