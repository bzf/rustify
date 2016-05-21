use openal::al;
use openal::alc;

use spotify;

pub struct OpenALPlayer {
  source: al::Source,
  buffers: Vec<al::Buffer>,
}

impl OpenALPlayer {
  pub fn new() -> OpenALPlayer {
    use std::sync::{Once, ONCE_INIT};
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| {
      let device = alc::Device::open(None).expect("Could not open device");
      let ctx = device.create_context(&[]).expect("Could not create context");
      ctx.make_current();
    });

    OpenALPlayer {
      source: al::Source::gen(),
      buffers: Vec::new(),
    }
  }
}

unsafe impl Send for OpenALPlayer { }
unsafe impl Sync for OpenALPlayer { }

impl spotify::MusicPlayer for OpenALPlayer {
  fn handle_music_delivery(&mut self, frames: &Vec<i32>) -> i32 {
    if self.source.get_buffers_queued() - self.source.get_buffers_processed() > 10 {
      return 0;
    }

    let buffer = al::Buffer::gen();
    unsafe {
      buffer.buffer_data(al::Format::Stereo16,
                         &frames,
                         44_100 as al::ALsizei)
    };

    self.source.queue_buffer(&buffer);
    self.buffers.push(buffer);

    for _ in 0..self.source.get_buffers_processed() {
      let mut f = self.buffers.remove(0);
      self.source.unqueue_buffer(&mut f);
      f.delete();
    }

    if !self.source.is_playing() {
      self.play(true);
    }

    return (frames.len() * 2) as i32;
  }

  fn reset(&mut self) {
    self.source = al::Source::gen();
    self.buffers = Vec::new();
  }

  fn is_playing(&self) -> bool {
    return self.source.is_playing();
  }

  fn play(&self, should_play: bool) {
    if should_play {
      self.source.play();
    } else {
      self.source.stop();
    }
  }
}
