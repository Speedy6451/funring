use mpris::PlayerFinder;
use anyhow::Result;
use tray_item::IconSource;
use tray_item::TrayItem;
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    let mut tracker = players[0].track_progress(1000/20)?;

    let icon = IconSource::Data{data: gen_icon(0.0), height: RES, width: RES};

    let mut tray = TrayItem::new("funring", icon)?;

    println!("Hello, {}!", players[0].identity());
    loop {
        let tick = tracker.tick();
        let elapsed = tick.progress.position().as_millis() as f64
            + tick.progress.age().as_millis() as f64 
            * tick.progress.playback_rate();
        let total = tick.progress.length().unwrap().as_millis();
        println!("{elapsed}s/{total}s!");
        let prog = elapsed as f32 / total as f32;
        println!("{prog}");

        tray.set_icon(
            IconSource::Data{data: gen_icon(prog), height: RES, width: RES}
        )?;
    }
}

const RES: i32 = 32;
const LEN: usize = (RES*RES) as usize;

fn gen_icon(prog: f32) -> Vec<u8> {
    let mut icon = Vec::with_capacity(LEN);
    for pix in 0..(LEN) {
        let x = pix % RES as usize;
        let y = pix / RES as usize;
        icon.push(1);
        icon.push(
            if prog > x as f32 / RES as f32 {
                255
            } else {
                127
            });
        icon.push(0);
        icon.push(0);
    }
    icon
}
