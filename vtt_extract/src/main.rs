#![feature(iter_intersperse, try_blocks)]

use std::collections::VecDeque;
use std::io::{stderr, BufRead, Write};

struct Section {
    from: u64,
    text: String,
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let times = match args
        .map(|arg| time_as_sec(&arg))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(times) => times,
        Err(e) => {
            let _ = writeln!(stderr(), "Failed to parse arg: {e}");
            return;
        }
    };
    let mut times = times.into_iter();
    let mut current_time = times.next();

    let std_in = std::io::stdin().lock();
    let mut std_out = std::io::stdout().lock();

    let mut sections = std_in
        .lines()
        .into_iter()
        .filter_map(|line| line.ok())
        .split(|line| line.is_empty());
    sections.next();
    sections
        .filter_map(|section| {
            let mut section: VecDeque<_> = section.into();
            section.pop_front()?;
            let time = section.pop_front()?;
            let (from, _) = time.split_once("-->")?;
            let from = time_as_sec(from.trim()).ok()?;
            Some(Section {
                from,
                text: section.into_iter().intersperse(" ".to_string()).collect(),
            })
        })
        .map(|section| {
            if current_time.is_some_and(|time| time <= section.from) {
                current_time = times.next();
                return format!("\n\n---\n\n{} ", section.text);
            } else {
                format!("{} ", section.text)
            }
        })
        .for_each(|section| {
            if let Err(_) = write!(std_out, "{section}") {
                let _ = write!(stderr(), "Error writing to stdout");
                std::process::exit(0);
            };
        });
    let _ = write!(stderr(), "Done");
}

fn time_as_sec(time: &str) -> Result<u64, String> {
    let mut parts = time.split(":");
    let res: Option<u64> = try {
        let hrs: u64 = parts.next()?.parse().ok()?;
        let mins: u64 = parts.next()?.parse().ok()?;
        let secs = parts.next()?.split(".").next()?;
        let secs: u64 = secs.parse().ok()?;
        secs + (60 * mins) + (60 * 60 * hrs)
    };
    res.ok_or(time.to_string())
}

use std::ops::FnMut;

struct SplitIter<I, T, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    inner: I,
    pred: F,
    done: bool,
}

trait Split<I, T, F>
where
    F: FnMut(&T) -> bool,
    I: Iterator<Item = T>,
{
    fn split(self, pred: F) -> SplitIter<I, T, F>;
}

impl<I, T, F> Split<I, T, F> for I
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    fn split(self, pred: F) -> SplitIter<I, T, F> {
        SplitIter {
            inner: self,
            pred,
            done: false,
        }
    }
}

impl<I, T, F> Iterator for SplitIter<I, T, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T) -> bool,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::new();
        while let Some(item) = self.inner.next() {
            if (self.pred)(&item) {
                return Some(chunk);
            } else {
                chunk.push(item);
            }
        }
        if self.done {
            None
        } else {
            self.done = true;
            Some(chunk)
        }
    }
}
