use dotenv::dotenv;
use models::Album::Album;
use models::Artist::Artist;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use urlencoding::encode;

mod db;
mod model;
mod models;

#[derive(Serialize, Deserialize)]
struct AuthResponseBody {
    access_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client_id: String = env::var("CLIENT_ID").unwrap();
    let client_secret: String = env::var("CLIENT_SECRET").unwrap();

    let client = reqwest::Client::new();
    
    let auth_token = get_auth_token(&client, &client_id, &client_secret).await?;

    for band in get_bands() {
        let albums = &get_artist_albums(&client, &auth_token, &band).await?;
        print_artist_albums(albums);
    }

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

fn print_latest_album(albums: &Vec<Album>) {
    for i in 0..albums.len() {
        println!("{:#?}", albums[i].name);
    }
}

async fn get_albums_by_artist_id(
    id: &str,
    client: &Client,
    auth_token: &str,
) -> Result<Vec<Album>, reqwest::Error> {
    let query_res = client
        .request(
            reqwest::Method::GET,
            "https://api.spotify.com/v1/artists/".to_owned() + id + "/albums",
        )
        .bearer_auth(&auth_token)
        .send()
        .await?
        .json::<Artist>()
        .await?;

    let artist_albums: Vec<Album> = query_res.items.unwrap();

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
    let mut form = HashMap::new();
    form.insert("grant_type", "client_credentials");

    let auth_res: AuthResponseBody = client
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

fn print_artist_albums(artist_albums: &Vec<Album>) {
    for album in artist_albums {
        println!("{}", album.name);
    }
}

fn get_bands() -> Vec<String> {
    let bandlist = fs::read_to_string("./bands")
        .expect("Could not find a bandlist file")
        .clone();
    return bandlist.split("\n").map(String::from).collect();
}
