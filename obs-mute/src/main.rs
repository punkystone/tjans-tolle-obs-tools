use std::{env, error::Error};

use obws::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let audio_source = &args[1];
    let password = &args[2];
    let client = Client::connect("localhost", 4455, Some(password)).await?;

    client
        .inputs()
        .set_muted(
            &audio_source,
            !client
                .inputs()
                .muted(&audio_source)
                .await
                .unwrap_or_else(|_| false),
        )
        .await?;
    Ok(())
}
