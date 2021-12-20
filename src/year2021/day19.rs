//! 2021 day 19 puzzle
//! 
//! https://adventofcode.com/2021/day/19
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::*;
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

      println!("");
      println!("Total of {} beacons found!", merged.len());
      println!("");

      // Find relative offsets of all clouds compared to the merged version
      let clouds = create_clouds(&data);
      let mut relative_offsets: Vec<Vec<isize>> = vec![];
      for i in 0..clouds.len(){
        println!("Finding relative offset for cloud #{}", i);
        let transformations = SparsePointCloud::get_all_inplace_transformations(&clouds[i]);
        for k in 0..transformations.len() {
          let partial_merged = SparsePointCloud::merge_clouds(&merged, &transformations[k]);
          if partial_merged.0.len() == merged.len() {
            println!("  ... found: {:?}", partial_merged.1);
            relative_offsets.push(partial_merged.1);
            break;
          }
        }
      }

      // Find largest relative offset
      println!("");
      let mut max_manhattan_distance: usize = 0;
      for i in 0..relative_offsets.len() {
        for j in (i+1)..relative_offsets.len() {
          let mut manhattan_distance: usize = 0;
          for k in 0..relative_offsets[i].len() {
            manhattan_distance += (relative_offsets[i][k] - relative_offsets[j][k]).abs() as usize;
          }
          println!("Manhattan distance between #{} ({:?}) and #{} ({:?}) = {}", i, relative_offsets[i], j, relative_offsets[j], manhattan_distance);
          if manhattan_distance > max_manhattan_distance {
            max_manhattan_distance = manhattan_distance;
          }
        }
      }
      
      // Calculate and return result
      String::from(format!("{:?}", max_manhattan_distance))
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
  // Initialize clouds<>
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
/// A single point cloud with all the data merged into it
fn merge_all (clouds: &Vec<SparsePointCloud>) -> SparsePointCloud {

  // Find transformations
  let mut clouds: Vec<(SparsePointCloud, Vec<SparsePointCloud>, bool)> = clouds.iter().map(|c| {
    let transformations = SparsePointCloud::get_all_inplace_transformations(&c);
    (c.clone(), transformations, false)
  }).collect();

  // Test best match for scanners
  let mut hash: HashMap<(SparsePointCloud, SparsePointCloud), SparsePointCloud> = HashMap::new();
  while clouds.len() > 1 {

    // Merge all couds that can be merged
    for i in 0..clouds.len() {
      if clouds[i].2 { continue; }
      for j in 0..clouds.len() {
        if clouds[i].2 { continue; }
        if clouds[j].2 { continue; }
        if i == j { continue; }

        for k in 0..clouds[j].1.len() {
          
          if clouds[i].2 == true || clouds[j].2 == true {
            print!("");
          }

          // Test merge two clouds (use cache if possible)
          let key = (clouds[i].0.clone(), clouds[j].1[k].clone());
          let merged = if hash.contains_key(&key) {
            hash.get(&key).unwrap().clone()
          } else {
            let merged = SparsePointCloud::merge_clouds(&clouds[i].0, &clouds[j].1[k]).0;
            hash.insert(key, merged.clone());
            merged
          };

          // Check fit and keep merge if fit good enough
          let fit = clouds[i].0.len() + clouds[j].1[k].len() - merged.len();
          if fit >= 12 {
            clouds[i].2 = true;
            clouds[j].2 = true;
            let transformations = SparsePointCloud::get_all_inplace_transformations(&merged);
            clouds.push((merged, transformations, false));
            println!(">>> MERGED {} and {} - fit: {}", i, j, fit);
            println!("    REMAINING: {}", clouds.iter().filter(|c| !c.2).collect::<Vec<&(SparsePointCloud, Vec<SparsePointCloud>, bool)>>().len());
            break;
          }
        }
      }
    }

    // Prune already merged clouds
    for i in (0..clouds.len()).rev() {
      if clouds[i].2 {
        clouds.remove(i);
      }
    }
  }

  clouds[0].0.clone()
}
