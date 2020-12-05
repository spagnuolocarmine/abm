extern crate abm;
extern crate amethyst;

use crate::environment::Environment;
use crate::main_system_bundle::MainSystemBundle;
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    utils::fps_counter::FpsCounterBundle,
};

mod agent_adapter;
mod environment;
mod main_system_bundle;
mod resources;
mod static_object;
mod systems;

// Only used to calculate the diagonal cutdown. Remove once const fns are powerful enough to handle float arithmetic.
#[macro_use]
extern crate lazy_static;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?
        .join("examples")
        .join("antsforage_amethyst");

    // Window config such as size and window title
    let display_config_path = app_root.join("config").join("display.ron");

    // The folder containing our assets, graphical and ron spritesheet configs
    let assets_dir = app_root.join("assets");

    // The "components", or bundles, of our simulation. Here we're simply saying we
    // want an application with rendering (with a white background), where we are
    // going to render 2D graphics, positioning them through a Transform, and with
    // several bundles: Transform and FpsCounter by Amethyst, and our custom MainSystem one.
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([255.0, 255.0, 255.0, 1.0]), // white opaque
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(MainSystemBundle)?;

    // Run our simulation by setting the initial state to Environment, the one and only state.
    let mut game = Application::new(assets_dir, Environment, game_data)?;
    game.run();

    Ok(())
}