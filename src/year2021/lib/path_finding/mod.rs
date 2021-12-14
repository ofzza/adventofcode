//! Path finding module
//! 
//! Finds path through connected graphs
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::*;

/// Path finding struct
pub struct PathFinding<T> where T: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Debug {
  connections: HashMap<T, Vec<T>>
}
/// Path finding implementation
impl<T> PathFinding<T> where T: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Debug {

  /// Constructor
  /// 
  /// # Arguments
  /// * connections: Traversable connections in the graph
  pub fn new(connections: Vec<(T, T)>) -> PathFinding<T> {
    // Initialize connections
    let mut hash: HashMap<T, Vec<T>> = HashMap::with_capacity(connections.len());
    for i in 0..connections.len() {
      PathFinding::add_to_hash(&mut hash, connections[i]);
    }
    // Return initialized path finding
    PathFinding {
      connections: hash
    }
  }

  /// Adds a connection to the traversable graph
  /// 
  /// # Arguments
  /// * hash:       HAshmap containing the traversable graph
  /// * connection: Traversable connection
  fn add_to_hash (hash: &mut HashMap<T, Vec<T>>, connection: (T, T)) {
    match hash.get_mut(&connection.0) {
      Some(destinations) => {
        destinations.push(connection.1);
      },
      None => {
        hash.insert(connection.0, vec![connection.1]);
      },
    }
    match hash.get_mut(&connection.1) {
      Some(destinations) => {
        destinations.push(connection.0);
      },
      None => {
        hash.insert(connection.1, vec![connection.0]);
      },
    }
  }

  /// Finds all paths between 2 points on the graph
  /// 
  /// # Arguments
  /// * start:    Starting point on the graph
  /// * end:      Ending point on the graph
  /// * cache:    Shared cache, shared between callback executions
  /// * path:     Currently searching path (When running should just pass vec![])
  /// * paths:    Paths found so far (When running should just pass vec![])
  /// * callback: Callback function, called for every point before moving to it, should return if point is allowed to be navigated to
  /// 
  /// # Returns
  /// All found paths
  pub fn find_all_paths<C> (&self, start: T, end: T, cache: &mut C, path: &mut Vec<T>, paths: &mut Vec<Vec<T>>, callback: fn(key: &T, path: &mut Vec<T>, cache: &mut C) -> bool) {
    // Check if allowed
    if callback(&start, path, cache) {
      // Append current path
      path.push(start);
      // If reached end, add to paths
      if start == end {
        paths.push(path.clone());
      }
      // If not reached end, continue to all possible connections
      else {
        let destinations = self.connections.get(&start).unwrap();
        for i in 0..destinations.len() {
          self.find_all_paths(destinations[i], end, cache, &mut path.clone(), paths, callback);
        }
      }
    }
  }

}
