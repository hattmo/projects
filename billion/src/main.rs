#![feature(slice_split_once)]

use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, Read},
    sync::{
        mpsc::{self, Receiver, SyncSender},
        Mutex, RwLock,
    },
};

type City = [u8; 32];

struct CityData {
    min: i32,
    max: i32,
    total: i32,
    count: i32,
}

const NUM_THREADS: usize = 8;
const BUFFER_SIZE: usize = 1024 * 1024 * 512;

struct Buffer {
    len: usize,
    data: [u8; BUFFER_SIZE],
}

fn main() {
    let data_store: RwLock<HashMap<City, Mutex<CityData>>> = RwLock::new(HashMap::new());

    let (to_consumer, from_producer) = mpsc::sync_channel(NUM_THREADS * 2);
    let (to_producer, from_consumer) = mpsc::sync_channel(NUM_THREADS * 2);
    let from_producer = Mutex::new(from_producer);

    for _ in 0..(NUM_THREADS * 2) {
        to_producer
            .send(Box::new(Buffer {
                data: [0; BUFFER_SIZE],
                len: 0,
            }))
            .unwrap();
    }

    std::thread::scope(|scope| {
        for _ in 0..NUM_THREADS {
            scope.spawn(|| {
                let _ = consumer(&from_producer, &to_producer, &data_store);
            });
        }
        let _ = producer(from_consumer, to_consumer);
    });
}

struct ThreadDone;

fn producer(rx: Receiver<Box<Buffer>>, tx: SyncSender<Box<Buffer>>) -> Result<(), ThreadDone> {
    let mut data_file = OpenOptions::new()
        .read(true)
        .open("./data.txt")
        .or(Err(ThreadDone))?;
    let mut left_over = [0u8; 32];
    let mut left_over_len = 0;
    loop {
        let mut buffer = rx.recv().or(Err(ThreadDone))?;
        buffer.data[..left_over_len].copy_from_slice(&left_over[..left_over_len]);
        let num_read = data_file
            .read(&mut buffer.data[left_over_len..])
            .or(Err(ThreadDone))?;
        if num_read == 0 {
            break;
        }
        let buf = &buffer.data[..(left_over_len + num_read)];
        let (full_data, extra) = buf.rsplit_once(|c| *c == b'\n').ok_or(ThreadDone)?;
        buffer.len = full_data.len();
        left_over[..extra.len()].copy_from_slice(extra);
        left_over_len = extra.len();
        tx.try_send(buffer).or(Err(ThreadDone))?;
    }
    Ok(())
}

fn consumer(
    rx: &Mutex<Receiver<Box<Buffer>>>,
    tx: &SyncSender<Box<Buffer>>,
    data_store: &RwLock<HashMap<City, Mutex<CityData>>>,
) -> Result<(), ThreadDone> {
    let mut temp_city = [0u8; 32];
    loop {
        let buffer = rx.lock().unwrap().recv().or(Err(ThreadDone))?;
        let buf = &buffer.data[..buffer.len];
        for line in buf.lines() {
            temp_city.fill(0);
            let (city, temp) = line.split_once(';').ok_or(ThreadDone)?;
            temp_city[..city.len()].copy_from_slice(city.as_bytes());
        }
        tx.send(buffer).or(Err(ThreadDone))?;
    }
}
