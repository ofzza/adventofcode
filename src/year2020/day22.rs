//! 2020/22 puzzle
//! 
//! https://adventofcode.com/2020/day/22
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::inputs::*;
use crate::lib::puzzle::*;
use super::DeckOfCards;
use std::collections::HashMap;

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
      let input = PuzzleInput::Vector1D(vec![
        String::from("Player 1:|9|2|6|3|1").replace("|", "\n"),
        String::from("Player 2:|5|8|4|7|10").replace("|", "\n")
      ]);
      stats.update(
        Puzzle::new(2020, 22, 1, "test", input, implementation1, |r| (r, Some(306)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day22input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 22, 1, "solution", input, implementation1, |r| (r, Some(33925)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Run puzzle
  if (index == 0) || (index == 2) {
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("Player 1:|43|19").replace("|", "\n"),
        String::from("Player 2:|1|19|14").replace("|", "\n")
      ]);
      stats.update(
        Puzzle::new(2020, 22, 2, "test", input, implementation2, |r| (r, Some(0)))
          .run(verbose, obfuscate)
      );
    }
    // Run tests
    if (key == String::default()) || (key == "test") {
      // Test
      let input = PuzzleInput::Vector1D(vec![
        String::from("Player 1:|9|2|6|3|1").replace("|", "\n"),
        String::from("Player 2:|5|8|4|7|10").replace("|", "\n")
      ]);
      stats.update(
        Puzzle::new(2020, 22, 2, "test", input, implementation2, |r| (r, Some(291)))
          .run(verbose, obfuscate)
      );
    }
    // Run solution
    if (key == String::default()) || (key == "solution") {
      // Solution
      let input = parse_1d::<String>(load_input("./src/year2020/data/day22input.txt"), "\n\n");
      stats.update(
        Puzzle::new(2020, 22, 2, "solution", input, implementation2, |r| (r, Some(33441)))
          .run(verbose, obfuscate)
      );
    }
  }

  // Return composed stats
  return stats;

}

fn implementation1 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse input
      let decks = parse_input(input, verbose);
      
      // Play game until a win
      let deck_cards_a = decks[0].clone();
      let mut deck_a = DeckOfCards::new(deck_cards_a.0, deck_cards_a.1);
      let deck_cards_b = decks[1].clone();
      let mut deck_b = DeckOfCards::new(deck_cards_b.0, deck_cards_b.1);
      let winning_deck = play_card_game(&mut deck_a, &mut deck_b, false, verbose, 0).1;

      // Calculate winning score
      let mut score = 0;
      for i in 0..winning_deck.len() {
        score += winning_deck[i] * (winning_deck.len() - i);
      }

      // Return score
      return Ok(score);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

fn implementation2 (puzzle: &Puzzle<String, usize, usize>, verbose: bool) -> Result<usize, &str> {
  match &puzzle.input {
    PuzzleInput::Vector1D(input) => {
      // Parse input
      let decks = parse_input(input, verbose);
      
      // Play game until a win
      let deck_cards_a = decks[0].clone();
      let mut deck_a = DeckOfCards::new(deck_cards_a.0, deck_cards_a.1);
      let deck_cards_b = decks[1].clone();
      let mut deck_b = DeckOfCards::new(deck_cards_b.0, deck_cards_b.1);
      let winning_deck = play_card_game(&mut deck_a, &mut deck_b, true, verbose, 0).1;

      // Calculate winning score
      let mut score = 0;
      for i in 0..winning_deck.len() {
        score += winning_deck[i] * (winning_deck.len() - i);
      }

      // Return score
      // (34242 to high)
      return Ok(score);
    },
    _ => panic!("This shouldn't ever happen!")
  }
}

/// Parses input into 2 separate player decks
/// 
/// # Arguments
/// * `input`   - Strings representing each deck
/// * `verbose` - Outputs executing output of the puzzle to the console
fn parse_input (input: &Vec<String>, _verbose: bool) -> Vec<(Vec<usize>, usize)> {
  // Ready decks
  let mut decks: Vec<(Vec<usize>, usize)> = vec![];

  // Parse decks
  let mut all_cards: Vec<Vec<usize>> = vec![];
  let mut all_cards_count: usize = 0;
  for input in input {
    let cards: Vec<usize> = input.split(":\n")
      .collect::<Vec<&str>>()[1]
      .split("\n")
      .map(|card| card.parse::<usize>().unwrap())
      .collect();
    all_cards.push(cards.clone());
    all_cards_count += cards.len();
  }

  // Initialize decks
  for cards in all_cards {
    decks.push((cards, all_cards_count));
  }

  // Return parsed decks
  return decks;
}

/// Plays out a card game between 2 players
/// 
/// # Arguments
/// * `deck_a`    - 1st player's starting deck
/// * `deck_b`    - 2nd player's starting deck
/// * `recursive` - If the recursive variant of the game is being played
/// * `verbose`   - Outputs executing output of the puzzle to the console
fn play_card_game (deck_a: &mut DeckOfCards, deck_b: &mut DeckOfCards, recursive: bool, verbose: bool, depth: usize) -> (char, Vec<usize>) {
  // Initialize infinite recursion protection
  let mut previous_decks: HashMap<String, bool> = HashMap::new();

  // Prompt
  let padding = vec!["|   "; depth * 2].join("");
  if verbose {
    println!("  {}GAME ...", padding);
  }  

  // Play game until a deck is empty
  let mut move_counter = 0;
  let mut winning_deck: char = 'a';
  while !deck_a.is_empty() && !deck_b.is_empty() {
    // Prompt
    if verbose {
      println!("  {}Move: {}", padding, move_counter + 1);
      println!("  {}  > Player A's deck: {:?}", padding, deck_a.get_cards());
      println!("  {}  > Player B's deck: {:?}", padding, deck_b.get_cards());
    }

    // Count move
    move_counter += 1;

    // Find winner by strongest card
    if !recursive {
      winning_deck = if deck_a.get(0) > deck_b.get(0) { 'a' } else { 'b' };
    }
    
    // Find winner by recursive rules
    else {

      // Generate deck signatures
      let state_sign = vec![deck_a.to_string(), deck_b.to_string() ].join("-");

      // Validate against infinite recursion
      match previous_decks.get(&state_sign) {
        Some(_) => {
          return ('a', vec![]);
        },
        None => {
          previous_decks.insert(state_sign, true);
        }
      }

      // Find winner by a recursive sub-game
      if deck_a.get(0) < deck_a.length && deck_b.get(0) < deck_b.length {
        let subdeck_a = &mut deck_a.subset(1, deck_a.get(0));
        let subdeck_b = &mut deck_b.subset(1, deck_b.get(0));
        winning_deck = play_card_game(subdeck_a, subdeck_b, recursive, verbose, depth + 1 ).0;
      }

      else {
        // Find winner by strongest card
        winning_deck = if deck_a.get(0) > deck_b.get(0) { 'a' } else { 'b' };
      }

    }

    // Take cards from decks
    let pot: Vec<usize> = if winning_deck == 'a' {
      vec![ deck_a.pop().clone(), deck_b.pop().clone() ]
    } else {
      vec![ deck_b.pop().clone(), deck_a.pop().clone() ]
    };
    
    // Place pot to bottom of winning deck
    if winning_deck == 'a' {
      deck_a.push(pot[0]);
      deck_a.push(pot[1]);
    } else {
      deck_b.push(pot[0]);
      deck_b.push(pot[1]);
    }
  }

  // Prompt
  if verbose {
    println!("  {}Game FINISHED on move: {}", padding, move_counter + 1);
    println!("  {}  > Player A's deck: {:?}", padding, deck_a.get_cards());
    println!("  {}  > Player B's deck: {:?}", padding, deck_b.get_cards());
  }
  
  // Return winning deck
  return (winning_deck, if winning_deck == 'a' { deck_a.get_cards() } else { deck_b.get_cards() });
}
