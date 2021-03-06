use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::ctx::player::player_entity::Player;
use crate::app::modules::track::external_track_view::ExternalTrackView;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::modules::track::track_view::TrackView;
use crate::app::routing::Commands;
use crate::infra::next::Next;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

pub struct TrackService {}

impl TrackService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn play_track(&self, player: &mut Player, track: &TrackEntity) {
        player.play_track(track.clone()).await
    }

    pub async fn test_download(&self, url: &String) -> Result<(), Box<dyn Error>> {
        let path = "./assets/download.mp3";
        let client = reqwest::Client::new();

        let res = client
            .get(url)
            .send()
            .await
            .or(Err(format!("Failed to GET from '{}'", &url)))?;
        let total_size = res
            .content_length()
            .ok_or(format!("Failed to get content length from '{}'", &url))?;

        // Indicatif setup
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
            .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", url));

        // download chunks
        let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.or(Err(format!("Error while downloading file")))?;
            file.write_all(&chunk)
                .or(Err(format!("Error while writing to file")))?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        pb.finish_with_message(format!("Downloaded {} to {}", url, path));
        Ok(())
    }

    pub async fn download_track(&self, track: &TrackEntity) -> bool {
        self.test_download(track.get_path()).await.unwrap();
        // let path = Path::new("./assets/download.mp3");
        // let mut file = match File::create(&path) {
        //     Err(why) => panic!("couldn't create {}", why),
        //     Ok(file) => file,
        // };
        // let response = reqwest::get(track.get_path()).await.unwrap();
        // let mut content = Cursor::new(response.bytes().await.unwrap());
        // std::io::copy(&mut content, &mut file).unwrap();
        true
    }
}
