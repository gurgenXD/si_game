use crate::domain::entities::package::Package;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Database;
use uuid::Uuid;

pub async fn get_package(db: &Database, uuid: Uuid) -> Option<Package> {
    let collection = db.collection::<Package>("packages");

    let package = collection
        .find_one(doc! {"uuid": uuid}, None)
        .await
        .unwrap();

    package
}

pub async fn get_packages(db: &Database) -> Vec<Package> {
    let collection = db.collection::<Package>("packages");
    let cursor = collection.find(None, None).await.unwrap();
    let packages: Vec<Package> = cursor.try_collect().await.unwrap();

    packages
}

pub async fn create_package(db: &Database, package: &Package) {
    let collection = db.collection::<Package>("packages");

    collection.insert_one(package, None).await.unwrap();
}
