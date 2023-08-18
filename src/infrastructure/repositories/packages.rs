use crate::domain::entities::package::Package;
use mongodb::Database;

pub async fn create_package(db: &Database, package: &Package) {
    let collection = db.collection::<Package>("packages");

    collection.insert_one(package, None).await.unwrap();
}
