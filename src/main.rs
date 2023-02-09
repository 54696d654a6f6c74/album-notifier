use std::collections::HashMap;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;


const CLIENT_ID: &str = "a288ad6e4e1646a79b0c52bd43d34881";
const CLIENT_SECRET: &str = "7de880b72a1d4087ae383a19cfb01292";

// To send an auth request I need to hit: https://accounts.spotify.com/api/token
// Check this page for more details - https://developer.spotify.com/documentation/general/guides/authorization/client-credentials/

#[derive(Serialize, Deserialize)]
struct AuthResponseBody {
    access_token: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut form = HashMap::new();
    form.insert("grant_type", "client_credentials");

    let auth_token = get_auth_token(&client).await?;

    let top_artist_id = get_top_artist_id("BeyondTheBlack", &client, &auth_token).await?;

    let artist_albums = get_albums_by_artist_id(&top_artist_id, &client, &auth_token).await?;

    return Ok(print_artist_albums(&artist_albums));
}

async fn get_albums_by_artist_id(id: &str, client: &Client, auth_token: &str) -> Result<Vec<Value>, reqwest::Error> {
    let query_res = client
        .request(reqwest::Method::GET, "https://api.spotify.com/v1/artists/".to_owned() + id + "/albums")
        .bearer_auth(&auth_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    let artist_albums = query_res.get("items").unwrap().as_array().unwrap().to_owned();

    return Ok(artist_albums);
}

async fn get_top_artist_id(artist_name: &str, client: &Client, auth_token: &str) -> Result<String, reqwest::Error> {
    let query_res = client
        .request(reqwest::Method::GET, "https://api.spotify.com/v1/search?type=artist&q=".to_owned() + artist_name)
        .bearer_auth(&auth_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    let top_artist_id: String = query_res
        .get("artists")
        .unwrap()
        .get("items")
        .unwrap()
        .get(0)
        .unwrap()
        .get("id")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();

    return Ok(top_artist_id);
}

async fn get_auth_token(client: &Client) -> Result<String, reqwest::Error> {
    let mut form = HashMap::new();
    form.insert("grant_type", "client_credentials");

    let auth_res: AuthResponseBody = client
        .request(reqwest::Method::POST, "https://accounts.spotify.com/api/token")
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .form(&form)
        .send()
        .await?
        .json()
        .await?;
    
    return Ok(auth_res.access_token);
}

fn print_artist_albums(artist_albums: &Vec<Value>) {
    for album in artist_albums {
        let album_name = album.get("name").unwrap().as_str().unwrap();
        println!("{:#?}", album_name);
    }
}
