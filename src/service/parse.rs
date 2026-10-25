use super::ServiceError;
use crate::model::Track;
use serde_json::Value;
use std::collections::HashSet;

pub fn valid_id(id: &str) -> bool {
    id.len() == 11
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}

fn text(value: &Value) -> String {
    let text = if let Some(simple) = value["simpleText"].as_str() {
        simple.to_owned()
    } else {
        value["runs"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|run| run["text"].as_str())
            .collect::<String>()
    };
    text.chars().take(512).collect()
}

pub fn chart_playlist(response: &Value) -> Option<String> {
    fn walk(node: &Value, depth: usize) -> Option<String> {
        if depth > 64 {
            return None;
        }
        if let Some(item) = node.get("musicTwoRowItemRenderer") {
            let title = text(&item["title"]).to_lowercase();
            if title.contains("tendance") && title.contains("france") {
                return item
                    .pointer("/navigationEndpoint/browseEndpoint/browseId")
                    .and_then(Value::as_str)
                    .filter(|id| {
                        id.starts_with("VL")
                            && id.len() <= 160
                            && id
                                .bytes()
                                .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-'))
                    })
                    .map(str::to_owned);
            }
        }
        match node {
            Value::Array(items) => items.iter().find_map(|item| walk(item, depth + 1)),
            Value::Object(items) => items.values().find_map(|item| walk(item, depth + 1)),
            _ => None,
        }
    }
    walk(response, 0)
}

pub fn section(response: &Value, title: &str) -> Vec<Track> {
    fn find<'a>(node: &'a Value, title: &str, depth: usize) -> Option<&'a Value> {
        if depth > 64 {
            return None;
        }
        if let Some(shelf) = node.get("musicCarouselShelfRenderer")
            && text(&shelf["header"]["musicCarouselShelfBasicHeaderRenderer"]["title"]) == title
        {
            return Some(shelf);
        }
        match node {
            Value::Array(items) => items.iter().find_map(|item| find(item, title, depth + 1)),
            Value::Object(items) => items.values().find_map(|item| find(item, title, depth + 1)),
            _ => None,
        }
    }
    find(response, title, 0)
        .and_then(|shelf| search(&serde_json::json!({"contents":shelf})).ok())
        .unwrap_or_default()
}

fn duration_ms(node: &Value) -> Option<u64> {
    let raw = text(&node["fixedColumns"][0]["musicResponsiveListItemFixedColumnRenderer"]["text"]);
    let mut seconds = 0_u64;
    let mut parts = 0;
    for part in raw.split(':') {
        let value = part.parse::<u64>().ok()?;
        if parts > 0 && value >= 60 {
            return None;
        }
        seconds = seconds.checked_mul(60)?.checked_add(value)?;
        parts += 1;
    }
    if !(2..=3).contains(&parts) {
        return None;
    }
    seconds.checked_mul(1000)
}

fn track(node: &Value) -> Option<Track> {
    let column = |index: usize| {
        &node["flexColumns"][index]["musicResponsiveListItemFlexColumnRenderer"]["text"]
    };
    let title = [text(&node["title"]), text(column(0))]
        .into_iter()
        .find(|title| !title.trim().is_empty())?;
    let id = ["/playlistItemData/videoId", "/videoId", "/navigationEndpoint/watchEndpoint/videoId", "/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text/runs/0/navigationEndpoint/watchEndpoint/videoId", "/overlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/videoId"]
        .iter().find_map(|path| node.pointer(path).and_then(Value::as_str).filter(|id| valid_id(id)))?;
    let artist_text = if column(1).is_object() {
        column(1)
    } else {
        &node["subtitle"]
    };
    let artists = artist_text["runs"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|run| {
            run.pointer("/navigationEndpoint/browseEndpoint/browseId")
                .and_then(Value::as_str)
                .is_some_and(|id| id.starts_with("UC"))
        })
        .filter_map(|run| run["text"].as_str())
        .map(|name| name.chars().take(256).collect())
        .take(8)
        .collect();
    Some(Track {
        id: id.to_owned(),
        title,
        artists,
        duration_ms: duration_ms(node),
        thumbnail: [
            "/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails",
            "/thumbnailRenderer/musicThumbnailRenderer/thumbnail/thumbnails",
            "/thumbnail/thumbnails",
        ]
        .iter()
        .filter_map(|path| node.pointer(path).and_then(Value::as_array))
        .flatten()
        .filter(|thumbnail| {
            thumbnail["url"]
                .as_str()
                .is_some_and(|url| url.len() <= 4096 && super::valid_artwork_url(url))
        })
        .min_by_key(|thumbnail| {
            let width = thumbnail["width"].as_u64().unwrap_or(0);
            let height = thumbnail["height"].as_u64().unwrap_or(0);
            let aspect_error =
                width.abs_diff(height).saturating_mul(512) / width.max(height).max(1);
            let size_error = width.abs_diff(512) + height.abs_diff(512);
            (aspect_error, size_error)
        })
        .and_then(|thumbnail| thumbnail["url"].as_str())
        .map(str::to_owned),
    })
}

pub fn search(response: &Value) -> Result<Vec<Track>, ServiceError> {
    let contents = response
        .get("contents")
        .filter(|contents| contents.is_object())
        .ok_or(ServiceError::InvalidResponse)?;
    fn walk(node: &Value, depth: usize, seen: &mut HashSet<String>, tracks: &mut Vec<Track>) {
        if depth > 64 || tracks.len() >= 100 {
            return;
        }
        match node {
            Value::Array(values) => {
                for value in values {
                    walk(value, depth + 1, seen, tracks);
                }
            }
            Value::Object(map) => {
                for (key, value) in map {
                    if matches!(
                        key.as_str(),
                        "musicResponsiveListItemRenderer" | "musicTwoRowItemRenderer"
                    ) {
                        if let Some(track) = track(value)
                            && tracks.len() < 100
                            && track.valid()
                            && seen.insert(track.id.clone())
                        {
                            tracks.push(track);
                        }
                    } else {
                        walk(value, depth + 1, seen, tracks);
                    }
                }
            }
            _ => {}
        }
    }
    let mut tracks = Vec::new();
    walk(contents, 0, &mut HashSet::new(), &mut tracks);
    Ok(tracks)
}
