use crate::app::{AppWindow, SearchRow};
use sillage::model::Track;
use slint::Model;

pub fn row(track: &Track) -> SearchRow {
    SearchRow {
        id: track.id.clone().into(),
        title: track.title.clone().into(),
        artist: track.artists.join(" · ").into(),
        artwork: slint::Image::default(),
        duration: track
            .duration_ms
            .map(|ms| format!("{}:{:02}", ms / 60000, ms / 1000 % 60))
            .unwrap_or_default()
            .into(),
        duration_ms: track.duration_ms.unwrap_or(0) as f32,
        thumbnail: track.thumbnail.clone().unwrap_or_default().into(),
    }
}
pub fn track(row: &SearchRow) -> Track {
    Track {
        id: row.id.to_string(),
        title: row.title.to_string(),
        artists: vec![row.artist.to_string()],
        duration_ms: (row.duration_ms > 0.0).then_some(row.duration_ms as u64),
        thumbnail: (!row.thumbnail.is_empty()).then(|| row.thumbnail.to_string()),
    }
}
pub fn model(tracks: &[Track]) -> slint::ModelRc<SearchRow> {
    slint::ModelRc::new(slint::VecModel::from(
        tracks.iter().map(row).collect::<Vec<_>>(),
    ))
}
pub fn image(ui: &AppWindow, id: &str, image: &slint::Image) {
    for rows in [
        ui.get_results(),
        ui.get_saved_rows(),
        ui.get_recent_rows(),
        ui.get_queue_rows(),
        ui.get_up_next_rows(),
        ui.get_trending_rows(),
        ui.get_discovery_rows(),
    ] {
        if let Some(model) = rows.as_any().downcast_ref::<slint::VecModel<SearchRow>>() {
            for index in 0..model.row_count() {
                if let Some(mut row) = model.row_data(index)
                    && row.id.as_str() == id
                {
                    row.artwork = image.clone();
                    model.set_row_data(index, row);
                }
            }
        }
    }
    if ui.get_selected_id().as_str() == id {
        ui.set_track_artwork(image.clone());
    }
}
