extern crate rustify;

use std::sync::{Arc, Mutex};

mod application_key;

fn main() {
  let args: Vec<_> = std::env::args().collect();
  if args.len() < 3 {
    println!("Usage: ./search USERNAME PASSWORD");
    return;
  }

  // Use OpenAL for playback
  let openal_player = rustify::OpenALPlayer::new();
  let player = Arc::new(Mutex::new(openal_player));

  let (mut session, receiver) = rustify::Session::new(
    application_key::get(),
    "tmp/",
    "tmp/",
    "search-example",
    player,
  );

  session.login(&args[1], &args[2]);

  loop {
    let event = receiver.recv().unwrap();

    match event {
      rustify::Event::LoggedIn => {
        println!("[Tekno] Logged in!");
        break;
      },
      _ => (),
    }
  }

  let search = session.search("Livin la vida Larsson");
  let tracks = search.tracks();

  for track in tracks {
    println!("- {}", track);
  }
}
