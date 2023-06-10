use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urlencoding::encode;

use crate::models::album::Album;
use crate::models::artist::Artist;

pub fn get_artist_albums(
    client: &Client,
    auth_token: &str,
    artist: &str,
) -> Result<Vec<Album>, reqwest::Error> {
    let top_artist_id =
        get_top_artist_id(encode(&artist).to_string(), &client, &auth_token)?;

    return get_albums_by_artist_id(&top_artist_id, &client, &auth_token);
}

fn get_albums_by_artist_id(
    id: &str,
    client: &Client,
    auth_token: &str,
) -> Result<Vec<Album>, reqwest::Error> {
    #[derive(Deserialize)]
    struct ResBody {
        items: Vec<Album>,
    }

    let query_res: ResBody = client
        .request(
            reqwest::Method::GET,
            "https://api.spotify.com/v1/artists/".to_owned() + id + "/albums",
        )
        .bearer_auth(&auth_token)
        .send()?
        .json()?;

    let artist_albums: Vec<Album> = query_res.items;

    return Ok(artist_albums);
}

fn get_top_artist_id(
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
        .send()?
        .json::<ResBody>()?
        .artists
        .items;

    let top_artist_id = query_res[0].id.to_owned();

    return Ok(top_artist_id);
}

pub fn get_auth_token(
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
        .send()?
        .json()?;

    return Ok(auth_res.access_token);
}
