use std::{
    any::Any,
    collections::{HashMap, HashSet},
    io::Error,
    net::SocketAddr,
    sync::Arc,
};

use crate::domain::entities::game;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, Path, State, WebSocketUpgrade,
    },
    headers,
    response::Response,
    routing::get,
    Extension, Json, Router, TypedHeader,
};
use chrono::Utc;
use domain::entities::{
    board::Board, events::GameEvent, game::Game, package::Package, package::Question,
    package::QuestionType, package::Round, package::RoundType, package::Topic, user::User,
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use tokio::sync::{broadcast, Mutex, MutexGuard};
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

mod domain;
mod presentation;

#[tokio::main]
async fn main() {
    presentation::web_api::server::run().await;
}


// // async fn ws_handler(
// //     ws: WebSocketUpgrade,
// //     State(state): State<Arc<WsState>>,
// //     Path(game_uuid): Path<Uuid>,
// // ) -> Response {
// //     debug!("New websocket connection. Game UUID: {}", game_uuid);

// //     ws.on_upgrade(move |socket| handle_socket(socket, state, game_uuid))
// // }

// // async fn handle_socket(socket: WebSocket, state: Arc<WsState>, game_uuid: Uuid) {
// //     let (sender, mut receiver) = socket.split();

// //     {
// //         let mut games = state.games.lock().await;

// //         let game_state = games.entry(game_uuid).or_insert(GameState {
// //             game: None,
// //             txs: Vec::new(),
// //         });

// //         game_state.add_session(sender).await;
// //     }

// //     while let Some(msg) = receiver.next().await {
// //         let event: GameEvent = serde_json::from_str(msg.unwrap().to_text().unwrap()).unwrap();

// //         debug!("{:?}", event);

// //         let result_msg = match event {
// //             GameEvent::Create {
// //                 capacity,
// //                 package_uuid,
// //                 presenter_uuid,
// //             } => create_game_event(&state, game_uuid, package_uuid, presenter_uuid, capacity).await,
// //             GameEvent::Start => Ok(Message::Text(String::from("{}"))),
// //             _ => Ok(Message::Text(String::from("test"))),
// //         };

// //         {
// //             let mut games = state.games.lock().await;
// //             let game_state = games.get_mut(&game_uuid).unwrap();

// //             game_state.send_msg(result_msg.unwrap()).await;
// //         }
// //     }
// // }

// // async fn create_game_event(
// //     state: &Arc<WsState>,
// //     game_uuid: Uuid,
// //     package_uuid: Uuid,
// //     presenter_uuid: Uuid,
// //     capacity: usize,
// // ) -> Result<Message, &'static str> {
// //     let mut games = state.games.lock().await;

// //     if games.get(&game_uuid).is_some() {
// //         debug!("{:?}", games.get(&game_uuid));
// //         return Err("Game already exists");
// //     }

// //     games.entry(game_uuid).and_modify(|game_state| {
// //         game_state.game = Some(Game::new(
// //             capacity,
// //             create_package(package_uuid),
// //             create_user(presenter_uuid),
// //         ))
// //     });

// //     if let Some(game_state) = games.get(&game_uuid) {
// //         return Ok(Message::Text(
// //             serde_json::to_string(&game_state.game).unwrap(),
// //         ));
// //     };

// //     Err("Cannot create game")
// // }

// // #[derive(Serialize, Debug)]
// // enum ServerMessage<'a> {
// //     Patch,
// //     Full { game: &'a Game },
// // }

// // #[derive(Debug)]
// // pub struct WsState {
// //     games: Mutex<HashMap<Uuid, GameState>>,
// // }

// // impl WsState {
// //     fn new() -> Self {
// //         Self {
// //             games: Mutex::new(HashMap::new()),
// //         }
// //     }
// // }

// // #[derive(Debug)]
// // pub struct GameState {
// //     game: Option<Game>,
// //     txs: Vec<SplitSink<WebSocket, Message>>,
// // }

// // impl GameState {
// //     async fn add_session(&mut self, new_tx: SplitSink<WebSocket, Message>) {
// //         self.txs.push(new_tx);
// //         self.send_msg(Message::Text(String::from("connected")))
// //             .await;
// //         debug!("New user was connected");
// //     }

// //     async fn send_msg(&mut self, msg: Message) {
// //         for tx in self.txs.iter_mut() {
// //             tx.send(Message::Text(String::from("connected")))
// //                 .await
// //                 .unwrap();
// //         }
// //     }
// // }

// // fn create_user(uuid: Uuid) -> User {
// //     User {
// //         uuid,
// //         name: String::from("Username 1"),
// //         password_hash: String::from("Username 1"),
// //         email: String::from("email1@email.ru"),
// //         file_path: None,
// //     }
// // }


