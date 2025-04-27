mod config;
mod download;
mod upload;

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        std::process::exit(1);
    }
    let command = &args[1];

    // Load environment variables
    let config = config::load();

    // Execute the command
    match command.as_str() {
        "upload" => {
            upload::upload(
                config.images_folder,
                config.art_webhook_url,
                config.animation_webhook_url,
            )
            .await;
        }
        "download" => {
            download::download(
                config.images_folder,
                config.discord_token,
                config.discord_channel_id,
                config.discord_after_message_id,
            )
            .await;
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
