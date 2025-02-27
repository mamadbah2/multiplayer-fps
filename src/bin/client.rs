use bevy::prelude::*;
use multiplayer_fps::{
    client::{
        plugins::{
            enemy_plugin::EnemyPlugin, player_plugin::PlayerPlugin, world_plugin::WorldPlugin,
        },
        resources::{
            enemy_resource::EnemyResource,
            network_resource::{input_connexion, NetworkResource},
            player_resource::PlayerResource,
        },
        systems::{
            common::{fps_display_system::setup_fps_counter, window_config_system::config_window},
            enemy::receiving_update_enemy::handle_network_messages,
        },
        udp::Client,
    },
    common::types::protocol::{CommonPlayer, Message},
};
// use rapier3d::crossbeam::epoch::Pointable;
use tokio::runtime::Runtime;
fn main() {
    // Créer le runtime une seule fois
    let runtime = Runtime::new().unwrap();

    let (name, server_address) = input_connexion();

    // Établir la connexion et obtenir le socket
    let socket = runtime.block_on(async {
        let client = Client::new(name);
        match client.connect(server_address).await {
            Ok(ok) => ok,
            Err(e) => {
                println!("error: {:?}", e);
                panic!()
            }
        }
    });

    let mut playr = CommonPlayer::new();
    let mut enemis = Vec::new();

    let mut is_game_started = false;
    println!("Wait for the number of player to be completed !");
    while !is_game_started {
        let mut buf = vec![0; 1024];
        match socket.try_recv(&mut buf) {
            Ok(len) => {
                if let Ok(message) = bincode::deserialize(&buf[..len]) {
                    match message {
                        Message::StartGame { player, enemies } => {
                            playr = player;
                            enemis = enemies;
                            is_game_started = true;
                        }
                        _ => {}
                    }
                }
            }
            Err(_) => {}
        };
    }
    // println!("enemis {:?}", enemis);
    // println!("playr {:?}", playr);

    App::new()
        .add_plugins((DefaultPlugins, WorldPlugin, PlayerPlugin, EnemyPlugin))
        // .add_systems(Msaa::default())
        .insert_resource(NetworkResource::new(socket))
        .insert_resource(EnemyResource::new(enemis))
        .insert_resource(PlayerResource::new(playr))
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Update, (handle_network_messages, config_window))
        .run();
}
