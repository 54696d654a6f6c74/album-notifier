use mongodb::{
    error::Error, options::ClientOptions, results::InsertOneResult, Client, Collection, Database,
};
use serde::{Deserialize, Serialize};

pub struct Db<T> {
    client: Client,
    database: Database,
    collection: Collection<T>,
}

impl<'a, T> Db<T>
where
    T: Serialize + Deserialize<'a>,
{
    pub async fn new(db_name: &str, collection_name: &str) -> Result<Db<T>, Error> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

        let client = Client::with_options(client_options)?;
        let database = client.database(db_name);
        let collection = database.collection::<T>(collection_name);

        return Ok(Db::<T> {
            client,
            database,
            collection,
        });
    }

    pub async fn insert(&self, obj: T) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(obj, None).await
    }
}
