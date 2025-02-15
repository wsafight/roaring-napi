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
    // 直接修改覆盖
    self.bitmap = InnerRoaringBitmap::deserialize_from(&bytes[..]).unwrap();
  }

  #[napi]
  pub fn union(&self, other: &RoaringBitmap) -> RoaringBitmap {
    RoaringBitmap {
      bitmap: &self.bitmap | &other.bitmap,
    }
  }

  #[napi]
  pub fn intersection(&self, other: &RoaringBitmap) -> RoaringBitmap {
    RoaringBitmap {
      bitmap: &self.bitmap & &other.bitmap,
    }
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

  #[test]
  fn test_serialize() {
    let mut roaring = RoaringBitmap::new();
    roaring.add(1);
    roaring.add(1000);
    roaring.add(10);
    let byte_vec: Vec<u8> = roaring.serialize();
    assert_eq!(
      byte_vec,
      vec![58, 48, 0, 0, 1, 0, 0, 0, 0, 0, 2, 0, 16, 0, 0, 0, 1, 0, 10, 0, 232, 3]
    )
  }

  #[test]
  fn test_deserialize() {
    let mut roaring: RoaringBitmap = RoaringBitmap::new();
    let byte_vec: Vec<u8> = vec![
      58, 48, 0, 0, 1, 0, 0, 0, 0, 0, 2, 0, 16, 0, 0, 0, 1, 0, 10, 0, 232, 3,
    ];
    roaring.deserialize(byte_vec);

    assert_eq!(roaring.len(), 3);
    assert_eq!(roaring.has(1000), true);
  }

  #[test]
  fn test_serialize_deserialize() {
    let mut roaring: RoaringBitmap = RoaringBitmap::new();

    roaring.add(1);
    roaring.add(1000);
    roaring.add(10);
    roaring.add(1000000000);

    let byte_vec: Vec<u8> = roaring.serialize();
    let mut roaring2 = RoaringBitmap::new();
    roaring2.deserialize(byte_vec.clone());
    assert_eq!(byte_vec, roaring2.serialize())
  }

  #[test]
  fn test_union() {
      let mut bitmap1 = RoaringBitmap::new();
      bitmap1.add(1);
      bitmap1.add(2);
      bitmap1.add(3);
      let mut bitmap2 = RoaringBitmap::new();
      bitmap2.add(3);
      bitmap2.add(4);
      bitmap2.add(5);

      let result = bitmap1.union(&bitmap2);
      assert_eq!(bitmap1.len(), 3);
      assert_eq!(bitmap2.len(), 3);
      assert_eq!(result.len(), 5);
  }

  #[test]
    fn test_intersection() {
        let mut bitmap1 = RoaringBitmap::new();
        bitmap1.add(1);
        bitmap1.add(2);
        bitmap1.add(3);

        let mut bitmap2 = RoaringBitmap::new();
        bitmap2.add(2);
        bitmap2.add(3);
        bitmap2.add(4);

        let result = bitmap1.intersection(&bitmap2);

        assert_eq!(result.len(), 2);
        assert_eq!(result.has(2), true);
        assert_eq!(result.has(3), true);
        assert_eq!(result.has(1), false);
        assert_eq!(result.has(4), false);
    }
}
