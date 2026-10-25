use serde_json::json;
use sillage::service::{Cancellation, ServiceError, parse};

#[test]
fn rejects_invalid_video_ids() {
    assert!(parse::valid_id("abcDE_12-34"));
    for id in ["", "short", "abcdefghij/", "abcdefghij?", "éabcdefghi"] {
        assert!(!parse::valid_id(id));
    }
}

#[test]
fn extracts_tracks_and_deduplicates_ids() {
    let row = json!({"musicResponsiveListItemRenderer": {
        "playlistItemData": {"videoId": "abcdefghijk"},
        "flexColumns": [{"musicResponsiveListItemFlexColumnRenderer": {"text": {"runs": [{"text": "Example track"}]}}}],
        "futureField": {"unused": true}
    }});
    let result = parse::search(&json!({"contents": {"rows": [row.clone(), row]}})).expect("search");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "abcdefghijk");
    assert_eq!(result[0].title, "Example track");
}

#[test]
fn accepts_empty_results_but_rejects_wrong_response_shape() {
    assert!(
        parse::search(&json!({"contents": {}}))
            .expect("empty results")
            .is_empty()
    );
    assert_eq!(
        parse::search(&json!({"error": "failure"})),
        Err(ServiceError::InvalidResponse)
    );
}

#[test]
fn cancellation_is_shared_between_request_phases() {
    let cancel = Cancellation::default();
    let copy = cancel.clone();
    assert_eq!(cancel.check(), Ok(()));
    copy.cancel();
    assert_eq!(cancel.check(), Err(ServiceError::Cancelled));
}
