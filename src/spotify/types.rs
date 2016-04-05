extern crate libc;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender};
use std::ptr;

pub trait MusicPlayer: Sync + Send {
  fn handle_music_delivery(&mut self, &Vec<i32>) -> i32;
}

pub struct CallbackHelper {
  pub player: Arc<Mutex<MusicPlayer>>,
  pub sender: Sender<SpSessionCallback>,
}

pub enum SpSession { }
pub enum SpPlaylistContainer { }
pub enum SpPlaylist { }
pub enum SpTrack { }
pub enum SpAudioformat { }

#[allow(dead_code)]
pub enum SpArtist { }

#[allow(dead_code)]
pub enum SpAlbum { }

#[allow(dead_code)]
pub enum SpAlbumbrowse { }

#[allow(dead_code)]
pub enum SpAudioBufferFormat { }

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
#[repr(C)]
pub enum SpPlaylistType {
  SpPlaylistTypePlaylist,
  SpPlaylistTypeStartFolder,
  SpPlaylistTypeEndFolder,
  SpPlaylistTypePlaceholder,
}

#[derive(Debug)]
#[allow(dead_code)]
#[repr(C)]
pub enum SpConnectionState {
  SpConnectionStateLoggedOut,
  SpConnectionStateLoggedIn,
  SpConnectionStateDisconnected,
  SpConnectionStateUndefined,
  SpConnectionStateOffline,
}

#[derive(Debug)]
pub enum SpSessionCallback {
  LoggedIn,
  LoggedOut,
  MetadataUpdated,
  ConnectionError(SpError),
  NotifyMainThread,
  PlayTokenLost,
  EndOfTrack,
  StreamingError(SpError),
  UserinfoUpdated,
  StartPlayback,
  StopPlayback,
  OfflineStatusUpdated,
  OfflineError(SpError),
  ConnectionstateUpdated,
  ScrobbleError(SpError),
  PrivateSessionModeChanged(bool),
}

pub use spotify::callbacks::{SpSessionCallbacks};

#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum SpError {
  SP_ERROR_OK,
  SP_ERROR_BAD_API_VERSION,
  SP_ERROR_API_INITIALIZATION_FAILED,
  SP_ERROR_TRACK_NOT_PLAYABLE,
  SP_ERROR_BAD_APPLICATION_KEY,
  SP_ERROR_BAD_USERNAME_OR_PASSWORD,
  SP_ERROR_USER_BANNED,
  SP_ERROR_UNABLE_TO_CONTACT_SERVER,
  SP_ERROR_CLIENT_TOO_OLD,
  SP_ERROR_OTHER_PERMANENT,
  SP_ERROR_BAD_USER_AGENT,
  SP_ERROR_MISSING_CALLBACK,
  SP_ERROR_INVALID_INDATA,
  SP_ERROR_INDEX_OUT_OF_RANGE,
  SP_ERROR_USER_NEEDS_PREMIUM,
  SP_ERROR_OTHER_TRANSIENT,
  SP_ERROR_IS_LOADING,
  SP_ERROR_NO_STREAM_AVAILABLE,
  SP_ERROR_PERMISSION_DENIED,
  SP_ERROR_INBOX_IS_FULL,
  SP_ERROR_NO_CACHE,
  SP_ERROR_NO_SUCH_USER,
  SP_ERROR_NO_CREDENTIALS,
  SP_ERROR_NETWORK_DISABLED,
  SP_ERROR_INVALID_DEVICE_ID,
  SP_ERROR_CANT_OPEN_TRACE_FILE,
  SP_ERROR_APPLICATION_BANNED,
  SP_ERROR_OFFLINE_TOO_MANY_TRACKS,
  SP_ERROR_OFFLINE_DISK_CACHE,
  SP_ERROR_OFFLINE_EXPIRED,
  SP_ERROR_OFFLINE_NOT_ALLOWED,
  SP_ERROR_OFFLINE_LICENSE_LOST,
  SP_ERROR_OFFLINE_LICENSE_ERROR,
  SP_ERROR_LASTFM_AUTH_ERROR,
  SP_ERROR_INVALID_ARGUMENT,
  SP_ERROR_SYSTEM_FAILURE,
}

#[repr(C)]
#[derive(Debug)]
pub struct SpSessionConfig {
  api_version: libc::c_int,
  pub cache_location: *const libc::c_char,
  pub settings_location: *const libc::c_char,
  pub application_key: *const libc::c_char,
  pub application_key_size: libc::c_int,
  pub user_agent: *const libc::c_char,

  pub callbacks: *const SpSessionCallbacks,
  pub userdata: *const libc::c_void,

  compress_playlists: bool,
  dont_save_metadata_for_playlists: bool,
  initially_unload_playlists: bool,
  device_id: *const libc::c_char,
  proxy: *const libc::c_char,
  proxy_username: *const libc::c_char,
  ca_certs_filename: *const libc::c_char,
  tracefile: *const libc::c_char,
}

impl Default for SpSessionConfig {
  fn default() -> SpSessionConfig {
    SpSessionConfig {
      api_version: 12,
      cache_location: ptr::null(),
      settings_location: ptr::null(),
      application_key: ptr::null(),
      application_key_size: 0,
      user_agent: ptr::null(),

      callbacks: ptr::null(),
      userdata: ptr::null(),

      compress_playlists: false,
      dont_save_metadata_for_playlists: false,
      initially_unload_playlists: false,
      device_id: ptr::null(),
      proxy: ptr::null(),
      proxy_username: ptr::null(),
      tracefile: ptr::null(),
      ca_certs_filename: ptr::null(),
    }
  }
}
