use mongodb::{options::ClientOptions, Client, Database};

pub async fn get_connection() -> Database {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();

    client_options.app_name = Some(String::from("SI Game"));

    let client = Client::with_options(client_options).unwrap();
    let db = client.database("si_game");

    db
}
