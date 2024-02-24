use std::io::Read;
use std::io::Write;

use peer_router::Engine;
use peer_router::EngineConfig;
use peer_router::JobError;
fn main() {
    let mut engine = Engine::new(EngineConfig {
        node_id: 12345,
        stale_time: std::time::Duration::from_secs(5 * 60),
    });
    let mut job = engine.create_job();
    let thread = std::thread::spawn(move || -> Result<(), JobError> {
        let mut new_job = job.create_job()?;
        let stream = new_job.accept()?;

        Ok(())
    });
    let err = engine.start().err().unwrap();
}
