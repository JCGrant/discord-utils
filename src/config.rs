use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub images_folder: String,
    // Uploader
    pub art_webhook_url: String,
    pub animation_webhook_url: String,
    // Downloader
    pub discord_token: String,
    pub discord_channel_id: u64,
    pub discord_after_message_id: u64,
}

pub fn load() -> Config {
    dotenvy::dotenv().ok();
    envy::from_env::<Config>().unwrap()
}
