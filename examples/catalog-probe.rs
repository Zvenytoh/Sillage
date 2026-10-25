//! Explicit public metadata probe; does not create a window, decode audio or open a device.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().skip(1).collect::<Vec<_>>() != ["--metadata-only"] {
        return Err("Argument requis : --metadata-only. Aucun son ni compte utilisé.".into());
    }
    let page = sillage::service::Client::default()
        .home("FR", &sillage::service::Cancellation::default())?;
    let valid = page.country == "FR"
        && page.trending.len() == 20
        && !page.discovery.is_empty()
        && page
            .trending
            .iter()
            .chain(&page.discovery)
            .all(|track| track.valid());
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "country":page.country,"source":page.source,"trending":page.trending.len(),"discovery":page.discovery.len(),
            "with_artwork":page.trending.iter().chain(&page.discovery).filter(|track|track.thumbnail.is_some()).count(),
            "valid":valid,"audio_played":false,"window_created":false,"account_used":false,
            "invalid_metadata":page.trending.iter().chain(&page.discovery).filter(|track|!track.valid()).map(|track|serde_json::json!({
                "valid_id":sillage::service::parse::valid_id(&track.id),
                "title_chars":track.title.chars().count(),"artist_count":track.artists.len(),"duration_ms":track.duration_ms,
                "artwork_host":track.thumbnail.as_ref().and_then(|raw|url::Url::parse(raw).ok()).and_then(|url|url.host_str().map(str::to_owned))
            })).collect::<Vec<_>>()
        }))?
    );
    if !valid {
        return Err("Le catalogue reçu ne satisfait pas les critères de la sonde.".into());
    }
    Ok(())
}
