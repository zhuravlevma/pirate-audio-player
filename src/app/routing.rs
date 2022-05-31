use crate::app::ctx::Ctx;
use crate::infra::next::Next;
use crate::modules::external::muzati::Muzati;
use crate::modules::main::main_controller::MainController;
use crate::modules::playlist::playlist_controller::PlaylistController;
use crate::modules::playlist::playlist_entity::Playlist;
use crate::modules::playlist::playlist_repository::PlaylistRepository;
use crate::modules::track::track_controller::TrackController;

pub struct Routing {
    playlist_controller: PlaylistController,
    track_controller: TrackController,
    main_controller: MainController,
}

#[derive(Clone)]
pub enum Commands {
    GetMainMenu,
    Exit,
    GetPlaylist,
    BackToMain,
    ShowTrack,
    BackToPlaylist,
    PlayTrack,
    Pause,
    Continue,
}

impl Routing {
    pub fn new() -> Self {
        Self {
            playlist_controller: PlaylistController::new(Playlist::new(
                PlaylistRepository::new("./assets", Muzati::new()),
            )),
            track_controller: TrackController::new(),
            main_controller: MainController::new(),
        }
    }

    pub fn routes(&mut self, request: Next, ctx: &mut Ctx) -> Next {
        match request.command {
            Commands::GetMainMenu => self.main_controller.show_menu(request, ctx),
            Commands::Exit => self.main_controller.exit(request, ctx),
            Commands::GetPlaylist => self.playlist_controller.get_track_list(request, ctx),
            Commands::BackToMain => self.playlist_controller.back(request, ctx),
            Commands::ShowTrack => self.track_controller.get_current_track(request, ctx),
            Commands::BackToPlaylist => self.track_controller.back(request, ctx),
            Commands::PlayTrack => self.track_controller.play_track(request, ctx),
            Commands::Pause => self.track_controller.pause(request, ctx),
            Commands::Continue => self.track_controller.track_continue(request, ctx),
        }
    }
}
