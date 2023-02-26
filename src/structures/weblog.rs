use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeblogEntriesResponse {
    message: String,
    entries: Vec<WeblogEntry>,
}

#[derive(Deserialize, Debug)]
pub struct WeblogEntryResponse {
    message: String,
    entry: WeblogEntry,
}

#[derive(Deserialize, Debug)]
pub struct WeblogEntry {
    location: String,
    title: String,
    date: u32,
    status: String,
    body: String,
    source: String,
    metadata: WeblogMetadata,
    output: String,
    entry: String,

    #[serde(rename(deserialize = "type"))]
    entry_type: String,
}

#[derive(Deserialize, Debug)]
pub struct WeblogMetadata {
    date: String,
    slug: String,
}

#[derive(Deserialize, Debug)]
pub struct WeblogConfigurationResponse {
    message: String,
    configuration: WeblogConfigurationFormats,
}

#[derive(Deserialize, Debug)]
pub struct WeblogConfigurationFormats {
    object: WeblogConfiguration,
    json: String,
    raw: String,
}

#[derive(Deserialize, Debug)]
pub struct WeblogConfiguration {
    weblog_title: String,
    weblog_description: String,
    author: String,
    separator: String,
    tag_path: String,
    timezone: String,
    date_format: String,
    default_post: String,
    feed_post_count: String,
    recents_posts_format: String,
    post_list_format: String,
    search_satus: String,
    search_status_success_message: String,
    search_results_failure_message: String,
    search_results_format: String,
}

#[derive(Deserialize, Debug)]
pub struct WeblogTemplateResponse {
    message: String,
    template: String,
}
