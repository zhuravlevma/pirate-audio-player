use crate::app::command::home_command::HomeCommand;
use crate::app::command::playlist_command::PlaylistCommand;
use crate::app::command::track_command::TrackCommand;
use crate::app::ctx::Ctx;
use crate::app::modules::home::home_controller::HomeController;
use crate::app::modules::home::home_serivce::HomeService;
use crate::app::modules::home::home_view::HomeView;
use crate::app::modules::playlist::external::muzati::Muzati;
use crate::app::modules::playlist::playlist_controller::PlaylistController;
use crate::app::modules::playlist::playlist_repository::PlaylistRepository;
use crate::app::modules::playlist::playlist_service::Playlist;
use crate::app::modules::track::external_track_view::ExternalTrackView;
use crate::app::modules::track::track_controller::TrackController;
use crate::app::modules::track::track_service::TrackService;
use crate::app::modules::track::track_view::TrackView;
use crate::infra::next::Next;
use crate::utils::menu_factory::MenuFactory;
use std::error::Error;

pub struct Routing {
    playlist_controller: PlaylistController,
    track_controller: TrackController,
    main_controller: HomeController,
}

#[derive(Clone)]
pub enum Commands {
    MainMenu(HomeCommand),
    Playlist(PlaylistCommand),
    Track(TrackCommand),
    NotFound,
}

impl Routing {
    pub fn new() -> Self {
        Self {
            playlist_controller: PlaylistController::new(Playlist::new(PlaylistRepository::new(
                "./assets",
                Muzati::new(),
            ))),
            track_controller: TrackController::new(
                TrackService::new(),
                TrackView::new(MenuFactory::new()),
                ExternalTrackView::new(MenuFactory::new()),
            ),
            main_controller: HomeController::new(HomeService::new(), HomeView::new()),
        }
    }

    pub async fn routes(&mut self, request: Next, ctx: &mut Ctx) -> Result<Next, Box<dyn Error>> {
        Ok(match request.command {
            Commands::MainMenu(HomeCommand::GetMenu) => {
                self.main_controller.show_menu(request, ctx)
            }
            Commands::MainMenu(HomeCommand::Exit) => self.main_controller.exit(request, ctx),
            Commands::MainMenu(HomeCommand::GetLocalPlaylist) => {
                self.playlist_controller.get_track_list(request, ctx)
            }
            Commands::Playlist(PlaylistCommand::GetPlayingTrack) => {
                self.track_controller.get_playing_track(request, ctx)
            }
            Commands::Playlist(PlaylistCommand::Input(track)) => {
                self.playlist_controller.input(ctx, track)
            }
            Commands::MainMenu(HomeCommand::GetNewPlaylist) => {
                self.playlist_controller
                    .get_new_playlist(request, ctx)
                    .await?
            }
            Commands::MainMenu(HomeCommand::GetPopularPlaylist) => {
                self.playlist_controller
                    .get_popular_playlist(request, ctx)
                    .await?
            }
            Commands::Track(TrackCommand::PlayTrack(track)) => {
                self.track_controller.play_track(ctx, track).await
            }
            Commands::Track(TrackCommand::Refresh) => {
                self.track_controller.get_playing_track(request, ctx)
            }
            Commands::Track(TrackCommand::Download) => {
                self.track_controller.download(request, ctx).await
            }
            Commands::Track(TrackCommand::Pause) => self.track_controller.pause(request, ctx),
            Commands::Track(TrackCommand::Continue) => {
                self.track_controller.track_continue(request, ctx)
            }
            Commands::NotFound => Next::new(Commands::NotFound),
        })
    }
}
