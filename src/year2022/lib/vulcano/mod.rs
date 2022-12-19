//! Vulcano module
//! 
//! Vulcano module
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Vulcano structure
#[derive(Clone)]
#[derive(Debug)]
pub struct Vulcano<'a> {
  valves: HashMap<&'a str, Rc<RefCell<VulcanoValve<'a>>>>
}

#[derive(Clone)]
#[derive(Debug)]
pub struct VulcanoValve<'a> {
  index: usize,
  name: &'a str,
  flow_rate: usize,
  connections: Vec<&'a str>,
  max_released_timeline: Vec<(usize, usize, Vec<bool>, Vec<String>)>
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
    let mut valves: HashMap<&'b str, Rc<RefCell<VulcanoValve<'b>>>> = HashMap::with_capacity(data.len());
    
    // Initialize vulcano valves
    for i in 0..data.len() {
      let reference: Rc<RefCell<VulcanoValve>> = Rc::new(
        RefCell::new(
          VulcanoValve {
            index: i,
            name: data[i].0,
            flow_rate: data[i].1,
            connections: data[i].2.clone(),
            max_released_timeline: vec![]
          }
        )
      );
      valves.insert(
        data[i].0,
        reference
      );
    }

    // Construct and return a vulcano instance
    Vulcano {
      valves
    }
  }

  /// Calculates maximum release, if starting from a given valve, within a given time frame
  /// 
  /// # Arguments
  /// * name: Name of the startin valve
  /// * time_total: Total time in which to calculate maximum release
  /// 
  /// # Returns
  /// Maximum release, if starting from a given valve, within a given time frame
  pub fn calculate_max_release_in_minutes (&mut self, name: &str, time_total: usize) -> usize {

    // Count valves and get all valve names
    let valves_count = self.valves.len();
    let valve_connection_names: Vec<&str> = self.valves.keys().map(|k| k.clone()).collect::<Vec<&str>>();
    
    // For each valve, initialize max_released timeline
    for valve in self.valves.values_mut() {
      valve.borrow_mut().max_released_timeline = vec![];
    }

    // Calculate max releases per each minute, assuming starting point at each possible valve
    for minute in 0..time_total {
      println!("> Minute {}", minute);

      // Calculate max possible time for each position
      for valve_name in &valve_connection_names {
        println!("  > Considering valve {}", valve_name);

        // Get current valve
        let mut valve = self.valves.get(valve_name).unwrap().borrow_mut();

        // If no previous step, store as accumulated 0, and single flow rate
        if minute == 0 {
          // Set initial state for the first minute
          valve.max_released_timeline.push((0, 0, vec![false; valves_count], vec![format!("Starting at valve {}", valve_name)]));
        }
        // If single previous step, check which is best:
        // - Having opened the valve in same space
        // - Having traveled from another location
        else if minute >= 1 {

          // Get previous state for same valve
          let valve_previous_state = &valve.max_released_timeline[minute - 1];
          
          // Initialize best case state and projection
          let mut optimal_state: (usize, usize, Vec<bool>, Vec<String>) = valve_previous_state.clone();
          let mut optimal_projected_release = 0;
          
          // If valve in current location is not open, assume having opened the valve
          if !valve_previous_state.2[valve.index] {
            // Calculate accumulated and rate values
            let accumulated = valve_previous_state.0 + valve_previous_state.1;
            let optimal_projected_release_if_open_valve = accumulated + valve_previous_state.1 * (time_total - minute) + valve.flow_rate * (time_total - minute - 1);
            if optimal_projected_release_if_open_valve > optimal_projected_release {
              // Set best release projection and corresponding state
              optimal_projected_release = optimal_projected_release_if_open_valve;
              optimal_state = (
                accumulated + valve_previous_state.1,
                valve_previous_state.1 + valve.flow_rate,
                valve_previous_state.2.clone(),
                valve_previous_state.3.clone()
              );
              // Mark valve as open
              optimal_state.2[valve.index] = true;
              // Add description
              optimal_state.3.push(format!("Opening valve {}", valve_name));
            }
          }
          println!("    > Opening valve would: accumulate={} flow_rate={} projection={}", optimal_state.0, optimal_state.1, optimal_projected_release);

          // Verify arriving from any of the connected locations and check if projected release is better
          for connection_name in &valve.connections {
            // Get connected valves previous state
            let connected_valve = self.valves.get(connection_name).unwrap().borrow();
            let connected_valve_previous_state = &connected_valve.max_released_timeline[minute - 1];
            // Calculate accumulated and rate values
            let accumulated = connected_valve_previous_state.0;
            let optimal_projected_release_if_this_onnection = accumulated + connected_valve_previous_state.1 * (time_total - minute);
            if optimal_projected_release_if_this_onnection > optimal_projected_release {
              // Set best release projection and corresponding state
              optimal_projected_release = optimal_projected_release_if_this_onnection;
              optimal_state = (
                connected_valve_previous_state.0 + connected_valve_previous_state.1,
                connected_valve_previous_state.1.clone(),
                connected_valve_previous_state.2.clone(),
                connected_valve_previous_state.3.clone()
              );
              // Add description
              optimal_state.3.push(format!("Arriving to valve {} from valve {}", valve_name, connection_name));
              println!("    > Arriving from {} would: accumulate={} flow_rate={} projection={}", connection_name, optimal_state.0, optimal_state.1, optimal_projected_release_if_this_onnection);
            }
          }

          // Set best state for the current minute
          println!("    Best scenario: accumulate={} flow_rate={}", optimal_state.0, optimal_state.1);
          valve.max_released_timeline.push(optimal_state);

        }

      }
    }
    
    // Return max
    self.valves.values().map(|valve| { valve.borrow().max_released_timeline.last().unwrap().0 }).max().unwrap()
  }
}
