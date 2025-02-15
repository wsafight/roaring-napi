#![deny(clippy::all)]

use napi_derive::napi;
use roaring::RoaringBitmap as InnerRoaringBitmap;

#[napi]
pub struct RoaringBitmap {
  bitmap: InnerRoaringBitmap,
}

#[napi]
impl RoaringBitmap {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      bitmap: InnerRoaringBitmap::new(),
    }
  }

  #[napi]
  pub fn add(&mut self, value: u32) {
    self.bitmap.insert(value);
  }

  #[napi]
  pub fn has(&self, value: u32) -> bool {
    self.bitmap.contains(value)
  }

  #[napi]
  pub fn remove(&mut self, value: u32) {
    self.bitmap.remove(value);
  }

  #[napi]
  pub fn len(&self) -> u32 {
    self.bitmap.len() as u32
  }

  #[napi]
  pub fn is_empty(&self) -> bool {
    self.bitmap.is_empty()
  }

  #[napi]
  pub fn serialize(&self) -> Vec<u8> {
    let mut bytes = vec![];
    self.bitmap.serialize_into(&mut bytes).unwrap();
    bytes
  }

  #[napi]
  pub fn deserialize(&mut self, bytes: Vec<u8>) {
    self.bitmap = InnerRoaringBitmap::deserialize_from(&bytes[..]).unwrap();
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_empty() {
    let roaring = RoaringBitmap::new();
    assert_eq!(roaring.is_empty(), true);
    assert_eq!(roaring.len(), 0)
  }

  #[test]
  fn test_add_value() {
    let mut roaring = RoaringBitmap::new();
    roaring.add(1);
    assert_eq!(roaring.is_empty(), false);
    assert_eq!(roaring.len(), 1)
  }


  #[test]
  fn test_has_value() {
    let mut roaring = RoaringBitmap::new();
    roaring.add(1);
    assert_eq!(roaring.has(1), true)
  }
}
