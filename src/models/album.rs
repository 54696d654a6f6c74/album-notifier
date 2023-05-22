use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Album {
    pub name: String,
}
