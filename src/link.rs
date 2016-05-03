extern crate libc;

use std;

use spotify;
use track::{Track};

pub enum Link {
  TrackLink(Track),
  Invalid,
}

impl Link {
  pub fn new(link: &str) -> Link {
    let copy = std::ffi::CString::new(link).unwrap();
    let c_ptr = copy.into_raw();
    let link_ptr = unsafe { spotify::sp_link_create_from_string(c_ptr as *const libc::c_char) };

    if link_ptr.is_null() {
      return Link::Invalid;
    }

    let link_type = unsafe { spotify::sp_link_type(link_ptr) };

    match link_type {
      spotify::SpLinkType::SpLinktypeTrack => {
        let track_ptr = unsafe { spotify::sp_link_as_track(link_ptr) };
        return Link::TrackLink(Track::new(track_ptr));
      },
      _ => Link::Invalid,
    }
  }
}
