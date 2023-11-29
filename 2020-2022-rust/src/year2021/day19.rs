//! 2021 day 19 puzzle
//! 
//! https://adventofcode.com/2021/day/19
// -----------------------------------------------------------------------------

// Include dependencies
use crate::lib::puzzle::*;
use crate::lib::input::*;
use crate::year2021::lib::sparse_point_cloud::*;

/// Parses input data
fn parse(data: &String) -> Vec<Vec<Vec<isize>>> {
  Input::parse(data.trim(), "\n\n", |scanner| {
    Input::parse(scanner.trim(), "\n", |line| {
      Input::parse(line.trim(), ",", |n| {
        match n.parse::<isize>() {
          Ok(n) => n,
          _ => 0
        }
      })
    })[1..].to_vec()
  })
}

/// Registers puzzles for the day
pub fn init (mut registry: PuzzleRegistry) -> PuzzleRegistry {
  
  // Part I
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 19,
      index: 1,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);
      
      // Merge all data intoa single point cloud
      let clouds = create_clouds(&data);
      let merged = merge_all(&clouds);

      // Calculate and return result
      String::from(format!("{:?}", merged.len()))
    }

  );

  // Part II
  registry.register(

    // Info
    PuzzleInfo {
      year: 2021,
      day: 19,
      index: 2,
      tag: String::from("puzzle")
    },

    // Implementation
    |data: String| {
      // Process input data
      let data = parse(&data);

      // Merge all data intoa single point cloud
      let clouds = create_clouds(&data);
      let merged = merge_all(&clouds);
      let origins = merged.origins();

      // Find largest distance between origin points
      let mut max_manhattan_dist: usize = 0;
      for i in 0..(origins.len() - 1) {
        for j in i..origins.len() {
          let mut manhattan_dist: usize = 0;
          for k in 0..3 {
            manhattan_dist += (origins[i][k] - origins[j][k]).abs() as usize;
          }
          if manhattan_dist > max_manhattan_dist {
            max_manhattan_dist = manhattan_dist;
          }
        }
      }

      
      // Calculate and return result
      String::from(format!("{:?}", max_manhattan_dist))
    }

  );

  // Return registry ownership
  registry
}

/// Creates point clouds from input data
/// 
/// # Arguments
/// * data: Input data
/// 
/// # Returns
/// Vector of created coint clouds
fn create_clouds (data: &Vec<Vec<Vec<isize>>>) -> Vec<SparsePointCloud> {
  // Initialize clouds
  let mut clouds: Vec<SparsePointCloud> = Vec::with_capacity(data.len());
  // Create point clouds
  for i in 0..data.len() {
    let cloud = SparsePointCloud::new(&data[i]);
    clouds.push(cloud);
  }
  // Return clouds
  clouds
}

/// Converts data into point clouds and merges everything it can
/// 
/// # Arguments
/// * data: Input data
/// 
/// # Returns
/// A tuple consisting of:
/// - A single point cloud with all the data merged into it
/// - Sparse cloud collection of all transformed clouds transformed in a way they were when merged
fn merge_all (clouds: &Vec<SparsePointCloud>) -> SparsePointCloud {

  // Get all transformations for each cloud
  let mut tcollections: Vec<(TransformedSparsePointCloudCollection, bool)> = clouds.iter()
    .map(|c| { (TransformedSparsePointCloudCollection::new(TransformedSparsePointCloud::new(c.clone())), false) })
    .collect();

  // Keep track of merged clouds and their transformations
  while tcollections.len() > 1 {

    // Find best quick test merge candidates
    let mut merge_found = false;
    for i in 0..(tcollections.len() - 1) {
      if tcollections[i].1 { continue; }
      for j in (i + 1)..tcollections.len() {
        if tcollections[j].1 { continue; }

        // Quick check if merge is possible
        let quick_check = TransformedSparsePointCloudCollection::quick_test_merge(&tcollections[i].0, &tcollections[j].0);
        println!("... quick check merge {}x{} with quick check result: {} > 77?", i, j, quick_check.len());
        if quick_check.len() < (12 * 11 / 2) { continue; }

        // Try merging any of the transformations around each of the quick cehck candidates
        let mut break_merge_attempt = false;
        for k in (0..quick_check.len()).rev() {
          if break_merge_attempt { break; }
          for l in 0..tcollections[j].0.clouds.len() {
            if break_merge_attempt { break; }

            // Try merging clouds
            let merge_attempt = TransformedSparsePointCloud::merge(
              &tcollections[i].0.clouds[0], Some(quick_check[k].1.clone()),
              &tcollections[j].0.clouds[l], Some(quick_check[k].2.clone()),
              Some(12)
            );
            if merge_attempt.1 >= 12 {

              // Mark clouds as merged
              tcollections[i].1 = true;
              tcollections[j].1 = true;

              // Accept merge and add merged cloud
              tcollections.insert(0, (TransformedSparsePointCloudCollection::new(merge_attempt.0), false));
              break_merge_attempt = true;
              merge_found = true;

              // Prompt status
              println!(">>> MERGED:    {} ({:?}) + {} ({:?}) - fit: {}", i, &merge_attempt.2.transformations, j, &merge_attempt.3.transformations, merge_attempt.1);
              // println!("    Quick distance: {} / {}", k + 1, quick_check.len());
              println!("    REMAINING: {}", tcollections.iter().filter(|c| !c.1).collect::<Vec<&(TransformedSparsePointCloudCollection, bool)>>().len());
            }
          }
        }

      }
    }

    // Remove merged collections
    for i in (0..tcollections.len()).rev() {
      if tcollections[i].1 { tcollections.remove(i); }
    }

    // Check infinite loop if failed finding merges
    if tcollections.len() > 0 && !merge_found {
      panic!("No merge found with {} clouds remaining!", tcollections.len());
    }

  }

  // Return fully merged cloud
  tcollections[0].0.clouds[0].cloud.clone()
}
