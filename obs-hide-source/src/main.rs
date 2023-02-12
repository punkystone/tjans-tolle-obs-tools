#![windows_subsystem = "windows"]
use std::{env, error::Error};

use obws::{
    requests::scene_items::{Id, SetEnabled},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let scene = &args[1];
    let source = &args[2];
    let password = &args[3];
    let client = Client::connect("localhost", 4455, Some(password)).await?;
    let source_id = client
        .scene_items()
        .id(Id {
            scene: &scene,
            source,
            search_offset: None,
        })
        .await?;

    client
        .scene_items()
        .set_enabled(SetEnabled {
            scene: &scene,
            item_id: source_id,
            enabled: !client.scene_items().enabled(scene, source_id).await?,
        })
        .await?;

    Ok(())
}
