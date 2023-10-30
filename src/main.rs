#![feature(test)]

use std::collections::hash_map::DefaultHasher;
use std::env::args_os;
use std::hash::Hash;
use std::hash::Hasher;

use anyhow::Result;
use glam::UVec2;
use glam::Vec2;
use mpris::PlaybackStatus;
use mpris::PlayerFinder;
use tray_item::IconSource;
use tray_item::TrayItem;
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    for player in players.iter().map(|p| p.identity()) {
        println!("found {player}");
    }

    let player = if let Some(name) = args_os().nth(1) {
        pf.find_by_name(&name.into_string().unwrap())?
    } else {
        pf.find_active()?
    };

    let mut tracker = player.track_progress(25)?;

    let icon = IconSource::Data {
        data: gen_icon(0.0, false),
        height: RES,
        width: RES,
    };

    let mut tray = TrayItem::new("funring", icon)?;

    let mut last_hash = None;

    println!("tracking {}", player.identity());
    loop {
        let tick = tracker.tick();
        let elapsed = tick.progress.position().as_millis() as f64;
        let total = tick.progress.length();
        if total.is_none() {continue};
        let total = total.unwrap().as_millis();
        let prog = elapsed as f32 / total as f32;
        let playing = tick.progress.playback_status() == PlaybackStatus::Playing;

        let buffer = gen_icon(prog, playing);
        let hash = {
            let mut hasher = DefaultHasher::new();
            buffer.hash(&mut hasher);
            hasher.finish()
        };
        if !last_hash.replace(hash).is_some_and(|h| h == hash) {
            tray.set_icon(IconSource::Data {
                data: buffer,
                height: RES,
                width: RES,
            })?;
        }
    }
}

const RES: i32 = 32;
const LEN: usize = (RES * RES) as usize;

fn gen_icon(prog: f32, playing: bool) -> Vec<u8> {
    let mut icon = vec![0; LEN * 4];
    for pix in 0..(LEN) {
        let x = pix % RES as usize;
        let y = pix / RES as usize;
        icon_cs(
            UVec2::new(x as u32, y as u32),
            prog,
            playing,
            &mut icon[0..LEN * 4],
        );
    }
    icon
}

// TODO: integrate rust-gpu
fn icon_cs(idx: UVec2, progress: f32, playing: bool, buf: &mut [u8]) {
    let index = (idx.y * RES as u32 + idx.x) as usize;
    let pix = &mut buf[4 * index..4 * index + 4];
    pix[0] = 0;

    let center = Vec2::splat(RES as f32 / 2.0);
    let idx = idx.as_vec2() - center;
    let radius = idx.length() / RES as f32 * 2.0;
    let theta = std::f32::consts::PI - f32::atan2(idx.x, idx.y);

    let offset = theta - progress * std::f32::consts::TAU;

    if offset < 0.0 && (0.5..1.0).contains(&radius) {
        pix[1] = if playing { 255 } else { 0 };
        //pix[1] = if playing { 255 - (offset * -55.0) as u8 } else { 0 };
        pix[2] = if playing { 0 } else { 95 };
        pix[3] = 0;
    } else if (0.5..1.0).contains(&radius) {
        pix[1] = if playing { 6 } else { 20 };
        pix[2] = 0;
        pix[3] = 0;
    } else {
        pix[1] = 0;
        pix[2] = 0;
        pix[3] = 0;
    }
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use crate::gen_icon;

    #[bench]
    fn generate_icon(b: &mut Bencher) {
        b.iter(|| gen_icon(0.5, true));
    }
}
