use spotify;
use track::Track;

#[derive(Debug)]
pub struct Search {
  ptr: *const spotify::SpSearch,
}

impl Search {
  pub fn new(ptr: *const spotify::SpSearch) -> Search {
    unsafe { spotify::sp_search_add_ref(ptr) };
    return Search { ptr: ptr };
  }

  pub fn tracks(&self) -> Vec<Track> {
    let mut tracks = Vec::new();
    let number_of_tracks = unsafe { spotify::sp_search_num_tracks(self.ptr) };
    tracks.reserve(number_of_tracks as usize);

    for index in 0..number_of_tracks {
      let track_ptr = unsafe { spotify::sp_search_track(self.ptr, index) };
      tracks.push(Track::new(track_ptr));
    }

    return tracks;
  }
}

impl Clone for Search {
  fn clone(&self) -> Search {
    unsafe { spotify::sp_search_add_ref(self.ptr) };
    return Search::new(self.ptr);
  }
}

impl Drop for Search {
  fn drop(&mut self) {
    unsafe { spotify::sp_search_release(self.ptr) };
  }
}
