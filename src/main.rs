use mpris::PlayerFinder;
use anyhow::Result;
use tray_icon::{TrayIconEvent, TrayIconBuilder, Icon};
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    let icon = Icon::from_rgba(vec![0,1,0,0], 1, 1)?;

    gtk::init()?;
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon).build()
        .unwrap();
    gtk::main();

    let mut tracker = players[0].track_progress(500)?;
    let tray_channel = TrayIconEvent::receiver();


    println!("Hello, {}!", players[0].identity());
    loop {
        let tick = tracker.tick();
        let elapsed = tick.progress.position().as_secs();
        let total = tick.progress.length().unwrap().as_secs();
        println!("{elapsed}s/{total}s!");

        for event in tray_channel.try_iter() {
            println!("{:?}: event", event)
        }
    }
}
