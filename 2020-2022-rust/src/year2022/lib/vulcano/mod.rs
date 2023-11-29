//! Vulcano module
//! 
//! Vulcano module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::HashMap;

/// Vulcano structure
pub struct Vulcano<'a> {
  // Valves
  valves: HashMap<&'a str, VulcanoValve<'a>>,
  // Cached maximum flow rate for all valves opened
  _max_flow_rate: usize
}

pub struct VulcanoValve<'a> {
  // Valve unique index
  index: usize,
  // Valve unique name
  name: &'a str,
  // Valve flow rate once open
  flow_rate: usize,
  // Names of connected valves
  connections: Vec<&'a str>,
}

/// Vulcano implementation
impl<'a> Vulcano<'a> {

  /// Constructor
  /// 
  /// # Arguments
  /// * data: Vector of tuples of all valves' names, their flow rate and their connections
  ///         to other valves
  pub fn new<'b> (data: Vec<(&'b str, usize, Vec<&'b str>)>) -> Vulcano<'b> {
    // Initialize valves hashmap
    let mut valves: HashMap<&'b str, VulcanoValve<'b>> = HashMap::with_capacity(data.len());
    
    // Initialize vulcano valves
    for i in 0..data.len() {
      // Initialize a countet reference to the valve
      let valve: VulcanoValve = VulcanoValve {
        // Valve unique index
        index: i,
        // Valve unique name
        name: data[i].0,
        // Valve flow rate once open
        flow_rate: data[i].1,
        // Names of connected valves
        connections: data[i].2.clone()
      };
      // Index the valuve into the hashmap
      valves.insert(data[i].0, valve);
    }

    // Construct and return a vulcano instance
    Vulcano {
      // Valves
      valves,
      // Cached maximum flow rate for all valves opened
      _max_flow_rate: 0
    }
  }

  /// Calculates maximum release, if starting from given valves, within a given time frame
  /// 
  /// # Arguments
  /// * names: Names of the starting valves for each participant
  /// * time_total: Total time in which to calculate maximum release
  /// 
  /// # Returns
  /// Maximum release, if starting from a given valve, within a given time frame
  pub fn calculate_max_release (
    &mut self,
    starting_valve_names: Vec<&str>,
    time_total: usize
  ) -> usize {
    // Reset cached release and flow values
    self._max_flow_rate = 0;
    for valve in self.valves.values() {
      self._max_flow_rate += valve.flow_rate;
    }
    // Initialize skipped tracking for each participant
    let skipped = vec![vec![false; self.valves.len()]; starting_valve_names.len()];
    // Initialize opened tracking, setting valves with flow rate =0 as "good as open"
    let mut opened = vec![false; self.valves.len()];
    for valve in &self.valves.values().collect::<Vec<&VulcanoValve>>() {
      opened[valve.index] = if valve.flow_rate == 0 { true } else { false };
    }
    // Initialize a path for each participant
    let paths = starting_valve_names.iter().map(|name| vec![(name.clone(), false)]).collect::<Vec<Vec<(&str, bool)>>>();
    // Find max release
    let (global_max_release, _, max_paths) = self._calculate_max_release(
      0,
      starting_valve_names,
      time_total,
      0,
      0,
      opened,
      skipped,
      vec![false; paths.len()],
      paths
    );
    // Prompt max paths
    for path in max_paths {
      println!("Path: {:?} ", path.iter().map(|(name, opened)| if opened.clone() { name.to_ascii_uppercase() } else { name.to_ascii_lowercase() }).collect::<Vec<String>>());
    }
    // Return max release
    global_max_release
  }

  /// Internal, recursive method for calculating max release
  fn _calculate_max_release (
    &self,
    global_max_release: usize,
    names: Vec<&'a str>,
    time: usize,
    release_accumulated: usize,
    release_rate: usize,
    opened: Vec<bool>,
    skipped: Vec<Vec<bool>>,
    useless: Vec<bool>,
    paths: Vec<Vec<(&'a str, bool)>>
  ) -> (usize, usize, Vec<Vec<(&'a str, bool)>>) {

    // Make a mutable copy od global MAX release value
    let mut global_max_release = global_max_release;

    // Check if theoretical maximum with all valves opened now, could beat current maximum
    let release_max_theoretical = release_accumulated + (time * self._max_flow_rate);
    if global_max_release >= release_max_theoretical {
      return (global_max_release, release_accumulated + (time * release_rate), paths.clone());
    }

    // Check if no valves left to open
    if match opened.iter().find(|v| !v.clone()) {
      Option::Some(_) => false,
      Option::None => true,
    } {
      return (global_max_release, release_accumulated + (time * release_rate), paths.clone());
    }

    // Initialize a maximum release with a minimum possible value
    let mut release_max = release_accumulated;
    let mut paths_max: Vec<Vec<(&str, bool)>> = paths.clone();

    // Find starting valves
    let valves = names.iter().map(|name| self.valves.get(name).unwrap()).collect::<Vec<&VulcanoValve>>();
    // Find connected valves
    let connected_valves = valves.iter()
      .map(|valve| valve.connections
          .iter()
          .map(|name| self.valves.get(name).unwrap())
          .collect::<Vec<&VulcanoValve>>())
      .collect::<Vec<Vec<&VulcanoValve>>>();

    // Process all participants and compose options for next move
    let mut valves_permutation_candidates_next: Vec<Vec<&VulcanoValve>> = vec![];
    let mut participant_permutation_indexes: Vec<usize> = vec![0; names.len()];
    loop {
      // Get names for current indexes' permutation
      let mut valves_next: Vec<&VulcanoValve> = vec![];
      for participant_index in 0..participant_permutation_indexes.len() {
        valves_next.push(
          // Add connected valve
          if participant_permutation_indexes[participant_index] < connected_valves[participant_index].len() {
            connected_valves[participant_index][participant_permutation_indexes[participant_index]]
          }
          // Add current valve
          else {
            valves[participant_index]
          }
        );
      }
      valves_permutation_candidates_next.push(valves_next);
      
      // Increment indexes
      participant_permutation_indexes[0] += 1;
      // Check all indexes for overflow
      for participant_index in 0..participant_permutation_indexes.len() {
        // Check for overflow
        if participant_permutation_indexes[participant_index] > connected_valves[participant_index].len() {
          // Reset
          participant_permutation_indexes[participant_index] = 0;
          // In next index exist, overflow
          if participant_index < participant_permutation_indexes.len() - 1 {
            participant_permutation_indexes[participant_index + 1] += 1;
          }
        }
      }

      // Check if all indexes overflowed to 0
      if !match participant_permutation_indexes.iter().find(|i| i != &&0) {
        Option::Some(_) => true,
        Option::None => false,
      } {
        break;
      }
    }

    // Remove non-sensical permutations
    let valves_permutations_with_useless_tracked_next = valves_permutation_candidates_next
      .iter()
      .map(|valves_permutation_next| {

        // Initialize useless move tracker per perticipant
        let mut useless_next = useless.clone();

        // Process participants
        let mut valve_opening_count: Vec<bool> = vec![false; self.valves.len()];        
        let mut participants_useless_any: bool = false;
        let mut participants_useless_count: usize = 0;
        for participant_index in 0..valves_permutation_next.len() { 
          // Check non-useless participants returning to skipped valves
          //  - if returning to skipped valves, count towards filtering out permutation.
          //  - only filter out if no participants are making useful moves.
          //  - ... else, make participant with no useful move marked as useless for all future moves
          if useless_next[participant_index] || skipped[participant_index][valves_permutation_next[participant_index].index] {
            participants_useless_count += 1;
            participants_useless_any = true;
            useless_next[participant_index] = true;
          }

          // Check participants staying in place to open valves (unless known to be returning to skipped)
          if !useless_next[participant_index] && valves[participant_index].index == valves_permutation_next[participant_index].index {
            // If opening valve with flow rate 0, filter out permutation
            if valves[participant_index].flow_rate == 0 {
              return (valves_permutation_next, false, false, useless_next);
            }
            // If opening valve that is already open, filter out permutation
            if opened[valves[participant_index].index] {
              return (valves_permutation_next, false, false, useless_next);
            }
            // If valve already being opened by another participant, filter out permutation
            if valve_opening_count[valves[participant_index].index] {
              return (valves_permutation_next, false, false, useless_next);
            }
            // Count participant opening this valve
            valve_opening_count[valves[participant_index].index] = true;
          }

          // If all participants retuning to skipped valves, filter out permutation
          if participants_useless_count == valves_permutation_next.len() {
            return (valves_permutation_next, false, false, useless_next);
          }
        }

        // By default, keep permutation
        (valves_permutation_next, true, participants_useless_any, useless_next)
      })
      .filter(|(_, allowed, _, _)| allowed == &true)
      // .map(|(permutations, usefull, useless_participants)| (permutations.clone())
      .collect::<Vec<(&Vec<&VulcanoValve>, bool, bool, Vec<bool>)>>();

    // Remove duplicate permutations
    let mut valves_permutations_next_deduplicated: HashMap<Vec<&str>, (Vec<&VulcanoValve>, bool, Vec<bool>)> = HashMap::with_capacity(valves_permutations_with_useless_tracked_next.len());
    for (valves_permutation_next, _, useless_any, useless_next) in valves_permutations_with_useless_tracked_next {
      let names = valves_permutation_next.iter()
                    .enumerate()              
                    .map(|(i, value)|
                      // Use name to sort and deduplicate by
                      if !useless_next[i] { value.name }
                      // If returning to skipped value, use name replacement to deduplicate with other useless, returnung permutations
                      else {
                        "__"
                      })
                    .collect::<Vec<&str>>();
      let mut names_sorted = names; names_sorted.sort();
      if !valves_permutations_next_deduplicated.contains_key(&names_sorted) {
        valves_permutations_next_deduplicated.insert(names_sorted, (valves_permutation_next.clone(), useless_any, useless_next));
      }
    }
    let mut valves_permutations_next = valves_permutations_next_deduplicated.values()
      .map(|(permutation, useless_any, useless_next)| {
        (permutation.clone(), useless_any.clone(), useless_next.clone())
      })
      .collect::<Vec<(Vec<&VulcanoValve>, bool, Vec<bool>)>>();

    // Check if no valid permutations left to move to
    if valves_permutations_next.len() == 0 {
      return (global_max_release, release_accumulated + (time * release_rate), paths.clone());
    }

    // Sort permutations with no useless perticipants first
    valves_permutations_next
      .sort_by(|a, b| {
        (if !b.1 { 1 } else { 0 }).cmp(&if !a.1 { 1 } else { 0 })
      });

    // Try all moves and find max release
    for i in 0..valves_permutations_next.len() {

      // Get next valves for all participants to visit according to current permutation
      let valves_next = valves_permutations_next[i].0.clone();
      let mut useless_next = valves_permutations_next[i].2.clone();

      // Check if time left for opening a valve or for moving to another valve
      if time > 0 {

        // Append to each participant's:
        // - Opened valves tracker
        // - Path
        // - Skipped valves tracker
        // - Change to flow rate due to opening a valve
        let mut opened_next = opened.clone();
        let mut paths_next = paths.clone();
        let mut skipped_next = skipped.clone();
        let mut release_rate_diff: usize = 0;
        for participant_index in 0..valves_next.len() {
          // Get valve name and opening status
          let valve_next_name = valves_next[participant_index].name;
          let valve_current_index = valves[participant_index].index;
          let valve_current_flow_rate = valves[participant_index].flow_rate;
          let valve_opening = valves[participant_index].index == valves_next[participant_index].index;
          // Check if participant is useless
          let participant_is_useless = useless_next[participant_index] || skipped[participant_index][valve_current_index];
          if participant_is_useless {
            useless_next[participant_index] = true;
          }
          // Append to participant's path
          paths_next[participant_index].push((if !participant_is_useless { valve_next_name } else { "__" }, valve_opening));
          // Update participants skipped tracking
          if !participant_is_useless && valve_opening {
            skipped_next[participant_index] = vec![false; self.valves.len()];
          }
          else {
            skipped_next[participant_index][valve_current_index] = true;
          }
          // If opening valve, set opened status
          if !participant_is_useless && valve_opening {
            opened_next[valve_current_index] = true;
          }
          // If opening valve, add to flow rate differential
          if !participant_is_useless && valve_opening {
            release_rate_diff += valve_current_flow_rate;
          }
        }

        // Get names of next valves
        let names_next = valves_next.iter().map(|v| v.name.clone()).collect::<Vec<&str>>();
        // Calculate release after having opened the valve (and have spent an extra minute to do so ...)
        let release_accumulated_next = release_accumulated + release_rate;
        let release_rate_next = release_rate + release_rate_diff;
        let (_, returned_release, returned_path) = self._calculate_max_release(global_max_release, names_next, time - 1, release_accumulated_next, release_rate_next, opened_next, skipped_next, useless_next, paths_next);
        // Check if release is better than current max
        if returned_release > release_max {
          // Set next maximum
          release_max = returned_release;
          paths_max = returned_path.clone();
          if returned_release > global_max_release {
            global_max_release = returned_release;
            
            // println!("New MAX: {}", global_max_release);
            // for path in &returned_path {
            //   println!("- Path: {:?} ", path.iter().map(|(name, opened)| if opened.clone() { name.to_ascii_uppercase() } else { name.to_ascii_lowercase() }).collect::<Vec<String>>());
            // }
          }
        }

        // Prompt paths
        // if time > 20 {
        //   println!("Time {} | Permutation {}/{}:", time, i, valves_permutations_next.len());
        //   for path in &returned_path {
        //     println!("- Path: {:?} ", path.iter().map(|(name, opened)| if opened.clone() { name.to_ascii_uppercase() } else { name.to_ascii_lowercase() }).collect::<Vec<String>>());
        //   }
        // }

      }
    }

    // Add another minute of release rate to accumulated release
    (global_max_release, release_max, paths_max)
  }
}
