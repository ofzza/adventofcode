//! 2020/25 puzzle
//! 
//! https://adventofcode.com/2020/day/25
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;

/// Registers puzzles for the day
pub fn run (index: u32, key: &str, verbose: bool, obfuscate: bool) -> PuzzleExecutionStatistics {

  // Initialize stats
  let mut stats = PuzzleExecutionStatistics{
    ..Default::default()
  };

  // Run puzzle
  if (index == 0) || (index == 1) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![5764801, 17807724]);
      stats.update(
        Puzzle::new(2020, 25, 1, "test", input, implementation1, |r| (r, Some(14897079)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<usize>(load_input("./src/year2020/data/day25input.txt"), "\n");
      stats.update(
        Puzzle::new(2020, 25, 1, "solution", input, implementation1, |r| (r, Some(16902792)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<usize, usize, usize>, _verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(public_keys) => {
      // Crack public keys
      let modulo = 20201227;
      let seed = 7;
      let private_key_a = extract_private_key(public_keys[0], seed, modulo).expect("Failed cracking key!");
      // let private_key_b = extract_private_key(public_keys[1], seed, modulo).expect("Failed cracking key!");
      // Verify key
      // if encrypt(private_key_a, public_keys[1], modulo, 1) != encrypt(private_key_b, public_keys[0], modulo, 1) {
      //   panic!("Generated keys aren't symmetric!")
      // }
      // Calculate shared key
      let key = encrypt(private_key_a, public_keys[1], modulo, 1);
      // Return shared key
      Ok(key)
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Extracts a private key from a public key and known encrypted subject
/// 
/// Arguments
/// # * `public_key` - Public key that we're decrypting
/// # * `subject`    - Subject that was encrypted
/// # * `modulo`     - Value used for modular arithmetic algorithm used in generating encrypted data
fn extract_private_key (public_key: usize, subject: usize, modulo: usize) -> Option<usize> {
  // Test keys
  //   (subject ^ private_key?) % modulo = data
  //   (subject ^ private_key?) = (k? * modulo) + data
  //   private_key? = ((k? * modulo) + data).log(subject)
  let mut last_candidate_key = 1;
  for i in 0..modulo {
    // Continue calculation from previous iteration approach
    last_candidate_key = encrypt(1, subject, modulo, last_candidate_key.clone());
    if last_candidate_key == public_key {
      return Some(i + 1);
    }
  }
  // Return no private key found
  return None;
}

/// Encrypts a subject using a private key
/// 
/// Arguments
/// # * `private_key` - Private key to use for encryption
/// # * `subject`     - Subject to encrypt
/// # * `modulo`      - Value used for modular arithmetic algorithm used in generating encrypted data
/// # * `seed`        - Initial value to generate from (can pass previous privete_key and continue generation)
fn encrypt (private_key: usize, subject: usize, modulo: usize, seed: usize) -> usize {
  // Initialize encrypted data
  let mut data = seed;
  // Encrypt data:
  //   data? = (subject ^ private_key) % modulo
  for _ in 0..private_key {
    data = (data * subject) % modulo;
  }
  // Return encrypted data
  return data;
}
