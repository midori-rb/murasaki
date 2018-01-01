use std::time::Duration;

#[macro_use]
extern crate helix;
#[macro_use]
extern crate lazy_static;
extern crate mio;
use mio::{Poll};

static mut CLOSED: bool = true;
lazy_static! {
  static ref POLL: Poll = Poll::new().unwrap();
}

ruby! {
  class Selector {
    def echo() -> String {
      "Hello Murasaki Rust Selector!".to_string()
    }

    def close() {
      unsafe {
        CLOSED = true;
      }
    }

    #[ruby_name = "closed?"]
    def is_close() -> bool {
      unsafe { CLOSED }
    }

    def select(timeout: f64) {
      let millis = (timeout * 1_000.0) as u64;
      let duration = Duration::from_millis(millis);
    }
  }
}
