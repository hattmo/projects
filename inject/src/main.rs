use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() {
    let pid: Box<[_]> = std::env::args().collect();
    let [_, pid] = pid.as_ref() else {
        return Err(io::Error::other("Please provide a pid"));
    };
}

struct MemMaps {
    maps: HashMap<String, (u64, u64)>,
}
impl MemMaps {
    fn new(pid: u64) -> io::Result<()> {
        let f = BufReader::new(File::open(format!("/proc/{}/maps", pid))?);
        let mut maps: HashMap<_, _> = f
            .lines()
            .flatten()
            .filter_map(|l| {
                let parts: Box<[_]> = l.split_whitespace().collect();
                if parts.len() == 6 {
                    let (lower, upper) = parts[0].split_once("-")?;
                    return Some((
                        parts[5].to_owned(),
                        u64::from_str_radix(lower, 16).ok()?,
                        u64::from_str_radix(upper, 16).ok()?,
                    ));
                }
                None
            })
            .fold(HashMap::new(), |mut acc, (f, l, u)| {
                let e = acc.entry(f).or_insert((l, u));
                if e.0 > l {
                    e.0 = l
                }
                if e.1 < u {
                    e.1 = u
                }
                acc
            });
        let mut list: Box<[_]> = maps.into_iter().map(|(f, (l, u))| (f, l, u)).collect();
        list.sort_by_key(|(_, l, _)| *l);
        for (f, l, u) in list {
            println!("0x{l:X} - 0x{u:X}: {f}");
        }
        Ok(())
    }
}
