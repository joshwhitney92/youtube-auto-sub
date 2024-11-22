use serde::Deserialize;

/// A model for  a YouTube channel.
#[derive(Clone, Default, Deserialize)]
pub struct YouTubeChannel {
    pub channel_url: String,
    pub channel_id: String,
    pub channel_title: String
}

pub struct FailedYoutubeSubscription {
    pub channel_url: String,
    pub channel_id: String,
    pub error: Box<dyn std::error::Error>
}

/// Represents result of api call to subscribe to youtube channels.
/// # Fields
/// `expected`: The number of channel subscriptions expected.
/// `successful`: The number of successful channel subscriptions.
#[derive(Default)]
pub struct YouTubeSubscriptionResult {
    pub expected: i32,
    pub successful: i32,
    pub failed: Vec<FailedYoutubeSubscription>
}

