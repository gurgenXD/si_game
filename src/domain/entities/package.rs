use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum RoundType {
    Regular,
    Final,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    Regular,
    CatInABag,
    Auction,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Package {
    pub uuid: Uuid,
    pub title: String,
    pub author_uuid: Uuid,
    pub created_at: DateTime<Utc>,

    pub rounds: Vec<Round>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Round {
    pub uuid: Uuid,
    pub order: usize,
    pub type_: RoundType,

    pub topics: Vec<Topic>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub uuid: Uuid,
    pub title: String,

    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    pub uuid: Uuid,
    pub text: String,
    pub answer: String,
    pub cost: usize,
    pub type_: QuestionType,
    pub file_path: Option<String>,
}
