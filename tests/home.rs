use serde_json::json;
use sillage::service::parse;
#[test]
fn france_trending_route_is_taken_from_the_catalog() {
    let reply = json!({"contents":{"musicTwoRowItemRenderer":{"title":{"runs":[{"text":"Les 20 clips tendance France"}]},"navigationEndpoint":{"browseEndpoint":{"browseId":"VLexample_playlist"}},"futureField":true}}});
    assert_eq!(
        parse::chart_playlist(&reply).as_deref(),
        Some("VLexample_playlist")
    );
    assert_eq!(parse::chart_playlist(&json!({"contents":{}})), None);
}
#[test]
fn a_collection_cannot_become_a_playable_song() {
    let reply = json!({"contents":{"musicTwoRowItemRenderer":{"title":{"simpleText":"Un album"},"navigationEndpoint":{"browseEndpoint":{"browseId":"MPREexample"}}}}});
    assert!(parse::search(&reply).expect("valid response").is_empty());
}

#[test]
fn rejected_artwork_does_not_discard_a_playable_track() {
    let reply = json!({"contents":{"musicTwoRowItemRenderer":{
        "title":{"simpleText":"Titre"},"videoId":"abcdefghijk",
        "thumbnail":{"thumbnails":[{"url":"https://untrusted.example/cover.jpg","width":512}]}
    }}});
    let tracks = parse::search(&reply).expect("valid response");
    assert_eq!(tracks.len(), 1);
    assert!(tracks[0].thumbnail.is_none());
    assert!(tracks[0].valid());
}

#[test]
fn artwork_requires_an_exact_https_host() {
    use sillage::service::valid_artwork_url;
    assert!(valid_artwork_url(
        "https://yt3.googleusercontent.com/example"
    ));
    for url in [
        "http://i.ytimg.com/vi/abcdefghijk/hqdefault.jpg",
        "https://i.ytimg.com.evil.example/cover.jpg",
        "https://user@i.ytimg.com/cover.jpg",
        "https://i.ytimg.com:444/cover.jpg",
    ] {
        assert!(!valid_artwork_url(url));
    }
}
