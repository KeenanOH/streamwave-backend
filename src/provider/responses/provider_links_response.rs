use serde::{Deserialize, Serialize};

//{
//     "links": [
//         {
//             "link": "https://www084.vipanicdn.net/streamhls/613ef03091ebda83e483714d2cc71dac/ep.1.1703887888.m3u8",
//             "hls": true,
//             "mp4": false,
//             "resolutionStr": "hls P",
//             "priority": 3,
//             "fromCache": "2024-02-20T01:55:36.849Z"
//         },
//     ]
// }

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub link: String,
    // pub hls: bool,
    // pub mp4: bool,
    // #[serde(rename = "resolutionStr")]
    // pub resolution_str: String,
    pub priority: Option<f32>,
    // #[serde(rename = "fromCache")]
    // pub from_cache: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProviderLinksResponse {
    pub links: Vec<Link>,
}
