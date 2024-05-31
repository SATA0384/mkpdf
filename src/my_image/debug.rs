use std::fmt::Display;

const IS_DEBUG: bool = false;

pub(crate) fn debug_print(msg: impl Display) {
  if IS_DEBUG {
    eprintln!("{msg}");
  }
}
