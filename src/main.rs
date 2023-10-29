use mpris::PlaybackStatus;
use mpris::PlayerFinder;
use anyhow::Result;
use tray_item::IconSource;
use tray_item::TrayItem;
use glam::UVec2;
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    let mut tracker = players[0].track_progress(400)?;

    let icon = IconSource::Data{data: gen_icon(0.0, false), height: RES, width: RES};

    let mut tray = TrayItem::new("funring", icon)?;

    println!("Hello, {}!", players[0].identity());
    loop {
        let tick = tracker.tick();
        let elapsed = tick.progress.position().as_millis() as f64
            + tick.progress.age().as_millis() as f64 
            * tick.progress.playback_rate();
        let total = tick.progress.length().unwrap().as_millis();
        let prog = elapsed as f32 / total as f32;
        let playing = tick.progress.playback_status() == PlaybackStatus::Playing;

        tray.set_icon(
            IconSource::Data{data: gen_icon(prog, playing), height: RES, width: RES}
        )?;
    }
}

const RES: i32 = 8;
const LEN: usize = (RES*RES) as usize;

fn gen_icon(
    prog: f32,
    playing: bool,
) -> Vec<u8> {
    let mut icon = Vec::with_capacity(LEN*4);
    (0..LEN*4).for_each(|_| icon.push(0));
    for pix in 0..(LEN) {
        let x = pix % RES as usize;
        let y = pix / RES as usize;
        icon_cs(
            UVec2::new(x as u32,y as u32),
            prog,
            playing,
            &mut icon[0..LEN*4]
        );
    }
    icon
}

// TODO: integrate rust-gpu
fn icon_cs(
    pix: UVec2,
    progress: f32,
    playing: bool,
    buf: &mut [u8],
) {
    let idx = (pix.y * RES as u32  + pix.x) as usize;
    let pix = &mut buf[4*idx..4*idx+4];
    pix[0] = 1;

    if progress > idx as f32 / LEN as f32 {
        pix[1] = 255;
        pix[2] = 0;
        pix[3] = 0;
    } else {
        pix[1] = 0;
        pix[2] = 0;
        pix[3] = 0;
    }
}
