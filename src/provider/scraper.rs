use reqwest::Client as ReqwestClient;
use crate::provider::responses::search_response::{SearchResponse};
use crate::provider::responses::episode_response::{EpisodeResponse, Source};
use crate::provider::responses::provider_links_response::ProviderLinksResponse;

pub async fn search(client: ReqwestClient, base_url: &str, show_name: &str) -> SearchResponse {
    let variables = format!("{{\"search\":{{\"query\":\"{}\"}},\"countryOrigin\":\"ALL\"}}", show_name);
    let query = "query (
        $search: SearchInput
        $translationType: VaildTranslationTypeEnumType
    ) {
        shows(
            search: $search
            limit: 40
            page: 1
            translationType: $translationType
        ) {
            edges {
                _id
                name
                englishName
                availableEpisodes
                thumbnail
            }
        }
    }";
    let url = format!("{}/api?variables={}&query={}", base_url, variables, query);
    let response = client.get(url)
        .header("Origin", "https://allmanga.to")
        .header("Referer", "https://allmanga.to/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.3")
        .send()
        .await
        .unwrap();

    return response.json::<SearchResponse>().await.unwrap();
}

pub async fn get_episode(client: ReqwestClient, base_url: &str, show_id: &str, translation_type: &str, episode_num: &str) -> EpisodeResponse {
    let variables = format!(r#"{{"showId":"{}","translationType":"{}","episodeString":"{}"}}"#, show_id, translation_type, episode_num);
    let query = "query ($showId: String!, $translationType: VaildTranslationTypeEnumType!, $episodeString: String!) {
        episode(
            showId: $showId
            translationType: $translationType
            episodeString: $episodeString
        ) {
            episodeString
            sourceUrls
            episodeInfo {
                notes
            }
        }
    }";

    let url = format!("{}/api?variables={}&query={}", base_url, variables, query);
    let response = client.get(url)
        .header("Origin", "https://allmanga.to")
        .header("Referer", "https://allmanga.to/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.3")
        .send()
        .await
        .unwrap();

    let response_json = response.json::<EpisodeResponse>().await.unwrap();
    return response_json;
}

pub fn decrypt_allanime(source_url: &str) -> String {
    let decoded_link: String = hex::decode(&source_url[2..])
        .unwrap()
        .into_iter()
        .map(|segment| (segment ^ 56) as char)
        .collect();

    return decoded_link.replace(
        "/apivtwo/clock?id=",
        "https://allanime.day/apivtwo/clock.json?id=",
    );
}

pub async fn get_best_streaming_link(client: ReqwestClient, episode_response: EpisodeResponse) -> [Option<String>; 2] {
    let mut best_source: Option<Source> = None;
    let mut priority: f32 = 0.0;
    let episode = episode_response.data.episode;

    if episode.is_none() {
        return [None, None]
    }

    for source in episode.unwrap().source_urls {
        if source.priority > priority {
            if ["Default", "Sak", "Luf-mp4"].contains(&source.source_name.as_str()) {
                priority = source.priority;
                best_source = Some(source);
            }
        }
    }

    if best_source.is_none() {
        return [None, None];
    }

    return [get_streaming_link(client, decrypt_allanime(best_source.clone().unwrap().source_url.as_str()).as_str()).await, Some(best_source.unwrap().source_name)];
}

// Sak, Luf-mp4, Default
pub async fn get_streaming_link(client: ReqwestClient, source_url: &str) -> Option<String> {
    let response = client.get(source_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.3")
        .send()
        .await
        .unwrap();

    let response_json = response.json::<ProviderLinksResponse>().await.unwrap();
    let mut response: Option<String> = None;
    let mut priority: f32 = 0.0;

    for link in response_json.links {
        if let Some(current_priority) = link.priority {
            if current_priority > priority {
                priority = current_priority;
                response = Some(link.link)
            }
        } else {
            return Some(link.link);
        }
    }

    return response;
}
