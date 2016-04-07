#![deny(warnings)]

extern crate libc;
extern crate openal;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ffi;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

mod player;
mod link;
mod track;
mod playlist;

pub use self::player::{OpenALPlayer};
pub use self::link::Link;
pub use self::track::Track;
pub use self::playlist::Playlist;

mod spotify;
pub use spotify::MusicPlayer;

#[derive(Debug)]
struct SessionPtr(*const spotify::SpSession);
unsafe impl Send for SessionPtr { }
unsafe impl Sync for SessionPtr { }

#[derive(Debug)]
pub enum Event {
  LoggedIn,
  EndOfTrack,
}

pub struct Session {
  #[allow(dead_code)]
  callbacks: spotify::SpSessionCallbacks,

  session: Arc<SessionPtr>,

  #[allow(dead_code)]
  callback_helper: *const spotify::CallbackHelper,
}

impl Session {
  pub fn new(application_key: Vec<u8>,
             cache_location: &str,
             settings_location: &str,
             user_agent: &str,
             player: Arc<Mutex<MusicPlayer>>) -> (Session, Receiver<Event>) {
    let (sp_sender, sp_receiver): (Sender<spotify::SpSessionCallback>, Receiver<spotify::SpSessionCallback>) = mpsc::channel();

    let callbacks = spotify::SpSessionCallbacks::default();
    let callbacks_ptr: *const spotify::SpSessionCallbacks = &callbacks;
    let mut config = spotify::SpSessionConfig::default();

    config.application_key = application_key.as_ptr() as *const i8;
    config.application_key_size = application_key.len() as i32;
    config.cache_location = ffi::CString::new(cache_location).unwrap().into_raw();
    config.settings_location = ffi::CString::new(settings_location).unwrap().into_raw();
    config.user_agent = ffi::CString::new(user_agent).unwrap().as_ptr();
    config.callbacks = callbacks_ptr;

    let foo = spotify::CallbackHelper {
      sender: sp_sender,
      player: player.clone(),
    };

    let callback_helper: *const spotify::CallbackHelper = Box::into_raw(Box::new(foo));
    config.userdata = callback_helper as *const libc::c_void;

    let session = unsafe {
      let mut session = std::ptr::null_mut() as *mut spotify::SpSession;
      spotify::sp_session_create(&config, &mut session);
      session
    } as *const spotify::SpSession;

    // Open a channel for returning event from the API
    let (rustify_sender, rustify_receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    let mut session = Session {
      callbacks: callbacks,
      session: Arc::new(SessionPtr(session)),
      callback_helper: callback_helper,
    };

    // Start a worker thread for doing all the `sp_session_process_event` stuff
    session.start_channel_thread(sp_receiver, rustify_sender);

    return (session, rustify_receiver);
  }

  pub fn playlist(&self, index: i32) -> Option<Playlist> {
    let container = self.playlist_container();
    let number_of_playlists = unsafe {
      spotify::sp_playlistcontainer_num_playlists(container)
    };

    if index >= number_of_playlists {
      return None;
    }

    let playlist_ptr = unsafe { spotify::sp_playlistcontainer_playlist(container, index) };
    return Some(Playlist::new(playlist_ptr));
  }

  pub fn play_track(&self, track: &Track) -> bool {
    // sp_error sp_session_player_load(sp_session *session, sp_track *track)
    unsafe { spotify::sp_session_player_load(self.session.0, track.ptr()) };

    // sp_error sp_session_player_play(sp_session *session, bool play)
    unsafe { spotify::sp_session_player_play(self.session.0, true) };

    return true;
  }

  pub fn login(&mut self, username: &String, password: &String) {
    let c_username = ffi::CString::new(username.as_bytes()).unwrap();
    let c_password = ffi::CString::new(password.as_bytes()).unwrap();

    unsafe { spotify::sp_session_login(self.session.0,
                                       c_username.as_ptr(),
                                       c_password.as_ptr(),
                                       false,
                                       std::ptr::null());
    };

    thread::sleep(Duration::from_millis(2000));
  }

  fn start_channel_thread(&mut self,
                          receiver: Receiver<spotify::SpSessionCallback>,
                          sender: Sender<Event>) {
    let session_ptr = self.session.clone();

    thread::spawn(move || {
      let mut next_timeout = 0 as i32;

      loop {
        let event = receiver.recv().unwrap();

        match event {
          spotify::SpSessionCallback::LoggedIn => sender.send(Event::LoggedIn).unwrap(),
          spotify::SpSessionCallback::EndOfTrack => sender.send(Event::EndOfTrack).unwrap(),
          spotify::SpSessionCallback::NotifyMainThread => unsafe {
            spotify::sp_session_process_events(session_ptr.0, &mut next_timeout);
          },
          _ => (),
        }
      }
    });
  }

  fn playlist_container(&self) -> *const spotify::SpPlaylistContainer {
    let playlist_container = unsafe {
      spotify::sp_session_playlistcontainer(self.session.0)
    };

    loop {
      if unsafe { spotify::sp_playlistcontainer_is_loaded(playlist_container) } {
        break;
      }

      thread::sleep(Duration::from_millis(100));
    }

    return playlist_container;
  }
}
