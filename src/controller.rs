use crate::{error::AppError, models::FeedSource};

pub async fn fetch_feed(url: &str) -> Result<FeedSource, AppError> {
    let content = reqwest::get(url).await?.bytes().await?;

    if let Ok(rss_channel) = rss::Channel::read_from(&content[..]) {
        return Ok(FeedSource::Rss(rss_channel));
    }

    if let Ok(atom_feed) = atom_syndication::Feed::read_from(&content[..]) {
        return Ok(FeedSource::Atom(atom_feed));
    }

    Err(AppError::ParseFeedError(
        "Couldn't parse as neither atom nor rss feed".to_owned(),
    ))
}
