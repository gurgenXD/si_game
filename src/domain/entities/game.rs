use super::board::Board;
use super::package::Package;
use super::user::User;
use serde::Serialize;
// use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Serialize, Default, Debug)]
pub enum GameStatus {
    #[default]
    Created,
    Started,
    Paused,
    Finished,
}

#[derive(Serialize, Debug)]
pub struct Game {
    pub uuid: Uuid,
    pub capacity: usize,
    pub status: GameStatus,
    pub is_deleted: bool,
    pub package: Package,
    pub round_uuid: Option<Uuid>,
    pub board: Option<Board>,
    presenter: User,
    // players: HashMap<&'a User, isize>,
    // viewers: HashSet<&'a User>,
}

impl Game {
    pub fn new(capacity: usize, package: Package, presenter: User) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            capacity,
            status: GameStatus::Created,
            round_uuid: None,
            board: None,
            package,
            is_deleted: false,
            presenter,
            // players: HashMap::new(),
            // viewers: HashSet::new(),
        }
    }

    // // Players
    // pub fn become_player(&mut self, user: &'a User) -> Result<(), &str> {
    //     if self.players.len() >= self.capacity {
    //         return Err("players capacity is maximum");
    //     }

    //     if self.players.contains_key(user) {
    //         return Err("player already exists");
    //     }

    //     if self.presenter == Some(user) {
    //         self.presenter = None;
    //     }

    //     self.viewers.remove(user);
    //     self.players.insert(user, 0);

    //     Ok(())
    // }

    // pub fn remove_player(&mut self, player: &'a User) {
    //     self.players.remove(player);
    // }

    // pub fn get_players(&self) -> &HashMap<&'a User, isize> {
    //     &self.players
    // }

    // // Viewers
    // pub fn become_viewer(&mut self, user: &'a User) -> Result<(), &str> {
    //     if self.viewers.contains(user) {
    //         return Err("viewer already exists");
    //     }

    //     if self.presenter == Some(user) {
    //         self.presenter = None;
    //     }

    //     self.players.remove(user);
    //     self.viewers.insert(user);

    //     Ok(())
    // }

    // pub fn remove_viewer(&mut self, user: &'a User) {
    //     self.viewers.remove(user);
    // }

    // pub fn get_viewers(&self) -> &HashSet<&'a User> {
    //     &self.viewers
    // }

    // // Presenter
    // pub fn become_presenter(&mut self, user: &'a User) -> Result<(), &str> {
    //     if self.presenter.is_some() {
    //         return Err("presenter already exists");
    //     }

    //     self.presenter = Some(user);

    //     Ok(())
    // }

    // pub fn remove_presenter(&mut self) {
    //     self.presenter = None;
    // }

    // pub fn get_presenter(&self) -> &Option<&'a User> {
    //     &self.presenter
    // }
}
