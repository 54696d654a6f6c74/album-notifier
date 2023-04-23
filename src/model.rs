use crate::db::Db;
use mongodb::bson::Document;

struct Model {
    db: Db<Document>,
}

impl Model {
    async fn new() -> Self {
        return Model {
            db: Db::<Document>::new("album_notifier", "artist_albums")
                .await
                .expect("Failed to get DB handle"),
        };
    }
}
