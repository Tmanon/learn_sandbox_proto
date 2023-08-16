use bevy::{prelude::*, window::WindowMode::*};
use bevy_xpbd_2d::prelude::*;

pub mod actions;
pub mod camera;
pub mod game;
use actions::ActionPlugin;
use camera::CameraPlugin;
use game::GamePlugin;

pub mod body_bundles;
pub mod constants;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            ActionPlugin,
            CameraPlugin,
            GamePlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}