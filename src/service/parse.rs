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
    let artists = column(1)["runs"]
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
                .is_some_and(|url| url.len() <= 4096)
        })
        .min_by_key(|thumbnail| thumbnail["width"].as_u64().unwrap_or(0).abs_diff(320))
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
