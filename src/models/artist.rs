use serde::Deserialize;

use crate::models::album::Album;

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub id: String,
    pub items: Option<Vec<Album>>,
}
