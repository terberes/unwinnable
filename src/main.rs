mod init;
mod ball;
mod parse_input;
mod main_menu;
mod game;
mod algorithm;
mod pause;

use amethyst::{core::transform::TransformBundle, prelude::*, renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
}, utils::application_root_dir, LoggerConfig, LogLevelFilter, StdoutLog};

use crate::ball::BallMouseControl;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{UiBundle, RenderUi};
use amethyst::renderer::RenderDebugLines;
use specs_physics::{physics_dispatcher, register_physics_systems};
use specs_physics::systems::{SyncBodiesToPhysicsSystem, SyncCollidersToPhysicsSystem, PhysicsStepperSystem, SyncParametersToPhysicsSystem, SyncBodiesFromPhysicsSystem};

pub trait Togglable {
    fn toggle(&mut self);
}

impl Togglable for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Info,
        stdout: StdoutLog::Colored,
        log_file: None,
        ..LoggerConfig::default()
    });

    let mut app_root = application_root_dir()?;

    if app_root.ends_with("debug") { // Костыль
        app_root.pop();
        app_root.pop();
    }

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        // .with_bundle(HotReloadBundle::default())?
        // .with_bundle(FpsCounterBundle::default())?
        .with(BallMouseControl, "ball_mouse_control", &["input_system"]);

    let mut game = Application::new(
        assets_dir, main_menu::MainMenu::default(), game_data)?;
    game.run();

    Ok(())
}

