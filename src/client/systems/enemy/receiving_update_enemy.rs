use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{
    client::{
        components::{enemy_component::Enemy, player_component::Player}, resources::{enemy_resource::EnemyResource, network_resource::NetworkResource},
        systems::{common::remove_the_dead::despawn_the_dead, enemy::move_enemy::move_enemy},
    },
    common::types::protocol::Message,
};

pub fn handle_network_messages(
    network: Res<NetworkResource>,
    mut commands: Commands,
    enemy_query: Query<(&Parent, &Enemy)>,
    enemy_query_2: Query<(&Parent, &Enemy), With<Enemy>>,
    // query_player: Query<(&Parent, &Player), With<Player>>,
    query_player: Single<(Entity, &Player)>,
    mut exit_writer: EventWriter<AppExit>,
    parent_query: Query<&mut Transform>,
    enemy_resource: ResMut<EnemyResource>,
) {
    let mut buf = vec![0; 1024];
    match network.socket.try_recv(&mut buf) {
        Ok(len) => {
            if let Ok(message) = bincode::deserialize(&buf[..len]) {
                match message {
                    Message::Leave => {
                        info!("Un joueur a quitté le serveur");
                    }
                    Message::PlayerUpdateReceiving {
                        name,
                        position,
                        rotation,
                        all_dead_players
                    } => {
                        // println!("All dead player {:?}", all_dead_players.clone());
                        move_enemy(
                            name,
                            position,
                            rotation,
                            enemy_query,
                            parent_query,
                        );
                        let is_new_dead = add_dead_player_if_not_exists(enemy_resource, all_dead_players.clone());
                        if is_new_dead {
                            despawn_the_dead(commands.reborrow(), &all_dead_players, &enemy_query_2, &query_player);
                        }
                    }
                    Message::Win => {
                        println!("Nahhh, I'd Win !!! 😎🔥");
                        exit_writer.send(AppExit::Success);
                    }
                    Message::Lose => {
                        println!("You Loserrrrr ❌");
                        exit_writer.send(AppExit::Success);
                    }
                    _ => todo!(),
                }
            }
        }
        Err(_) => {} // Ignore errors
    }
}

pub fn add_dead_player_if_not_exists(
    mut enemy_resource_dead: ResMut<EnemyResource>,
    dead_players: Vec<String>,
 ) -> bool {
    for dead_player in dead_players {
        if !enemy_resource_dead.dead_players.contains(&dead_player) {
            enemy_resource_dead.dead_players.push(dead_player);
            return true;
        }
    }
    false
}