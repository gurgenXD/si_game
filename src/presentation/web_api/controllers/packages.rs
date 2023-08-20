use crate::{
    domain::entities::{
        package::Package,
        package::Question,
        package::QuestionType,
        package::RoundType,
        package::Topic,
        package::{self, Round},
    },
    infrastructure::repositories,
};
use axum::{extract, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;

pub async fn get_packages() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let conn = repositories::connection::get_connection().await;
    let packages = repositories::packages::get_packages(&conn).await;

    Ok((StatusCode::OK, Json(packages)))
}

pub async fn get_package(
    extract::Path(uuid): extract::Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let conn = repositories::connection::get_connection().await;

    let package = repositories::packages::get_package(&conn, uuid).await;
    dbg!(package);

    if let Some(package) = repositories::packages::get_package(&conn, uuid).await {
        dbg!(package);
        return Ok((StatusCode::OK, ()));
    }

    Err((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "message": format!("Package with UUID: {} not found", uuid)
        })),
    ))
}

pub async fn create_package(
    extract::Json(package): extract::Json<Package>,
) -> Result<impl IntoResponse, (StatusCode, ())> {
    let conn = repositories::connection::get_connection().await;
    repositories::packages::create_package(&conn, &package).await;

    Ok((StatusCode::NO_CONTENT, ()))
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
