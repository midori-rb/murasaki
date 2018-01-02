#![recursion_limit="128"]
#[macro_use]
extern crate helix;
extern crate mio;
use std::sync::Arc;
use std::time::Duration;
use helix::Symbol;
use mio::{Poll, Events};

mod utils;

ruby! {
  class Selector {
    struct {
      closed: bool,
      poll: Arc<Poll>,
    }

    def initialize(helix) {
      let poll = match Poll::new() {
        Ok(poll) => poll,
        Err(e) => panic!("failed to create Poll instance; err={:?}", e),
      };
      Selector {
        helix,
        closed: false,
        poll: Arc::new(poll),
      }
    }

    def backend(&self) -> Symbol {
      utils::backend()
    }

    def close(&mut self) {
      self.closed = true;
    }

    #[ruby_name = "closed?"]
    def is_closed(&self) -> bool {
      self.closed
    }

    def _deregister(_fd: u64) {
      panic!("Not Implemented");
    }

    #[ruby_name = "empty?"]
    def is_empty(&self) {
      panic!("Not Implemented");
    }

    def _register(_fd: u64, _interest: Symbol) {
      panic!("Not Implemented");
    }

    #[ruby_name = "_registered?"]
    def is_registered(_fd: u64) {
      panic!("Not Implemented");
    }

    def _select(&self, timeout: f64) {
      let mut events = Events::with_capacity(1024);
      let poll = self.poll.clone();
      let timeout_millis = (timeout * 1000.0) as u64;
      let duration = Duration::from_millis(timeout_millis);
      let _result = poll.poll(&mut events, Some(duration)); // The reult should be ignored
      for _event in events.iter() {
        // TODO: Deal with the event
      }
      panic!("Not Implemented");
    }
  }

  class Monitor {
    struct {
      closed: bool,
      interests: Symbol,
      fd: u64,
      selector: Selector,
    }

    def initialize(helix, interests: Symbol, fd: u64, selector: Selector) {
      Monitor{
        helix,
        closed: false,
        interests,
        fd,
        selector,
      }
    }

    def fd(&self) -> u64 {
      self.fd
    }

    def interests(&self) -> Symbol {
      self.interests
    }

    def readiness(&self) {
      panic!("Not Implemented");
    }

    def close(&mut self) {
      // TODO: deregister first
      self.closed = true;
      panic!("Not Implemented");
    }

    #[ruby_name = "closed?"]
    def is_closed(&self) -> bool {
      self.closed
    }

    #[ruby_name = "readable?"]
    def is_readable(&self){
      panic!("Not Implemented");
    }

    def remove_interest(&self, _interest: Symbol){
      panic!("Not Implemented");
    }

    #[ruby_name = "writable?"]
    def is_writable(&self){
      panic!("Not Implemented");
    }
  }
}
