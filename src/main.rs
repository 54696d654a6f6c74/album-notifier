use dotenv::dotenv;
use models::album::Album;
use models::artist::Artist;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use urlencoding::encode;

use crate::db::EntryShape;

mod db;
mod models;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client_id: String = env::var("CLIENT_ID").unwrap();
    let client_secret: String = env::var("CLIENT_SECRET").unwrap();

    let client = reqwest::Client::new();
    
    let auth_token = get_auth_token(&client, &client_id, &client_secret).await?;

    let mut database = db::Db::new(Path::new("./album_history.json"));

    for band in get_bands() {
        let albums = &get_artist_albums(&client, &auth_token, &band).await?;
        database.insert(EntryShape {name: albums[0].name.to_owned(), band, timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()})
    }

    database.commit();

    return Ok(());
}

async fn get_artist_albums(
    client: &Client,
    auth_token: &str,
    band: &str,
) -> Result<Vec<Album>, reqwest::Error> {
    let top_artist_id = get_top_artist_id(encode(&band).to_string(), &client, &auth_token).await?;

    return get_albums_by_artist_id(&top_artist_id, &client, &auth_token).await;
}

async fn get_albums_by_artist_id(
    id: &str,
    client: &Client,
    auth_token: &str,
) -> Result<Vec<Album>, reqwest::Error> {
    #[derive(Deserialize)]
    struct ResBody {
        items: Vec<Album>
    }

    let query_res: ResBody = client
        .request(
            reqwest::Method::GET,
            "https://api.spotify.com/v1/artists/".to_owned() + id + "/albums",
        )
        .bearer_auth(&auth_token)
        .send()
        .await?
        .json()
        .await?;

    let artist_albums: Vec<Album> = query_res.items;

    return Ok(artist_albums);
}

async fn get_top_artist_id(
    artist_name: String,
    client: &Client,
    auth_token: &str,
) -> Result<String, reqwest::Error> {
    #[derive(Deserialize)]
    struct ResBody {
        artists: Artists,
    }

    #[derive(Deserialize)]
    struct Artists {
        items: Vec<Artist>,
    }

    let query_res = client
        .request(
            reqwest::Method::GET,
            "https://api.spotify.com/v1/search?type=artist&q=".to_owned() + &artist_name,
        )
        .bearer_auth(&auth_token)
        .send()
        .await?
        .json::<ResBody>()
        .await?
        .artists
        .items;

    let top_artist_id = query_res[0].id.to_owned();

    return Ok(top_artist_id);
}

async fn get_auth_token(
    client: &Client,
    client_id: &str,
    client_secret: &str,
) -> Result<String, reqwest::Error> {
    #[derive(Serialize, Deserialize)]
    struct ResBody {
        access_token: String,
    }

    let mut form = HashMap::new();
    form.insert("grant_type", "client_credentials");

    let auth_res: ResBody = client
        .request(
            reqwest::Method::POST,
            "https://accounts.spotify.com/api/token",
        )
        .basic_auth(client_id, Some(client_secret))
        .form(&form)
        .send()
        .await?
        .json()
        .await?;

    return Ok(auth_res.access_token);
}

fn get_bands() -> Vec<String> {
    let bandlist = fs::read_to_string("./bands")
        .expect("Could not find a bandlist file")
        .clone();
    return bandlist.split("\n").map(String::from).collect();
}
