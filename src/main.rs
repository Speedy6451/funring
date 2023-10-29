use mpris::PlayerFinder;
use anyhow::Result;
use tray_item::IconSource;
use tray_item::TrayItem;
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    let mut tracker = players[0].track_progress(1000/20)?;

    let icon = IconSource::Data{data: gen_icon(0), height: RES, width: RES};

    let mut tray = TrayItem::new("funring", icon)?;

    println!("Hello, {}!", players[0].identity());
    loop {
        let tick = tracker.tick();
        let elapsed = tick.progress.position().as_secs();
        let total = tick.progress.length().unwrap().as_secs();
        println!("{elapsed}s/{total}s!");
        let prog = elapsed as f32 / total as f32 * 256.0;
        println!("{prog}");

        tray.set_icon(
            IconSource::Data{data: gen_icon(prog as u8), height: RES, width: RES}
        )?;
    }
}

const RES: i32 = 64;

fn gen_icon(red: u8) -> Vec<u8> {
    let mut icon = Vec::new();
    for pix in 0..(RES*RES) {
        icon.push(1);
        icon.push(
            if red as f32 > (pix as f32 / (RES * RES) as f32 * 256.0) {
                red
            } else {
                0
            });
        icon.push(0);
        icon.push(0);
    }
    icon
}
