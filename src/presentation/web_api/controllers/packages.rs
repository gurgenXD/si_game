use crate::domain::entities::{
    package::Package, package::Question, package::QuestionType, package::Round, package::RoundType,
    package::Topic,
};
use axum::{extract::Path, Json};
use chrono::Utc;
use mongodb::{options::ClientOptions, Client};
use uuid::Uuid;

pub async fn get_packages() -> Json<Vec<Package>> {
    let package = create_package_obj(Uuid::new_v4());

    dbg!("hello");
    Json(vec![package])
}

pub async fn get_package(Path(uuid): Path<Uuid>) -> Json<Package> {
    let package = create_package_obj(uuid);

    dbg!("hello");
    Json(package)
}

pub async fn create_package() -> Json<Package> {
    let uuid = Uuid::new_v4();
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    client_options.app_name = Some(String::from("SI Game"));

    let client = Client::with_options(client_options).unwrap();

    for db_name in client.list_database_names(None, None).await.unwrap() {
        println!("DB: {}", db_name);
    }

    let db = client.database("si_game");

    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("COLLECTION: {}", collection_name);
    }

    let collection = db.collection::<Package>("packages");

    let package = create_package_obj(uuid);

    collection.insert_one(&package, None).await.unwrap();

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
