use malachite::Natural;
use malachite_base::num::{
  arithmetic::traits::{ModAdd, ModInverse, ModMul, ModNeg, ModPow},
  conversion::traits::FromStringBase,
  logic::traits::BitIterable,
};
use std::collections::HashMap;
use std::ops::{Add, Mul};

const GX: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";

pub fn g() -> Point {
  Point {
    x: Natural::from_string_base(16, GX).unwrap(),
    y: Natural::from_string_base(
      16,
      "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
    )
    .unwrap(),
    z: Natural::from(1u32),
  }
}

fn zero() -> Point {
  Point {
    x: malachite::Natural::from(0u32),
    y: malachite::Natural::from(0u32),
    z: malachite::Natural::from(0u32),
  }
}

pub fn powed_points() -> HashMap<usize, Point> {
  let mut res: HashMap<usize, Point> = HashMap::new();
  res.insert(0, g().clone());
  for pow in 1..256 {
    res.insert(pow, res[&(pow - 1)].double());
  }
  res
}

mod curve {
  use super::*;

  pub fn p() -> malachite::Natural {
    natural_from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
  }

  pub fn order() -> malachite::Natural {
    natural_from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141")
  }
}

#[derive(Debug, Clone)]
pub struct Point {
  pub x: malachite::Natural,
  pub y: malachite::Natural,
  pub z: malachite::Natural,
}

impl Point {
  pub fn double(&self) -> Point {
    let x1 = self.x.clone();
    let y1 = self.y.clone();
    let p = curve::p();
    let n2 = Natural::from(2u32);
    let xx = x1.clone().mod_pow(&n2, &p);
    let yy = y1.clone().mod_pow(&n2, &p);
    let yyyy = yy.clone().mod_pow(&n2, &p);
    let s = x1
      .clone()
      .mod_add(&yy, &p)
      .mod_pow(&n2, &p)
      .mod_add(xx.clone().mod_neg(&p), &p)
      .mod_add(yyyy.clone().mod_neg(&p), &p)
      .mod_mul(&n2, &p);
    let m = xx.clone().mod_mul(malachite::Natural::from(3u32), &p);
    let t = m
      .clone()
      .mod_pow(&n2, &p)
      .mod_add(s.clone().mod_mul(&n2, &p).mod_neg(&p), &p);
    let z3 = y1.mod_mul(&n2, &p);
    let y3 = s.mod_add(t.clone().mod_neg(&p), &p).mod_mul(m, &p).mod_add(
      yyyy.mod_mul(malachite::Natural::from(8u32), &p).mod_neg(&p),
      &p,
    );
    Point { x: t, y: y3, z: z3 }.scale()
  }
  pub fn is_zero(&self) -> bool {
    0 == self.x && 0 == self.y && 0 == self.z
  }
  pub fn scale(&self) -> Point {
    let p = curve::p();
    let z = self.z.clone().mod_inverse(&p).unwrap();
    let x = self
      .x
      .clone()
      .mod_mul(z.clone().mod_pow(Natural::from(2u32), &p), &p);
    let y = self
      .y
      .clone()
      .mod_mul(z.clone().mod_pow(Natural::from(3u32), &p), &p);
    Point { x: x, y: y, z: z }
  }
}

impl Mul<malachite::Natural> for Point {
  type Output = Self;
  fn mul(self, other: malachite::Natural) -> Self::Output {
    let powed = powed_points();
    let mut acc = zero();
    for (ind, bit) in other.bits().enumerate() {
      if bit {
        acc = acc + powed.get(&(ind)).unwrap().clone();
      }
    }
    acc
  }
}

impl Add<Point> for Point {
  type Output = Self;
  fn add(self, rhs: Point) -> Self::Output {
    if self.is_zero() {
      return rhs;
    }
    if rhs.is_zero() {
      return self;
    }
    let p = curve::p();
    let n2 = Natural::from(2u32);
    let h = rhs.x.clone().mod_add(self.x.clone().mod_neg(&p), &p);
    let hh = h.clone().mod_pow(&n2, &p);
    let ii = hh.mod_mul(Natural::from(4u32), &p);
    let j = h.clone().mod_mul(ii.clone(), &p);
    let r = self
      .y
      .clone()
      .mod_neg(&p)
      .mod_add(rhs.y.clone(), &p)
      .mod_mul(&n2, &p);
    let v = self.x.clone().mod_mul(ii, &p);
    let x3 = r
      .clone()
      .mod_pow(&n2, &p)
      .mod_add(j.clone().mod_neg(&p), &p)
      .mod_add(v.clone().mod_neg(&p).mod_mul(&n2, &p), &p);
    let z3 = h.clone().mod_mul(&n2, &p);
    let y3 = x3
      .clone()
      .mod_neg(&p)
      .mod_add(v, &p)
      .mod_mul(r, &p)
      .mod_add(n2.mod_mul(self.y.clone(), &p).mod_mul(j, &p).mod_neg(&p), &p);

    Point {
      x: x3,
      y: y3,
      z: z3,
    }
    .scale()
  }
}

impl Mul<u128> for Point {
  type Output = Self;
  fn mul(self, other: u128) -> Self::Output {
    if 1 == other {
      return self;
    }
    if 2 == other {
      return self.double();
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
  use malachite_base::num::conversion::traits::FromStringBase;

  use super::*;

  #[test]
  fn check_g_mul() {
    let p = g() * 1;
    assert_eq!(p.x, natural_from_hex(GX));

    let p = g() * 2;
    assert_eq!(
      p.x,
      malachite::Natural::from_string_base(
        10,
        "89565891926547004231252920425935692360644145829622209833684329913297188986597"
      )
      .unwrap()
    );
    assert_eq!(
      p.y,
      malachite::Natural::from_string_base(
        10,
        "12158399299693830322967808612713398636155367887041628176798871954788371653930"
      )
      .unwrap()
    );

    let p = g() * 5;
    assert_eq!(
      p.x,
      malachite::Natural::from_string_base(
        10,
        "21505829891763648114329055987619236494102133314575206970830385799158076338148"
      )
      .unwrap()
    );
    assert_eq!(
      p.y,
      malachite::Natural::from_string_base(
        10,
        "98003708678762621233683240503080860129026887322874138805529884920309963580118"
      )
      .unwrap()
    );
  }
}
