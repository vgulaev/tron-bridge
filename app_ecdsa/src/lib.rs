pub fn hex_str_to_limbs(numb: &str) -> Vec<u64> {
  let mut res: Vec<u64> = Vec::new();
  let mut word: String = "".to_owned();
  for (ind, char) in numb.split("").enumerate() {
    word = word + char;
    if (ind > 0) && (0 == ind % 16) {
      res.push(u64::from_str_radix(word.as_str(), 16).unwrap());
      word = "".to_owned();
    }
  }
  res
}
