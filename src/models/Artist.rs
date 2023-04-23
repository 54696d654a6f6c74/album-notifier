use serde::Deserialize;

use crate::models::Album::Album;

#[derive(Deserialize)]
pub struct Artist {
    pub id: String,
    items: Option<Vec<Album>>,
}
