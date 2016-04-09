extern crate libc;

use spotify::types::{SpSession, SpAudioBufferFormat, SpAudioformat, SpError, SpSessionCallback};
use spotify;

fn send_event(session: *mut SpSession, event: SpSessionCallback) {
  // Fetch the Sender from `sp_session_userdata` and send the OnLoggedIn
  let userdata: *const spotify::CallbackHelper = unsafe { spotify::sp_session_userdata(session) as *const spotify::CallbackHelper };
  let foo: &spotify::CallbackHelper = unsafe { &*userdata };

  foo.sender.send(event).unwrap();
}

extern fn on_logged_in(session: *mut SpSession,
                       _: SpError) {
  send_event(session, SpSessionCallback::LoggedIn);
}

extern fn on_logged_out(session: *mut SpSession) {
  send_event(session, SpSessionCallback::LoggedOut);
}

extern fn on_metadata_updated(session: *mut SpSession) {
  send_event(session, SpSessionCallback::MetadataUpdated);
}

extern fn on_connection_error(session: *mut SpSession,
                              error: SpError) {
  send_event(session, SpSessionCallback::ConnectionError(error));
}

extern fn on_message_to_user(_: *mut SpSession,
                             _: *const libc::c_char) {
}

extern fn on_notify_main_thread(session: *mut SpSession) {
  send_event(session, SpSessionCallback::NotifyMainThread);
}

extern fn on_music_delivery(session: *mut SpSession,
                            _: *const SpAudioformat,
                            frames_ptr: *const libc::c_void,
                            num_frames: i32) -> i32 {
  let mut frames: Vec<i32> = Vec::new();

  for i in 0..num_frames {
    let f = frames_ptr as *const i32;
    let frame = unsafe { *f.offset(i as isize) } as i32;
    frames.push(frame);
  }

  let userdata: *const spotify::CallbackHelper = unsafe { spotify::sp_session_userdata(session) as *const spotify::CallbackHelper };
  let foo: &spotify::CallbackHelper = unsafe { &*userdata };
  return foo.player.lock().unwrap().handle_music_delivery(&frames);
}

extern fn on_play_token_lost(session: *mut SpSession) {
  send_event(session, SpSessionCallback::PlayTokenLost);
}

extern fn on_log_message(_: *mut SpSession,
                         _: *const libc::c_char) {
}

extern fn on_end_of_track(session: *mut SpSession) {
  unsafe { spotify::sp_session_player_unload(session) };

  let userdata: *const spotify::CallbackHelper = unsafe {
    spotify::sp_session_userdata(session) as *const spotify::CallbackHelper
  };
  let cb_helper: &spotify::CallbackHelper = unsafe { &*userdata };
  cb_helper.player.lock().unwrap().reset();

  send_event(session, SpSessionCallback::EndOfTrack);
}

extern fn on_streaming_error(session: *mut SpSession,
                             error: SpError) {
  send_event(session, SpSessionCallback::StreamingError(error));
}

extern fn on_userinfo_updated(session: *mut SpSession) {
  send_event(session, SpSessionCallback::UserinfoUpdated);
}

extern fn on_start_playback(session: *mut SpSession) {
  send_event(session, SpSessionCallback::StartPlayback);
}

extern fn on_stop_playback(session: *mut SpSession) {
  send_event(session, SpSessionCallback::StopPlayback);
}

extern fn on_get_audio_buffer_stats(_: *mut SpSession,
                                        _: *const SpAudioBufferFormat) {
}

extern fn on_offline_status_updated(session: *mut SpSession) {
  send_event(session, SpSessionCallback::OfflineStatusUpdated);
}

extern fn on_offline_error(session: *mut SpSession,
                           error: SpError) {
  send_event(session, SpSessionCallback::OfflineError(error));
}

extern fn on_credentials_blob_updated(_: *mut SpSession,
                                          _: *const libc::c_char) {
}

extern fn on_connectionstate_updated(session: *mut SpSession) {
  send_event(session, SpSessionCallback::ConnectionstateUpdated);
}

extern fn on_scrobble_error(session: *mut SpSession,
                            error: SpError) {
  send_event(session, SpSessionCallback::ScrobbleError(error));
}

extern fn on_private_session_mode_changed(session: *mut SpSession,
                                          value: bool) {
  send_event(session, SpSessionCallback::PrivateSessionModeChanged(value));
}

#[repr(C)]
pub struct SpSessionCallbacks {
  logged_in: extern fn(session: *mut SpSession, error: SpError),
  logged_out: extern fn(session: *mut SpSession),
  metadata_updated: extern fn(session: *mut SpSession),
  connection_error: extern fn(session: *mut SpSession, error: SpError),
  message_to_user: extern fn(session: *mut SpSession, message: *const libc::c_char),
  notify_main_thread: extern fn(session: *mut SpSession),
  music_delivery: extern fn(session: *mut SpSession, format: *const SpAudioformat, frames: *const libc::c_void, num_frames: i32) -> i32,
  play_token_lost: extern fn(session: *mut SpSession),
  log_message: extern fn(session: *mut SpSession, data: *const libc::c_char),
  end_of_track: extern fn(session: *mut SpSession),
  streaming_error: extern fn(session: *mut SpSession, error: SpError),
  userinfo_updated: extern fn(session: *mut SpSession),
  start_playback: extern fn(session: *mut SpSession),
  stop_playback: extern fn(session: *mut SpSession),
  get_audio_buffer_stats: extern fn(session: *mut SpSession, stats: *const SpAudioBufferFormat),
  offline_status_updated: extern fn(session: *mut SpSession),
  offline_error: extern fn(session: *mut SpSession, error: SpError),
  credentials_blob_updated: extern fn(session: *mut SpSession, blob: *const libc::c_char),
  connectionstate_updated: extern fn(session: *mut SpSession),
  scrobble_error: extern fn(session: *mut SpSession, error: SpError),
  private_session_mode_changed: extern fn(session: *mut SpSession, is_private: bool),
}

impl Default for SpSessionCallbacks {
  fn default() -> SpSessionCallbacks {
    SpSessionCallbacks {
      logged_in: on_logged_in,
      logged_out: on_logged_out,
      metadata_updated: on_metadata_updated,
      connection_error: on_connection_error,
      message_to_user: on_message_to_user,
      notify_main_thread: on_notify_main_thread,
      music_delivery: on_music_delivery,
      play_token_lost: on_play_token_lost,
      log_message: on_log_message,
      end_of_track: on_end_of_track,
      streaming_error: on_streaming_error,
      userinfo_updated: on_userinfo_updated,
      start_playback: on_start_playback,
      stop_playback: on_stop_playback,
      get_audio_buffer_stats: on_get_audio_buffer_stats,
      offline_status_updated: on_offline_status_updated,
      offline_error: on_offline_error,
      credentials_blob_updated: on_credentials_blob_updated,
      connectionstate_updated: on_connectionstate_updated,
      scrobble_error: on_scrobble_error,
      private_session_mode_changed: on_private_session_mode_changed,
    }
  }
}
