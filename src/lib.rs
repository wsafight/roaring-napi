#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_plus_100 () {
    assert_eq!(plus_100(0), 100)
  }
}
