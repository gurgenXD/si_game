use crate::{
    domain::entities::{
        package::Package, package::Question, package::QuestionType, package::Round,
        package::RoundType, package::Topic,
    },
    infrastructure::repositories,
};
use axum::{extract, Json};
use chrono::Utc;
use uuid::Uuid;

pub async fn get_packages() -> Json<Vec<Package>> {
    let package = create_package_obj(Uuid::new_v4());

    dbg!("hello");
    Json(vec![package])
}

pub async fn get_package(extract::Path(uuid): extract::Path<Uuid>) -> Json<Package> {
    let package = create_package_obj(uuid);

    dbg!("hello");
    Json(package)
}

pub async fn create_package(extract::Json(package): extract::Json<Package>) -> Json<Package> {
    let conn = repositories::connection::get_connection().await;
    repositories::packages::create_package(&conn, &package).await;

    dbg!("hello");
    Json(package)
}

fn create_package_obj(uuid: Uuid) -> Package {
    Package {
        uuid,
        title: String::from("Package 1"),
        author_uuid: Uuid::new_v4(),
        created_at: Utc::now(),
        rounds: vec![
            Round {
                uuid: Uuid::new_v4(),
                order: 0,
                type_: RoundType::Regular,
                topics: vec![
                    Topic {
                        uuid: Uuid::new_v4(),
                        title: String::from("Topic 1.1"),
                        questions: vec![
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 1.1.1"),
                                answer: String::from("Answer 1.1.1"),
                                cost: 100,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 1.1.2"),
                                answer: String::from("Answer 1.1.2"),
                                cost: 200,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 1.1.3"),
                                answer: String::from("Answer 1.1.3"),
                                cost: 300,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                        ],
                    },
                    Topic {
                        uuid: Uuid::new_v4(),
                        title: String::from("Topic 1.2"),
                        questions: vec![
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 1.2.1"),
                                answer: String::from("Answer 1.2.1"),
                                cost: 100,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 1.2.2"),
                                answer: String::from("Answer 1.2.2"),
                                cost: 200,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 1.2.3"),
                                answer: String::from("Answer 1.2.3"),
                                cost: 300,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                        ],
                    },
                ],
            },
            Round {
                uuid: Uuid::new_v4(),
                order: 0,
                type_: RoundType::Regular,
                topics: vec![
                    Topic {
                        uuid: Uuid::new_v4(),
                        title: String::from("Topic 2.1"),
                        questions: vec![
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 2.1.1"),
                                answer: String::from("Answer 2.1.1"),
                                cost: 100,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 2.1.2"),
                                answer: String::from("Answer 2.1.2"),
                                cost: 200,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 2.1.3"),
                                answer: String::from("Answer 2.1.3"),
                                cost: 300,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                        ],
                    },
                    Topic {
                        uuid: Uuid::new_v4(),
                        title: String::from("Topic 2.2"),
                        questions: vec![
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 2.2.1"),
                                answer: String::from("Answer 2.2.1"),
                                cost: 100,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 2.2.2"),
                                answer: String::from("Answer 2.2.2"),
                                cost: 200,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                            Question {
                                uuid: Uuid::new_v4(),
                                text: String::from("Question 2.2.3"),
                                answer: String::from("Answer 2.2.3"),
                                cost: 300,
                                type_: QuestionType::Regular,
                                file_path: None,
                            },
                        ],
                    },
                ],
            },
        ],
    }
}
