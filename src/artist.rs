use std;
use spotify;

pub struct Artist {
  ptr: *const spotify::SpArtist,
}

impl Artist {
  pub fn new(ptr: *const spotify::SpArtist) -> Self {
    return Artist {
      ptr: ptr,
    };
  }

  pub fn name(&self) -> String {
    self.wait_until_loaded();

    let name = unsafe {
      std::ffi::CStr::from_ptr(spotify::sp_artist_name(self.ptr) as *mut i8)
    };

    return match name.to_str() {
      Ok(name) => String::from(name),
      Err(_) => String::from("Loading..."),
    }
  }

  fn wait_until_loaded(&self) {
    loop {
      if unsafe { spotify::sp_artist_is_loaded(self.ptr) } { break; }
      std::thread::sleep(std::time::Duration::from_millis(100));
    }
  }
}
