use spotify;
use track::{Track};

pub enum Link {
  TrackLink(Track),
  Invalid,
}

impl Link {
  pub fn new(link: String) -> Link {
    let link_ptr = unsafe { spotify::sp_link_create_from_string(link.as_ptr() as *const i8) };

    if link_ptr.is_null() {
      return Link::Invalid;
    }

    let link_type = unsafe { spotify::sp_link_type(link_ptr) };

    match link_type {
      spotify::SpLinkType::SP_LINKTYPE_TRACK => {
        let track_ptr = unsafe { spotify::sp_link_as_track(link_ptr) };
        return Link::TrackLink(Track::new(track_ptr));
      },
      _ => Link::Invalid,
    }
  }
}
