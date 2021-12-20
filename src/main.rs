use std::error::Error;

mod api;
mod data_structure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let API_ENDPOINT = dotenv::var("API_ENDPOINT")?;
    let user = dotenv::var("SUBSONIC_USER")?;
    let password = dotenv::var("SUBSONIC_PASSWORD")?;
    let subsonic_client =
        crate::api::subsonic_client::SubsonicClient::new(&API_ENDPOINT, &user, &password);
    if let Some(artists) = subsonic_client.get_artists().await? {
        println!("{:#?}", artists);
    };
    if let Some(genres) = subsonic_client.get_genres().await? {
            println!("{:#?}", genres);
    };
    // if let Some(directory) = subsonic_client.get_directory().await? {
    //         println!("{:#?}",directory);
    // };
    if let Some(music_folders) = subsonic_client.get_music_folders().await? {
            println!("{:#?}",music_folders);
    };
    if let Some(indexes) = subsonic_client.get_indexes().await? {
            println!("{:#?}", indexes);
    };
    Ok(())
}
