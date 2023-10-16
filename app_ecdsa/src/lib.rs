use std::ops::Mul;

const GX: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";

pub fn g() -> Point {
  Point {
    x: natural_from_hex(GX),
    y: natural_from_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8"),
    z: malachite::Natural::from(1u32),
  }
}

#[derive(Debug)]
pub struct Point {
  pub x: malachite::Natural,
  pub y: malachite::Natural,
  pub z: malachite::Natural,
}

// impl Point {
//   pub fn g() -> Point {
//     Point {
//     }
//   }
// }

impl Mul<malachite::Natural> for Point {
  type Output = Self;
  fn mul(self, other: malachite::Natural) -> Self::Output {
    Self {
      x: malachite::Natural::from(1u32),
      y: malachite::Natural::from(1u32),
      z: malachite::Natural::from(1u32),
    }
  }
}

impl Mul<u128> for Point {
  type Output = Self;
  fn mul(self, other: u128) -> Self::Output {
    println!("Mul u128");
    if 1 == other {
      return self;
    }
    self * malachite::Natural::from(other)
  }
}

pub fn natural_from_hex(numb: &str) -> malachite::Natural {
  let mut res: Vec<u64> = Vec::new();
  let mut word: String = "".to_owned();
  for (ind, char) in numb.split("").enumerate() {
    word = word + char;
    if (ind > 0) && (0 == ind % 16) {
      res.push(u64::from_str_radix(word.as_str(), 16).unwrap());
      word = "".to_owned();
    }
  }
  malachite::Natural::from_limbs_desc(&res)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_g_mul() {
    let p = g() * 1;
    assert_eq!(p.x, natural_from_hex(GX));
  }
}
