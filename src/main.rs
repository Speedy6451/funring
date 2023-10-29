use mpris::PlaybackStatus;
use mpris::PlayerFinder;
use anyhow::Result;
use tray_item::IconSource;
use tray_item::TrayItem;
use glam::UVec2;
use glam::Vec2;
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    let mut tracker = players[0].track_progress(400)?;

    let icon = IconSource::Data{data: gen_icon(0.0, false), height: RES, width: RES};

    let mut tray = TrayItem::new("funring", icon)?;

    println!("Hello, {}!", players[0].identity());
    loop {
        let tick = tracker.tick();
        let elapsed = tick.progress.position().as_millis() as f64;
        let total = tick.progress.length().unwrap().as_millis();
        let prog = elapsed as f32 / total as f32;
        let playing = tick.progress.playback_status() == PlaybackStatus::Playing;

        tray.set_icon(
            IconSource::Data{data: gen_icon(prog, playing), height: RES, width: RES}
        )?;
    }
}

const RES: i32 = 64;
const LEN: usize = (RES*RES) as usize;

fn gen_icon(
    prog: f32,
    playing: bool,
) -> Vec<u8> {
    let mut icon = vec![0; LEN*4];
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
    idx: UVec2,
    progress: f32,
    playing: bool,
    buf: &mut [u8],
) {
    let index = (idx.y * RES as u32  + idx.x) as usize;
    let pix = &mut buf[4*index..4*index+4];
    pix[0] = 1;

    let center = Vec2::splat(RES as f32 /2.0);
    let idx = idx.as_vec2()-center;
    let radius = idx.length() / RES as f32 * 2.0;
    let theta = std::f32::consts::PI - f32::atan2(idx.x, idx.y);

    let offset = theta - progress * std::f32::consts::TAU;

    if offset < 0.0 && 0.6 < radius && radius < 0.8 {
        pix[1] = if playing {255} else {0};
        pix[2] = if playing {0} else {95};
        pix[3] = 0;
    } else if 1.0 > radius {
        pix[1] = if playing {60} else {20};
        pix[2] = 0;
        pix[3] = 0;
    } else {
        pix[1] = 0;
        pix[2] = 0;
        pix[3] = 0;
    }
}
