use serde::Serialize;

use super::package::{Question, Round};

#[derive(Serialize, Debug)]
pub struct Board {
    pub is_played: bool,

    pub rows: Vec<BoardRow>,
}

#[derive(Serialize, Debug)]
pub struct BoardRow {
    pub title: String,
    pub is_played: bool,

    pub cells: Vec<BoardCell>,
}

#[derive(Serialize, Debug)]
pub struct BoardCell {
    pub question: Question,
    pub is_played: bool,
}

impl Board {
    pub fn new(round: &Round) -> Self {
        let mut board = Self {
            is_played: false,
            rows: Vec::new(),
        };

        for topic in round.topics.iter() {
            let row = BoardRow::new(
                topic.title.clone(),
                topic
                    .questions
                    .iter()
                    .map(|x| BoardCell::new(x.clone()))
                    .collect(),
            );

            board.rows.push(row)
        }

        board
    }
}

impl BoardRow {
    pub fn new(title: String, cells: Vec<BoardCell>) -> Self {
        Self {
            title,
            is_played: false,
            cells,
        }
    }
}

impl BoardCell {
    pub fn new(question: Question) -> Self {
        Self {
            is_played: false,
            question,
        }
    }
}
