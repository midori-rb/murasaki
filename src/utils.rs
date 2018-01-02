use helix::Symbol;

#[cfg(any(target_os = "android",
          target_os = "linux"))]
pub fn backend() -> Symbol {
  Symbol::from_string("epoll".to_string())
}

#[cfg(any(target_os = "dragonfly",
          target_os = "freebsd",
          target_os = "ios",
          target_os = "macos",
          target_os = "netbsd"))]
pub fn backend() -> Symbol {
  Symbol::from_string("kqueue".to_string())
}

#[cfg(target_os = "windows")]
pub fn backend() -> Symbol {
  Symbol::from_string("iocp".to_string())
}
