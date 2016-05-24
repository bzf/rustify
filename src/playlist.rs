use std;
use std::fmt;
use spotify;
use track::Track;

pub struct Playlist {
  ptr: *const spotify::SpPlaylist,
}

impl Playlist {
  pub fn new(ptr: *const spotify::SpPlaylist) -> Playlist {
    loop {
      if unsafe { spotify::sp_playlist_is_loaded(ptr) } { break; }
      std::thread::sleep(std::time::Duration::from_millis(100));
    }

    unsafe { spotify::sp_playlist_add_ref(ptr) };
    return Playlist { ptr: ptr };
  }

  pub fn ptr(&self) -> *const spotify::SpPlaylist {
    return self.ptr;
  }

  pub fn tracks(&self) -> Vec<Track> {
    self.wait_until_loaded();

    // Load all the playlists
    let number_of_tracks = unsafe { spotify::sp_playlist_num_tracks(self.ptr) };
    let mut tracks: Vec<Track> = Vec::new();
    tracks.reserve(number_of_tracks as usize);

    for i in 0..number_of_tracks {
      let track_ptr = unsafe { spotify::sp_playlist_track(self.ptr, i) };
      tracks.push(Track::new(track_ptr));
    }

    return tracks;
  }

  pub fn name(&self) -> String {
    self.wait_until_loaded();

    let name = unsafe {
      std::ffi::CStr::from_ptr(spotify::sp_playlist_name(self.ptr) as *mut i8)
    };

    return match name.to_str() {
      Ok(name) => String::from(name),
      Err(_) => String::from("Loading..."),
    }
  }

  pub fn track(&self, index: i32) -> Option<Track> {
    self.wait_until_loaded();

    let number_of_tracks = unsafe { spotify::sp_playlist_num_tracks(self.ptr) };
    if index >= number_of_tracks {
      return None;
    }

    let track_ptr = unsafe { spotify::sp_playlist_track(self.ptr, index) };
    return Some(Track::new(track_ptr));
  }

  fn wait_until_loaded(&self) {
    loop {
      if unsafe { spotify::sp_playlist_is_loaded(self.ptr) } { break; }
      std::thread::sleep(std::time::Duration::from_millis(100));
    }
  }
}

impl std::fmt::Display for Playlist {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl Clone for Playlist {
  fn clone(&self) -> Self {
    return Playlist::new(self.ptr);
  }
}

impl Drop for Playlist {
  fn drop(&mut self) {
    unsafe { spotify::sp_playlist_release(self.ptr) };
  }
}
