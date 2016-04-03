extern crate libc;

use spotify;
use spotify::types;

#[derive(Debug)]
pub enum SpLinkType {
  SP_LINKTYPE_INVALID = 0,
  SP_LINKTYPE_TRACK = 1,
  SP_LINKTYPE_ALBUM = 2,
  SP_LINKTYPE_ARTIST = 3,
  SP_LINKTYPE_SEARCH = 4,
  SP_LINKTYPE_PLAYLIST = 5,
  SP_LINKTYPE_PROFILE = 6,
  SP_LINKTYPE_STARRED = 7,
  SP_LINKTYPE_LOCALTRACK = 8,
  SP_LINKTYPE_IMAGE = 9,
}

pub enum SpLink { }

#[link(name = "spotify")]
extern {
  pub fn sp_link_create_from_string(link: *const libc::c_char) -> *const SpLink;

  pub fn sp_link_type(link: *const SpLink) -> SpLinkType;

  pub fn sp_link_as_track(link: *const SpLink) -> *const types::SpTrack;

  pub fn sp_link_as_album(link: *const SpLink) -> *const types::SpAlbum;

  pub fn sp_link_as_artist(link: *const SpLink) -> *const types::SpArtist;

  pub fn sp_link_relase(link: *const SpLink) -> types::SpError;
}
