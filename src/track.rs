use std;
use spotify;

pub struct Track {
  ptr: *const spotify::SpTrack,
}

impl Track {
  pub fn new(ptr: *const spotify::SpTrack) -> Track {
    return Track { ptr: ptr };
  }

  pub fn ptr(&self) -> *const spotify::SpTrack {
    return self.ptr;
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
