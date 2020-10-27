mod init;
mod ball;
mod control;
mod parse_input;
mod main_menu;
mod game;
mod algorithm;

use amethyst::{core::transform::TransformBundle, prelude::*, renderer::{
    plugins::{RenderFlat2D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle,
}, utils::application_root_dir, LoggerConfig, LogLevelFilter, StdoutLog};
use crate::ball::Ball;
use crate::control::BallMouseControl;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::ui::{UiCreator, UiBundle, RenderUi};
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst::assets::HotReloadBundle;
use amethyst::renderer::RenderDebugLines;
use std::collections::HashSet;
use maplit::hashset;
use crate::algorithm::calculate_strategy;


fn main() -> amethyst::Result<()> {
    // println!("{:?}", calculate_strategy(25, &vec![1, 2, 3, 4]));
    // return Ok(());

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
