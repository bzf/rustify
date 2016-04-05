extern crate libc;

use spotify::types;

#[derive(Debug)]
#[allow(dead_code)]
#[repr(C)]
pub enum SpLinkType {
  SpLinktypeInvalid = 0,
  SpLinktypeTrack = 1,
  SpLinktypeAlbum = 2,
  SpLinktypeArtist = 3,
  SpLinktypeSearch = 4,
  SpLinktypePlaylist = 5,
  SpLinktypeProfile = 6,
  SpLinktypeStarred = 7,
  SpLinktypeLocaltrack = 8,
  SpLinktypeImage = 9,
}

pub enum SpLink { }

#[link(name = "spotify")]
extern {
  pub fn sp_link_create_from_string(link: *const libc::c_char) -> *const SpLink;

  pub fn sp_link_type(link: *const SpLink) -> SpLinkType;

  pub fn sp_link_as_track(link: *const SpLink) -> *const types::SpTrack;
}
