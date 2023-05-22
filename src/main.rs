use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::db::EntryShape;

mod db;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client_id: String = env::var("CLIENT_ID").unwrap();
    let client_secret: String = env::var("CLIENT_SECRET").unwrap();

    let client = reqwest::Client::new();

    let auth_token = services::spotify::get_auth_token(&client, &client_id, &client_secret).await?;

    let mut database = db::Db::new(Path::new("./album_history.json"));

    for artist in get_artists() {
        let albums = &services::spotify::get_artist_albums(&client, &auth_token, &artist).await?;

        // Spotify always returns the albums in desc order of release date
        let latest_album = &albums[0];

        match database.get_by_artist_name(&artist) {
            Some(entry) => {
                if !entry.name.eq(&albums[0].name) {
                    println!(
                        "New album - {} found for artist - {}",
                        &latest_album.name, artist
                    );
                    services::toast::new_album(&artist, &latest_album.name);
                };
            }
            None => (),
        }

        database.insert(EntryShape {
            name: latest_album.name.to_owned(),
            artist,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        })
    }

    database.commit();

    return Ok(());
}

fn get_artists() -> Vec<String> {
    let artist_list = fs::read_to_string("./artists")
        .expect("Could not find a artist list file")
        .clone();
    return artist_list.split("\n").map(String::from).collect();
}
