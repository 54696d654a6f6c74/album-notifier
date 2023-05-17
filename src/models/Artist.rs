use serde::Deserialize;

use crate::models::Album::Album;

#[derive(Deserialize)]
pub struct Artist {
    pub id: String,
    pub items: Option<Vec<Album>>,
}
