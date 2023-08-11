use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub enum GameEvent {
    Create {
        capacity: usize,
        package_uuid: Uuid,
        presenter_uuid: Uuid,
    },
    Start,
    Pause,
    Finish,
}

#[derive(Debug)]
pub enum LobbyEvent {
    CreateRoom,
}

#[derive(Debug)]
pub enum BroadcastEvent {
    CreateRoom,
}
