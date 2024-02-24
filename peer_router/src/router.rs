use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

pub struct Router {
    table: HashMap<u64, Vec<RoutePath>>,
    stale_time: Duration,
}

struct RoutePath {
    via: u64,
    hops: u8,
    age: Instant,
}

pub struct Route {
    pub to: u64,
    pub via: u64,
    pub hops: u8,
}

impl PartialEq for RoutePath {
    fn eq(&self, other: &Self) -> bool {
        self.hops == other.hops
    }
}

impl Eq for RoutePath {}

impl PartialOrd for RoutePath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RoutePath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hops.cmp(&other.hops)
    }
}

impl Router {
    pub fn new(stale_time: Duration) -> Router {
        Router {
            table: HashMap::new(),
            stale_time,
        }
    }
    pub fn add_routes(&mut self, updates: &[Route]) {
        let now = Instant::now();
        for update in updates {
            let paths = self.table.entry(update.to).or_default();
            if let Some(path) = paths.iter_mut().find(|path| path.via == update.via) {
                if path.hops > update.hops {
                    path.hops = update.hops;
                }
                path.age = now;
            } else {
                paths.push(RoutePath {
                    age: now,
                    hops: update.hops,
                    via: update.via,
                })
            }
            paths.sort();
        }
    }
    pub fn purge_routes(&mut self) {
        let now = Instant::now();
        self.table
            .iter_mut()
            .for_each(|(_, val)| val.retain(|item| (now - item.age) < self.stale_time))
    }

    pub fn get_routes(&self) -> Vec<Route> {
        self.table
            .iter()
            .filter_map(|(to, paths)| {
                paths.first().map(|first| Route {
                    to: *to,
                    via: first.via,
                    hops: first.hops,
                })
            })
            .collect()
    }
    pub fn get_route(&self, to: u64) -> Option<Route> {
        self.table
            .get(&to)
            .and_then(|routes| routes.first())
            .map(|route| Route {
                to,
                via: route.via,
                hops: route.hops,
            })
    }
}

#[cfg(test)]
mod test {
    use super::{Route, RoutePath, Router};
    use rand::Rng;
    use std::fmt::Debug;
    use std::time::{Duration, Instant};

    impl Debug for Router {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let now = Instant::now();
            let width = f.width().unwrap_or(10);
            let precision = f.precision().unwrap_or(10);
            let dashes = String::from_utf8(vec![b'-'; width]).unwrap();
            writeln!(
                f,
                "|{:<width$.precision$}|{:<width$.precision$}|{:<width$.precision$}|{:<width$.precision$}|",
                "TO", "VIA", "HOPS", "TIME"
            )?;
            writeln!(f, "-{dashes}-{dashes}-{dashes}-{dashes}-")?;
            for (node, paths) in self.table.iter() {
                for path in paths {
                    let RoutePath { age, hops, via } = path;
                    let time = (now - *age).as_secs();
                    writeln!(f, "|{node:<width$.precision$}|{via:<width$.precision$}|{hops:<width$.precision$}|{time:<width$.precision$}|")?;
                }
            }
            Ok(())
        }
    }

    impl Debug for Route {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})--[{}]-->({})", self.via, self.hops, self.to)
        }
    }

    #[test]
    fn test() {
        let mut router = Router::new(Duration::from_secs(20));

        let mut rng = rand::thread_rng();
        loop {
            let to = rng.gen_range(0u64..10);
            let via = rng.gen_range(0u64..10);
            let hops = rng.gen_range(1u8..10);
            router.add_routes(&[Route { to, via, hops }]);
            println!("{router:?}");
            let best_routes = router.get_routes();
            println!("{best_routes:#?}");
            router.purge_routes();
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
