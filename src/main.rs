#[macro_use]
extern crate log;

use wlroots::extensions::server_decoration::ServerDecorationMode;
use wlroots::{CompositorBuilder, Seat};

pub mod dock;
pub mod event;
pub mod input_manager;
pub mod output_handler;
pub mod output_manager;
pub mod renderer;
pub mod seat_manager;
pub mod server;
pub mod shell;
pub mod space;
pub mod spring;
pub mod status;
pub mod utils;
pub mod view;

use self::input_manager::InputManager;
use self::output_manager::OutputManager;
use self::seat_manager::SeatManager;
use self::server::Server;
use self::shell::XdgV6ShellManager;

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                time::now().rfc3339(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stderr())
        .apply()
        .expect("Failed to init logger");
    info!("Starting...");

    let mut compositor = CompositorBuilder::new()
        .gles2(true)
        .data_device(true)
        .server_decoration_manager(true)
        .input_manager(Box::new(InputManager::new()))
        .output_manager(Box::new(OutputManager::new()))
        .xdg_shell_v6_manager(Box::new(XdgV6ShellManager::new()))
        .build_auto(Server::new());

    compositor
        .server_decoration_manager
        .as_mut()
        .unwrap()
        .set_default_mode(ServerDecorationMode::Server);

    let seat = Seat::create(
        &mut compositor,
        "seat0".into(),
        Box::new(SeatManager::new()),
    );
    {
        let server: &mut Server = compositor.data.downcast_mut().unwrap();
        server.seat = seat;
    }
    compositor.run();
}
