use serde::{Deserialize, Serialize};

//{
//     "data": {
//         "episode": {
//             "episodeString": "1",
//             "sourceUrls": [
//                 {
//                     "sourceUrl": " ",
//                     "priority": 7.7,
//                     "sourceName": "Luf-mp4",
//                     "type": "iframe",
//                     "className": "",
//                     "streamerId": "allanime"
//                 }
//             ],
//             "episodeInfo": {
//                 "notes": "The Apocalypse"
//             }
//         }
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct EpisodeInfo {
    pub notes: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Source {
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
    pub priority: f32,
    #[serde(rename = "sourceName")]
    pub source_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Episode {
    #[serde(rename = "episodeString")]
    pub episode_string: String,
    #[serde(rename = "sourceUrls")]
    pub source_urls: Vec<Source>,
    #[serde(rename = "episodeInfo")]
    pub episode_info: EpisodeInfo,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub episode: Option<Episode>,
}

#[derive(Serialize, Deserialize)]
pub struct EpisodeResponse {
    pub data: Data,
}
