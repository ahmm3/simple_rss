use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use uuid::Uuid;

use crate::error::AppError;

/// Wraps struct that we get from parsing libraries
#[derive(Debug)]
pub enum FeedSource {
    Atom(atom_syndication::Feed),
    Rss(rss::Channel),
}

#[derive(Debug, Serialize)]
pub enum FeedType {
    Atom,
    Rss,
}

/// Represents the `feed` db table
#[derive(Debug, Serialize)]
pub struct Feed {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub source_updated_at: Option<DateTime<FixedOffset>>,
    pub feed_type: FeedType,
    pub feed_items: Vec<FeedItem>,
}

/// Represents the `feed_item` db table
#[derive(Debug, Serialize)]
pub struct FeedItem {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub summary: Option<String>,
    #[serde(skip_serializing)]
    pub content: Option<String>,
}

impl TryFrom<FeedSource> for Feed {
    type Error = AppError;

    fn try_from(feed_source: FeedSource) -> Result<Self, Self::Error> {
        match feed_source {
            FeedSource::Atom(atom_feed) => atom_feed.try_into(),
            FeedSource::Rss(rss_channel) => rss_channel.try_into(),
        }
    }
}

impl TryFrom<rss::Channel> for Feed {
    type Error = AppError;

    fn try_from(rss_channel: rss::Channel) -> Result<Self, Self::Error> {
        let source_updated_at = match rss_channel.last_build_date {
            Some(last_build_date) => {
                let result = last_build_date.parse::<DateTime<FixedOffset>>();
                result.ok()
            }
            None => None,
        };

        let feed_items= rss_channel
            .items
            .iter()
            .cloned()
            .filter_map(|item| item.try_into().ok())
            .collect();

        Ok(Feed {
            id: Uuid::new_v4(),
            title: rss_channel.title,
            url: rss_channel.link,
            source_updated_at,
            feed_type: FeedType::Rss,
            feed_items,
        })
    }
}

impl TryFrom<atom_syndication::Feed> for Feed {
    type Error = AppError;

    fn try_from(atom_feed: atom_syndication::Feed) -> Result<Self, Self::Error> {
        // required field
        let link = atom_feed.links.get(0).ok_or(AppError::ParseFeedError(
            "Unable to find a link in an atom entry".to_owned(),
        ))?;

        let feed_items= atom_feed
            .entries
            .iter()
            .cloned()
            .filter_map(|entry| entry.try_into().ok())
            .collect();

        Ok(Feed {
            id: Uuid::new_v4(),
            title: atom_feed.title.value,
            url: link.href.to_owned(),
            source_updated_at: Some(atom_feed.updated),
            feed_type: FeedType::Atom,
            feed_items
        })
    }
}

impl TryFrom<rss::Item> for FeedItem {
    type Error = AppError;

    fn try_from(rss_item: rss::Item) -> Result<Self, Self::Error> {
        let title = rss_item.title.ok_or(AppError::ParseFeedError(
            "Rss item doesn't have a title".to_owned(),
        ))?;
        let url = rss_item.link.ok_or(AppError::ParseFeedError(
            "Rss item doesn't have a url".to_owned(),
        ))?;

        Ok(FeedItem {
            id: Uuid::new_v4(),
            title,
            url,
            summary: rss_item.description,
            content: rss_item.content,
        })
    }
}

impl TryFrom<atom_syndication::Entry> for FeedItem {
    type Error = AppError;

    fn try_from(atom_entry: atom_syndication::Entry) -> Result<Self, Self::Error> {
        // required field
        let link = atom_entry.links.get(0).ok_or(AppError::ParseFeedError(
            "Unable to find a link in an atom entry".to_owned(),
        ))?;

        // optional fields
        let summary = atom_entry.summary.map(|summary| summary.value);
        let content = atom_entry.content.and_then(|content| content.value);

        Ok(FeedItem {
            id: Uuid::new_v4(),
            title: atom_entry.title.value,
            url: link.href.to_owned(),
            summary,
            content,
        })
    }
}
