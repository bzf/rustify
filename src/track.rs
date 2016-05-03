use std;
use spotify;
use artist::Artist;

pub struct Track {
  ptr: *const spotify::SpTrack,
}

impl Track {
  pub fn new(ptr: *const spotify::SpTrack) -> Track {
    return Track { ptr: ptr };
  }

  pub fn ptr(&self) -> *const spotify::SpTrack {
    self.wait_until_loaded();

    return self.ptr;
  }

  pub fn artists(&self) -> Vec<Artist> {
    self.wait_until_loaded();

    let mut vector = Vec::new();
    let number_of_artists = unsafe {
      spotify::sp_track_num_artists(self.ptr)
    };

    for i in 0..number_of_artists {
      let ptr = unsafe {
        spotify::sp_track_artist(self.ptr, i)
      };

      vector.push(Artist::new(ptr));
    }

    return vector;
  }

  pub fn duration(&self) -> std::time::Duration {
    self.wait_until_loaded();

    let duration = unsafe { spotify::sp_track_duration(self.ptr) };

    return std::time::Duration::from_millis(duration as u64);
  }

  pub fn name(&self) -> String {
    self.wait_until_loaded();

    let name = unsafe {
      std::ffi::CString::from_raw(spotify::sp_track_name(self.ptr) as *mut i8)
    };

    return match std::ffi::CString::into_string(name) {
      Ok(name) => name,
      Err(_) => String::from("Loading..."),
    }
  }

  fn wait_until_loaded(&self) {
    loop {
      if unsafe { spotify::sp_track_is_loaded(self.ptr) } { break; }
      std::thread::sleep(std::time::Duration::from_millis(100));
    }
  }
}
