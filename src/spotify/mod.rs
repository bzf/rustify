extern crate libc;

mod types;
mod callbacks;
mod links;

pub use spotify::types::{
  CallbackHelper,
  MusicPlayer,
  SpSessionConfig,
  SpSession,
  SpSearch,
  SpSearchType,
  SpError,
  SpAudioBufferFormat,
  SpAudioformat,
  SpSessionCallback,
  SpPlaylist,
  SpPlaylistContainer,
  SpTrack,
  SpPlaylistType,
  SpConnectionState,
  SpArtist,
  SpAlbum,
  SpAlbumbrowse,
};

pub use spotify::callbacks::{SpSessionCallbacks, on_search_completed};
pub use spotify::links::{SpLink, SpLinkType, sp_link_create_from_string, sp_link_type, sp_link_as_track};

#[link(name = "spotify")]
extern {
  pub fn sp_session_create(config: *const SpSessionConfig,
                           session: *mut *mut SpSession) -> SpError;

  pub fn sp_session_login(session: *const SpSession,
                          username: *const libc::c_char,
                          password: *const libc::c_char,
                          remember_me: bool,
                          blob: *const libc::c_char) -> SpError;


  pub fn sp_session_playlistcontainer(session: *const SpSession) -> *const SpPlaylistContainer;

  pub fn sp_session_process_events(session: *const SpSession, next_timeout: &mut i32) -> SpError;

  pub fn sp_playlistcontainer_num_playlists(container: *const SpPlaylistContainer) -> i32;

  pub fn sp_playlistcontainer_is_loaded(container: *const SpPlaylistContainer) -> bool;

  pub fn sp_playlistcontainer_playlist_type(container: *const SpPlaylistContainer, index: i32) -> SpPlaylistType;

  pub fn sp_playlistcontainer_playlist(container: *const SpPlaylistContainer,
                                       index: i32) -> *const SpPlaylist;

  pub fn sp_playlist_name(playlist: *const SpPlaylist) -> *const libc::c_char;

  pub fn sp_artist_name(artist: *const SpArtist) -> *const libc::c_char;

  pub fn sp_artist_is_loaded(artist: *const SpArtist) -> bool;

  pub fn sp_playlist_is_loaded(playlist: *const SpPlaylist) -> bool;

  pub fn sp_track_is_loaded(track: *const SpTrack) -> bool;

  pub fn sp_playlist_track(playlist: *const SpPlaylist,
                           index: i32) -> *const SpTrack;

  pub fn sp_playlist_add_ref(playlist: *const SpPlaylist) -> SpError;

  pub fn sp_playlist_release(playlist: *const SpPlaylist) -> SpError;

  pub fn sp_session_player_load(session: *const SpSession,
                                track: *const SpTrack) -> SpError;

  pub fn sp_session_player_unload(session: *const SpSession);

  pub fn sp_session_player_play(session: *const SpSession,
                                play: bool) -> SpError;

  pub fn sp_playlist_num_tracks(playlist: *const SpPlaylist) -> i32;

  pub fn sp_track_name(track: *const SpTrack) -> *const libc::c_char;

  pub fn sp_session_userdata(session: *const SpSession) -> *mut libc::c_void;

  pub fn sp_track_num_artists(track: *const SpTrack) -> i32;

  pub fn sp_track_artist(track: *const SpTrack, index: i32) -> *const SpArtist;

  pub fn sp_track_duration(track: *const SpTrack) -> i32;

  pub fn sp_track_add_ref(sp_track: *const SpTrack) -> SpError;

  pub fn sp_track_release(sp_track: *const SpTrack) -> SpError;

  pub fn sp_search_create(session: *const SpSession,
                          query: *const libc::c_char,
                          track_offset: i32,
                          track_count: i32,
                          album_offset: i32,
                          album_count: i32,
                          artist_offset: i32,
                          artist_count: i32,
                          playlist_offset: i32,
                          playlist_count: i32,
                          search_type: SpSearchType,
                          callback: extern fn(*const SpSearch, *const libc::c_void),
                          userdata: *const libc::c_void) -> *const SpSearch;

  pub fn sp_search_is_loaded(search: *const SpSearch) -> bool;

  pub fn sp_search_add_ref(search: *const SpSearch) -> SpError;

  pub fn sp_search_release(search: *const SpSearch) -> SpError;

  pub fn sp_search_num_tracks(search: *const SpSearch) -> i32;

  pub fn sp_search_track(search: *const SpSearch, index: i32) -> *const SpTrack;
}
