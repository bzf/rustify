use std;
use std::fmt;
use spotify;
use artist::Artist;

pub struct Track {
  ptr: *const spotify::SpTrack,
}

impl Track {
  pub fn new(ptr: *const spotify::SpTrack) -> Track {
    unsafe { spotify::sp_track_add_ref(ptr) };
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
      std::ffi::CStr::from_ptr(spotify::sp_track_name(self.ptr) as *mut i8)
    };

    return match name.to_str() {
      Ok(name) => String::from(name),
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

impl std::fmt::Display for Track {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let names: Vec<String> = self.artists().iter().map(|x| x.name()).collect();
    write!(f, "{} - {}", self.name(), names.join(", ").clone())
  }
}

impl Clone for Track {
  fn clone(&self) -> Self {
    unsafe { spotify::sp_track_add_ref(self.ptr) };
    return Track::new(self.ptr);
  }
}

impl Drop for Track {
  fn drop(&mut self) {
    unsafe { spotify::sp_track_release(self.ptr) };
  }
}
