use mpris::PlayerFinder;
use anyhow::Result;
use tray_icon::menu::Menu;
use tray_icon::{TrayIconEvent, TrayIconBuilder, Icon};
use winit::event_loop::EventLoopBuilder;
use winit::event_loop::ControlFlow;
fn main() -> Result<()> {
    let pf = PlayerFinder::new()?;
    let players = pf.find_all()?;

    let icon = Icon::from_rgba(vec![0,1,0,0], 1, 1)?;

    // gtk thread
    std::thread::spawn(|| {
        gtk::init().unwrap();
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_icon(icon).build()
            .unwrap();
        gtk::main();
    });

    let mut tracker = players[0].track_progress(500)?;
    let tray_channel = TrayIconEvent::receiver();

    let event_loop = EventLoopBuilder::new().build()?;

    println!("Hello, {}!", players[0].identity());
    event_loop.run(move |event, event_loop| {
        event_loop.set_control_flow(ControlFlow::Poll);

        let player = &players[0];
        let elapsed = player.get_position().unwrap().as_secs();
        let total = player.get_metadata().unwrap().length().unwrap().as_secs();
        println!("{elapsed}s/{total}s!");

        for event in tray_channel.try_iter() {
            println!("{:?}: event", event)
        }
    })?;
    Ok(())
}
