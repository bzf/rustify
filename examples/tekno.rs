// Signs in with the given Spotify crendetials and plays the track 'Den
// modernitet som aldrig kom' from the album 'Tekno' by 1900
// (spotify:album:34pLp9NMX5SOZjx5A2BOba).

extern crate rustify;

mod application_key;

fn main() {
  let args: Vec<_> = std::env::args().collect();
  if args.len() < 3 {
    println!("Usage: ./tekno USERNAME PASSWORD");
    return;
  }

  let username = args[1].to_string();
  let password = args[2].to_string();

  use std::sync::{Arc, Condvar, Mutex};

  // Use OpenAL for playback
  let openal_player = rustify::OpenALPlayer::new();
  let mut player = Arc::new(Mutex::new(openal_player));

  let (mut session, receiver) = rustify::Session::new(
    application_key::get(),
    "tmp/",
    "tmp/",
    "tekno-example",
    player,
  );

  session.login(&username, &password);

  let track = rustify::Link::new(
    String::from("spotify:track:79ORARO8rXmk1ap0sfMPyC")
  );

  match track {
    rustify::Link::TrackLink(track) => {
      println!("[Tekno] Got the track '{}'", track.name());
      session.play_track(&track);
      ();
    },
    _ => {
      panic!("[Tekno] That link is not a track...");
    }
  }

  // Wait for the `EndOfTrack` event to end the program
  loop {
    let event = receiver.recv().unwrap();

    match event {
      rustify::Event::LoggedIn => {
        println!("[Tekno] Logged in!");
      },
      rustify::Event::EndOfTrack => {
        println!("[Tekno] Track ended");
        break;
      },
    }
  }
}
