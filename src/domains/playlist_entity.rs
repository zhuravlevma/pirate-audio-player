use std::fs;
use terminal_menu::TerminalMenu;
use crate::domains::track_entity::TrackEntity;
use crate::views::playlist_view::PlaylistView;

pub struct Playlist {
    current_track: Option<TrackEntity>,
    tracks: Vec<TrackEntity>,
}

impl Playlist {
    pub fn new(path: &str) -> Self {
        let paths = fs::read_dir(path).unwrap();
        let tracks = paths
            .into_iter()
            .map(|path| TrackEntity::new(path.unwrap().path().display().to_string()))
            .collect();
        Self {
            current_track: Option::None,
            tracks
        }
    }

    pub fn get_track_list(&self) -> TerminalMenu {
        match &self.current_track {
            None => {
                PlaylistView::get("", 0, &self.tracks)
            }
            Some(track) => {
                PlaylistView::get(&track.track_path, track.get_start(), &self.tracks)
            }
        }
    }

    pub fn find_track(&self, track_path: &str) -> Option<&TrackEntity> {
        self.tracks.iter().find(|&el| el.track_path == track_path)
    }

    pub fn change_track(&mut self, track_path: String) {
        let track = self.tracks.iter().find(|&el| el.track_path == track_path);
        match track {
            None => {}
            Some(track) => {
                self.current_track = Some(track.clone())
            }
        }
    }
}