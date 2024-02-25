use serde::{Deserialize, Serialize};

//{
//     "data": {
//         "shows": {
//             "edges": [
//                 {
//                     "_id": "vkD8H5e7HsG2jctw9",
//                     "name": "Boruto: Naruto Next Generations",
//                     "englishName": "Boruto: Naruto Next Generations",
//                     "availableEpisodes": {
//                         "sub": 293,
//                         "dub": 255,
//                         "raw": 0
//                     }
//                 },
//              ]
//          }
//      }

#[derive(Deserialize, Serialize)]
pub struct AvailableEpisodes {
    pub sub: u32,
    pub dub: u32,
    pub raw: u32
}

#[derive(Deserialize, Serialize)]
pub struct Show {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "englishName")]
    pub english_name: Option<String>,
    #[serde(rename = "availableEpisodes")]
    pub available_episodes: AvailableEpisodes,
    #[serde(rename = "thumbnail")]
    pub image_url: String
}

#[derive(Deserialize, Serialize)]
pub struct Edges {
    pub edges: Vec<Show>
}

#[derive(Deserialize, Serialize)]
pub struct Shows {
    pub shows: Edges
}

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    pub data: Shows
}
