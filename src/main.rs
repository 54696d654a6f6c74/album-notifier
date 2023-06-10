extern crate cronjob;

use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::Path;
use cronjob::CronJob;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::db::EntryShape;

mod db;
mod models;
mod services;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run("");

    setup_cron();

    return Ok(());
}

fn run(_: &str) {
    dotenv().ok();

    let client_id: String = env::var("CLIENT_ID").expect("env var `CLIENT_ID` is not set");
    let client_secret: String =
        env::var("CLIENT_SECRET").expect("env var `CLIENT_SECRET` is not set");
    let artist_list_path: String =
        env::var("ARTIST_LIST_PATH").expect("env var `ARTIST_LIST_PATH` is not set");

    let client = reqwest::blocking::Client::new();

    let auth_token = services::spotify::get_auth_token(&client, &client_id, &client_secret).expect("Failed to get auth token");

    let mut database = db::Db::new(Path::new("./album_history.json"));

    for artist in get_artists(&artist_list_path) {
        let albums = &services::spotify::get_artist_albums(&client, &auth_token, &artist).expect(&("Failed to get albums for artist: ".to_owned() + &artist));

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
}

fn get_artists(path: &str) -> Vec<String> {
    let artist_list = fs::read_to_string(path)
        .expect("Could not find the artist list file")
        .clone();
    return artist_list.split("\n").map(String::from).collect();
}

fn setup_cron() {
    let mut cron = CronJob::new("AlbumNotifier", run);
    cron.minutes("30");
    cron.hours("9,11,13,15,17,19,20,21");

    // Set offset for UTC
    cron.offset(0);

    cron.start_job();
}
