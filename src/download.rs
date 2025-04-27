use serenity::all::{ChannelId, GetMessages, MessageId};
use serenity::http::Http;

pub async fn download(
    images_folder: String,
    token: String,
    channel_id: u64,
    after_message_id: u64,
) {
    let http = Http::new(&token);
    let channel_id = ChannelId::new(channel_id);

    let mut after_message_id: MessageId = MessageId::new(after_message_id);
    let mut num_downloads = 0;
    loop {
        // Fetch messages from the channel
        let messages = channel_id
            .messages(&http, GetMessages::new().after(after_message_id).limit(100))
            .await
            .expect("Failed to fetch messages");
        if messages.is_empty() {
            println!("No more messages to download.");
            break;
        }

        // Process the messages
        for message in &messages {
            for attachment in &message.attachments {
                // Check if the attachment is an image
                if attachment.width.is_none() || attachment.height.is_none() {
                    continue;
                }
                let filepath = format!("{}/{}", images_folder, attachment.filename);
                download_file(&attachment.url, &filepath).await;
                num_downloads += 1;
                println!("Downloaded ({}): {}", num_downloads, filepath);
            }
        }

        // Update the after_message_id to the last message in the current batch.
        // Have to use the first message ID because the messages are sorted in reverse order.
        after_message_id = messages.first().unwrap().id;
    }
}

async fn download_file(url: &str, filepath: &str) {
    let response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    std::fs::write(filepath, &bytes).expect("Unable to write file");
}
