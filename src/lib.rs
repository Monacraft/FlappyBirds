use bevy::prelude::*;
pub mod client;
use client::ClientPlugin;

pub mod game;
use game::*;

pub fn start_client() {
    App::new().add_plugin(ClientPlugin).run();
}

pub fn start_server() {

}
