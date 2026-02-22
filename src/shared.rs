use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)] // derive implements traits passed in
// Serialize and Deserialize allow JSON formatting, Debug allows format printing
pub enum DaemonRequest {
    Search { query: String },
    Download { nasa_id: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DaemonResponse {
    SearchResults { items: Vec<NasaResultShort> },
    DownloadStarted { status: String },
    Error { message: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NasaResultShort {
    pub title: String,
    pub nasa_id: String,
    pub media_type: String,
}