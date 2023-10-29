use mpris::PlayerFinder;
use anyhow::Result;
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    let mut tracker = players[0].track_progress(500)?;

    println!("Hello, {}!", players[0].identity());
    loop {
        let tick = tracker.tick();
        let elapsed = tick.progress.position().as_secs();
        let total = tick.progress.length().unwrap().as_secs();
        println!("{elapsed}s/{total}s!");
    }
}
