use bevy::prelude::*;
use multiplayer_fps::client::{
    plugins::player::PlayerPlugin, resources::network::{handle_network_messages, input_connexion, NetworkResource}, udp::Client
};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn main() {
    // Créer le runtime une seule fois
    let runtime = Runtime::new().unwrap();

    let (name, server_address) = input_connexion();

    // Établir la connexion et obtenir le socket
    let socket = runtime.block_on(async {
        let client = Client::new(name);
        match client.connect(server_address).await {
            Ok(so) => so,
            Err(e) =>  panic!("Error: {}", e)
        }
    });

    // Une fois connecté, démarrer Bevy
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(NetworkResource {
            socket: Arc::new(socket),
        })
        .add_systems(Update, handle_network_messages)
        .run();
}
