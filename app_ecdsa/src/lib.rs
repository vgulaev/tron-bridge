use malachite::Natural;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::num::{
  arithmetic::traits::{DivMod, ModAdd, ModInverse, ModMul, ModNeg, ModPow},
  conversion::traits::{FromStringBase, ToStringBase},
  logic::traits::BitIterable,
};
use malachite_base::random::Seed;
use malachite_nz::natural::random::get_random_natural_with_bits;
use rand::Rng;
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

pub mod curve {
  use super::*;

  pub fn p() -> Natural {
    Natural::from_string_base(
      16,
      "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
    )
    .unwrap()
  }

  pub fn order() -> Natural {
    Natural::from_string_base(
      16,
      "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
    )
    .unwrap()
  }
}

#[derive(Debug, Clone)]
pub struct Point {
  pub x: Natural,
  pub y: Natural,
  pub z: Natural,
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
    let y3 = s
      .mod_add(t.clone().mod_neg(&p), &p)
      .mod_mul(m, &p)
      .mod_add(yyyy.mod_mul(Natural::from(8u32), &p).mod_neg(&p), &p);
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
    Point {
      x: x,
      y: y,
      z: Natural::from(1u32),
    }
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
      .mod_add(
        n2.mod_mul(self.y.clone(), &p).mod_mul(j, &p).mod_neg(&p),
        &p,
      );

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

fn get_rnd_seed() -> Seed {
  let mut bytes: [u8; 32] = [0; 32];
  for i in 0..32 {
    bytes[i] = rand::thread_rng().gen_range(0..255);
  }
  Seed::from_bytes(bytes)
}

fn number_to_string(numb: Natural, len: usize) -> String {
  let res = numb.to_string_base(16);
  if res.len() < len {
    let zeros = format!("{:0>64}", "");
    return String::from(&zeros[0..(len - res.len())]) + &res;
  }
  res
}

pub fn sign_hex_number_with_k(hexed: &str, private_key: &str, k: Natural) -> String {
  let e = Natural::from_string_base(16, &hexed).unwrap();
  let d = Natural::from_string_base(16, &private_key).unwrap();
  let r = g() * k.clone();
  let n = curve::order();
  let s = r
    .x
    .clone()
    .mod_mul(d, &n)
    .mod_add(e, &n)
    .mod_mul(k.clone().mod_inverse(&n).unwrap(), n);
  let rec_id = r
    .y
    .clone()
    .div_mod(Natural::from(2u32))
    .1
    .add(Natural::from(27u32));
  number_to_string(r.x, 64) + &number_to_string(s, 64) + &number_to_string(rec_id, 2)
}

pub fn sign_hex_number(hexed: &str, private_key: &str) -> String {
  sign_hex_number_with_k(
    hexed,
    private_key,
    get_random_natural_with_bits(&mut random_primitive_ints(get_rnd_seed()), 255),
  )
}

#[cfg(test)]
mod tests {
  use malachite_base::num::conversion::traits::FromStringBase;

  use super::*;

  #[test]
  fn check_g_mul() {
    let p = g() * 1;
    assert_eq!(p.x, Natural::from_string_base(16, GX).unwrap());

    let p = g() * 2;
    assert_eq!(
      p.x,
      Natural::from_string_base(
        10,
        "89565891926547004231252920425935692360644145829622209833684329913297188986597"
      )
      .unwrap()
    );
    assert_eq!(
      p.y,
      Natural::from_string_base(
        10,
        "12158399299693830322967808612713398636155367887041628176798871954788371653930"
      )
      .unwrap()
    );

    let p = g() * 5;
    assert_eq!(
      p.x,
      Natural::from_string_base(
        10,
        "21505829891763648114329055987619236494102133314575206970830385799158076338148"
      )
      .unwrap()
    );
    assert_eq!(
      p.y,
      Natural::from_string_base(
        10,
        "98003708678762621233683240503080860129026887322874138805529884920309963580118"
      )
      .unwrap()
    );

    let p = g()
      * Natural::from_string_base(
        10,
        "21505829891763648114329055987619236494102133314575206970830385799158076338148",
      )
      .unwrap();

    assert_eq!(
      p.x,
      Natural::from_string_base(
        10,
        "13219366370872709945630803740359226592071431062398954814922403555987972452022"
      )
      .unwrap()
    );
    assert_eq!(
      p.y,
      Natural::from_string_base(
        10,
        "1695008698159388468205708382445709394669011376943299176310085773645216812718"
      )
      .unwrap()
    );
    assert_eq!(p.z, Natural::from(1u32));
  }

  #[test]
  fn check_number_to_string() {
    assert_eq!(number_to_string(Natural::from(1u32), 10), "0000000001");
    assert_eq!(
      number_to_string(Natural::from_string_base(16, "483a").unwrap(), 10),
      "000000483a"
    );
    assert_eq!(
      number_to_string(
        Natural::from_string_base(16, "ffffffffffffffffffffff").unwrap(),
        32
      ),
      "0000000000ffffffffffffffffffffff"
    );
  }

  #[test]
  fn check_sign_hex_number_with_k() {
    assert_eq!(sign_hex_number_with_k(
      "bfc1d87ea2044ddbdc36c9f9725596d7855c2e88c033fdb0e29213c010ddb5d7",
      "ab4a34b671936ef061602752afe26fd13a31ce75d47d0c02401ae3fdcbca968a",
      Natural::from_string_base(10, "45815476243236585296874261893402625311887309792285287856772039354732826960223").unwrap()
    ),
      "1c319dc8fff6c16503b35e3278da189d3fb8f4d05d5f5ed12de443ed3e36b18aeda99b88479c3ee5ef4595f83941f14c713e9033379c28a4dd38bab7eb7010d71b");
  }
}
