#![warn(missing_docs)]
#![deny(clippy::disallowed_types)]

//! The common client&server code for OpenCubeGame

pub mod voxel;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use bevy::app::AppExit;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::TimePlugin;
use bevy::utils::synccell::SyncCell;
use ocg_schemas::OcgExtraData;

/// Target (maximum) number of game simulation ticks in a second.
pub const TICKS_PER_SECOND: i32 = 32;
/// Target (maximum) number of game simulation ticks in a second, as a `f32`.
pub const TICKS_PER_SECOND_F32: f32 = TICKS_PER_SECOND as f32;
/// Target (maximum) number of game simulation ticks in a second, as a `f64`.
pub const TICKS_PER_SECOND_F64: f64 = TICKS_PER_SECOND as f64;
/// Target (minimum) number of seconds in a game simulation tick, as a `f32`.
pub const SECONDS_PER_TICK_F32: f32 = 1.0f32 / TICKS_PER_SECOND as f32;
/// Target (minimum) number of seconds in a game simulation tick, as a `f64`.
pub const SECONDS_PER_TICK_F64: f64 = 1.0f64 / TICKS_PER_SECOND as f64;
/// Target (minimum) number of microseconds in a game simulation tick, as a `i64`.
pub const MICROSECONDS_PER_TICK: i64 = 1_000_000i64 / TICKS_PER_SECOND as i64;
/// One game tick as a [`Duration`]
pub const TICK: Duration = Duration::from_micros(MICROSECONDS_PER_TICK as u64);

// Ensure `MICROSECONDS_PER_TICK` is perfectly accurate.
static_assertions::const_assert_eq!(1_000_000i64 / MICROSECONDS_PER_TICK, TICKS_PER_SECOND as i64);

/// An [`OcgExtraData`] implementation containing server-side data for the game engine.
pub struct ServerData;

impl OcgExtraData for ServerData {
    type ChunkData = ();
    type GroupData = ();
}

/// Control commands for the server, for in-process communication.
pub enum GameServerControlCommand {
    /// Gracefully shuts down the server.
    Shutdown,
}

/// A struct to communicate with the "server"-side engine that runs the game simulation.
/// It has its own bevy App with a very limited set of plugins enabled to be able to run without a graphical user interface.
pub struct GameServer {
    thread: JoinHandle<()>,
    pause: AtomicBool,
}

/// A handle to a [`GameServer`] and its in-process control channel.
pub struct GameServerHandle {
    /// The spawned [`GameServer`] instance.
    pub server: Arc<GameServer>,
    /// The channel for sending [`GameServerControlCommand`] such as "Shutdown".
    pub control_channel: Sender<GameServerControlCommand>,
}

#[derive(Resource)]
struct GameServerControlCommandReceiver(SyncCell<Receiver<GameServerControlCommand>>);

impl GameServer {
    /// Spawns a new thread that runs the engine in a paused state, and returns a handle to control it.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GameServerHandle {
        let (tx, rx) = sync_channel(1);
        let (ctrl_tx, ctrl_rx) = channel();
        let thread = std::thread::Builder::new()
            .name("OCG Engine Thread".to_owned())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || GameServer::thread_main(rx, ctrl_rx))
            .expect("Could not create a thread for the engine");
        let server = Self {
            thread,
            pause: AtomicBool::new(true),
        };
        let server = Arc::new(server);
        tx.send(Arc::clone(&server))
            .expect("Could not pass initialization data to the engine thread");
        GameServerHandle {
            server,
            control_channel: ctrl_tx,
        }
    }

    /// Checks if the game logic is paused.
    pub fn is_paused(&self) -> bool {
        self.pause.load(SeqCst)
    }

    /// Sets the paused state for game logic, returns the previous state.
    pub fn set_paused(&mut self, paused: bool) -> bool {
        self.pause.swap(paused, SeqCst)
    }

    /// Checks if the engine thread is still alive.
    pub fn is_alive(&self) -> bool {
        !self.thread.is_finished()
    }

    fn thread_main(engine: Receiver<Arc<GameServer>>, ctrl_rx: Receiver<GameServerControlCommand>) {
        let _engine = {
            let e = engine
                .recv()
                .expect("Could not receive initialization data in the engine thread");
            drop(engine); // force-drop the receiver early to not hold onto its memory
            e
        };
        let mut app = App::new();
        app.add_plugins(LogPlugin::default())
            .add_plugins(TaskPoolPlugin::default())
            .add_plugins(TypeRegistrationPlugin)
            .add_plugins(FrameCountPlugin)
            .add_plugins(TimePlugin)
            .add_plugins(TransformPlugin)
            .add_plugins(HierarchyPlugin)
            .add_plugins(DiagnosticsPlugin)
            .add_plugins(AssetPlugin::default())
            .add_plugins(AnimationPlugin);
        app.insert_resource(FixedTime::new(TICK));
        app.insert_resource(GameServerControlCommandReceiver(SyncCell::new(ctrl_rx)));
        app.add_systems(PostUpdate, Self::control_command_handler_system);
        app.run();
    }

    fn control_command_handler_system(
        ctrl_rx: ResMut<GameServerControlCommandReceiver>,
        mut exiter: EventWriter<AppExit>,
    ) {
        let ctrl_rx = ctrl_rx.into_inner().0.get();
        for cmd in ctrl_rx.try_iter() {
            match cmd {
                GameServerControlCommand::Shutdown => {
                    exiter.send(AppExit);
                }
            }
        }
    }
}
