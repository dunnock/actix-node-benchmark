use serde_json;
use rand::{RngCore, FromEntropy};
use rand::rngs::SmallRng;


static FNAMES: [&str; 4946] = include!("../texts/first-names.json");
static LNAMES: [&str; 21986] = include!("../texts/names.json");


fn main() {
    let mut rng = SmallRng::from_entropy();

    let fnames: Vec<String> = (0..300).map(|_| FNAMES[rng.next_u32() as usize % 4946].into()).collect();
    serde_json::to_writer_pretty(std::fs::File::create("fnames.json").unwrap(), &fnames).unwrap();

    let lnames: Vec<String> = (0..300).map(|_| LNAMES[rng.next_u32() as usize % 21986].into()).collect();
    serde_json::to_writer_pretty(std::fs::File::create("lnames.json").unwrap(), &lnames).unwrap();

}
