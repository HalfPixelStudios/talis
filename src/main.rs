use bevy::prelude::*;
use GMTK2022::{
    assetloader::*,
    game::{GamePlugin, NextTurnEvent, StartLevelEvent, StartRoundEvent},
    prefab::PrefabPlugin,
    troop::TroopPlugin,
};

fn main() {
    let window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "bevy_test".into(),
        ..default()
    };

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(window_descriptor);
        // .add_system(bevy::input::system::exit_on_esc_system)

    app.add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadPlugin)
        .add_startup_system(setup);

    app.add_plugin(GamePlugin)
        .add_plugin(PrefabPlugin)
        .add_plugin(TroopPlugin);

    app.add_system(debug);

    app.run();
}

fn setup(mut commands: Commands, sheets: Res<AssetSheets>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn debug(
    keys: Res<Input<KeyCode>>,
    mut start_round_writer: EventWriter<StartLevelEvent>,
    mut next_turn_writer: EventWriter<NextTurnEvent>,
) {
    if keys.just_pressed(KeyCode::S) {
        info!("pressed: starting round");
        start_round_writer.send(StartLevelEvent { level: 0 });
    }
    if keys.just_pressed(KeyCode::T) {
        info!("pressed: next turn");
        next_turn_writer.send(NextTurnEvent);
    }
}
