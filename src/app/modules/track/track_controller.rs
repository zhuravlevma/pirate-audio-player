use crate::app::command::home_command::HomeCommand;
use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::ctx::Ctx;
use crate::app::modules::track::external_track_view::ExternalTrackView;
use crate::app::modules::track::track_entity::TrackEntity;
use crate::app::modules::track::track_view::TrackView;
use crate::app::routing::Commands;
use crate::infra::next::Next;

pub struct TrackController {}

impl TrackController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_playing_track(&self, _request: Next, ctx: &Ctx) -> Next {
        match ctx.player.get_current_track() {
            None => TrackView::not_found(),
            Some(track) => match track.is_external {
                true => match ctx.player.pause_time {
                    None => ExternalTrackView::get_track_with_header(
                        track.get_path(),
                        ctx.player.get_time(),
                    ),
                    Some(_) => {
                        ExternalTrackView::get_pause_track(track.get_path(), ctx.player.get_time())
                    }
                },
                false => match ctx.player.pause_time {
                    None => {
                        TrackView::get_track_with_header(track.get_path(), ctx.player.get_time())
                    }
                    Some(_) => TrackView::get_pause_track(track.get_path(), ctx.player.get_time()),
                },
            },
        }
    }

    pub async fn play_track(&self, ctx: &mut Ctx, track: TrackEntity) -> Next {
        ctx.player
            .play_track(TrackEntity::new(track.get_path().to_string(), false))
            .await;
        Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack), None)
    }

    pub fn pause(&self, _request: Next, ctx: &mut Ctx) -> Next {
        ctx.player.pause();
        Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack), None)
    }

    pub fn track_continue(&self, _request: Next, ctx: &mut Ctx) -> Next {
        ctx.player.play();
        Next::new(Commands::Playlist(PlaylistCommand::GetPlayingTrack), None)
    }
}
