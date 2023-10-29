use std::io::Cursor;

use mpris::PlayerFinder;
use anyhow::Result;
use tray_item::IconSource;
use tray_item::TrayItem;
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    let mut tracker = players[0].track_progress(500)?;

    let icon = IconSource::Data{data: gen_icon(), height: 32, width: 32};

    let mut tray = TrayItem::new("funring", icon);

    println!("Hello, {}!", players[0].identity());
    loop {
        let player = &players[0];
        let elapsed = player.get_position().unwrap().as_secs();
        let total = player.get_metadata().unwrap().length().unwrap().as_secs();
        println!("{elapsed}s/{total}s!");
    }
}

fn gen_icon() -> Vec<u8> {
    let mut icon = Vec::new();
    for pix in 0..(32*32) {
        icon.push(0);
        icon.push(0);
        icon.push(255);
        icon.push(1);
    }
    icon
}
