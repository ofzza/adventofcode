//! Sparse point cloud module
//! 
//! N-dimensional sparse point cloud implementation
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::*;

/// Sparse point cloud struct
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct SparsePointCloud {
  pub dimensionality: usize,
  pub points: Vec<(Vec<isize>, bool)>
}
/// Sparse point cloud implementation
impl SparsePointCloud {

  /// Constructor
  /// 
  /// # Arguments
  /// * points: Points making up the point cloud
  pub fn new(points: &Vec<Vec<isize>>) -> SparsePointCloud {
    // Fill with points and find dimensionality
    let mut dimensionality = 0;
    let mut processed_points: Vec<(Vec<isize>, bool)> = Vec::with_capacity(points.len());
    for i in 0..points.len() {
      if points[i].len() > dimensionality { dimensionality = points[i].len(); }
      processed_points.push((points[i].clone(), false));
    };
    // Add origin point
    let mut center: Vec<isize> = Vec::with_capacity(dimensionality);
    for _ in 0..dimensionality { center.push(0); }
    processed_points.push(
      (center, true)
    );
    // Initialize point cloud
    SparsePointCloud {
      dimensionality,
      points: processed_points
    }
  }

  /// Gets all origin points' coordinates
  /// 
  /// Returns
  /// All origin points' coordinates
  pub fn origins (&self) -> Vec<Vec<isize>> {
    self.points.iter().filter(|p| p.1).map(|p| p.0.clone()).collect()
  }

  /// Gets length of points array contained in the points cloud
  /// 
  /// # Returns
  /// Length of points array contained in the points cloud
  pub fn len (&self) -> usize {
    self.points.iter().filter(|p| !p.1).map(|p| p.0.clone()).collect::<Vec<Vec<isize>>>().len()
  }

}

/// Enumerates avaliable point cloud transformations
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum SparsePointCloudTransformation {
  TranslateToOrigin(Vec<isize>),
  RotateCW90(usize, usize),
  RotateCW180(usize, usize),
  RotateCW270(usize, usize)
}

/// Transformed sparse point cloud struct
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TransformedSparsePointCloud {
  pub cloud: SparsePointCloud,
  pub transformations: Vec<SparsePointCloudTransformation>
}
/// Transformed sparse point cloud implementation
impl TransformedSparsePointCloud {
  
  /// Constructor
  /// 
  /// # Arguments
  /// * cloud: Cloud to describe transformations for
  pub fn new (cloud: SparsePointCloud) -> TransformedSparsePointCloud {
    TransformedSparsePointCloud {
      cloud,
      transformations: vec![]
    }
  }

  /// Generates a new point cloud by running a transformation on an existing point cloud
  /// 
  /// # Arguments
  /// * cloud:          Original point cloud to transform
  /// * transformation: Transformation to run on the cloud
  /// 
  /// # Returns
  /// Transformed point cloud
  pub fn transform(tcloud: &TransformedSparsePointCloud, transformation: SparsePointCloudTransformation) -> TransformedSparsePointCloud {
    // Transform the cloud
    match transformation {
      // Translate to origin
      SparsePointCloudTransformation::TranslateToOrigin(point) => TransformedSparsePointCloud::translate_to_origin(tcloud, &point),
      // 90 degree clock-wise rotation
      SparsePointCloudTransformation::RotateCW90(first, second) => TransformedSparsePointCloud::transform_rotate_90cw(tcloud, first, second),
      // 180 degree clock-wise rotation
      SparsePointCloudTransformation::RotateCW180(first, second) => TransformedSparsePointCloud::transform_rotate_180cw(tcloud, first, second),
      // 270 degree clock-wise rotation
      SparsePointCloudTransformation::RotateCW270(first, second) => TransformedSparsePointCloud::transform_rotate_270cw(tcloud, first, second)
    }
  }
  /// Performs a clock-wise 90 degree rotation on a point cloud
  /// 
  /// # Arguments
  /// * cloud:  Cloud to transform
  /// * first:  First dimension of the plain of rotation
  /// * second: Second dimension of the plain of rotation
  /// 
  /// Returns
  /// Transformed point cloud
  fn transform_rotate_90cw(tcloud: &TransformedSparsePointCloud, first: usize, second: usize) -> TransformedSparsePointCloud {
    // Initialize transformed cloud
    let mut transformed_cloud = tcloud.cloud.clone();
    // Transform
    for i in 0..tcloud.cloud.points.len() {
      transformed_cloud.points[i].0[first] = tcloud.cloud.points[i].0[second];
      transformed_cloud.points[i].0[second] = -1 * tcloud.cloud.points[i].0[first];
    }
    // Return new transformed cloud
    let mut transformations = tcloud.transformations.clone();
    transformations.push(SparsePointCloudTransformation::RotateCW180(first, second));
    TransformedSparsePointCloud {
      cloud: transformed_cloud,
      transformations
    }
  }
  /// Performs a clock-wise 180 degree rotation on a point cloud
  /// 
  /// # Arguments
  /// * cloud: Cloud to transform
  /// * first:  First dimension of the plain of rotation
  /// * second: Second dimension of the plain of rotation
  /// 
  /// Returns
  /// Transformed point cloud
  fn transform_rotate_180cw(tcloud: &TransformedSparsePointCloud, first: usize, second: usize) -> TransformedSparsePointCloud {
    // Initialize transformed cloud
    let mut transformed_cloud = tcloud.cloud.clone();
    // Transform
    for i in 0..tcloud.cloud.points.len() {
      transformed_cloud.points[i].0[first] = -1 * tcloud.cloud.points[i].0[first];
      transformed_cloud.points[i].0[second] = -1 * tcloud.cloud.points[i].0[second];
    }
    // Return new transformed cloud
    let mut transformations = tcloud.transformations.clone();
    transformations.push(SparsePointCloudTransformation::RotateCW180(first, second));
    TransformedSparsePointCloud {
      cloud: transformed_cloud,
      transformations
    }
  }
  /// Performs a clock-wise 270 degree rotation on a point cloud
  /// 
  /// # Arguments
  /// * cloud: Cloud to transform
  /// * first:  First dimension of the plain of rotation
  /// * second: Second dimension of the plain of rotation
  /// 
  /// Returns
  /// Transformed point cloud
  fn transform_rotate_270cw(tcloud: &TransformedSparsePointCloud, first: usize, second: usize) -> TransformedSparsePointCloud {
    // Initialize transformed cloud
    let mut transformed_cloud = tcloud.cloud.clone();
    // Transform
    for i in 0..tcloud.cloud.points.len() {
      transformed_cloud.points[i].0[first] = -1 * tcloud.cloud.points[i].0[second];
      transformed_cloud.points[i].0[second] = tcloud.cloud.points[i].0[first];
    }
    // Return new transformed cloud
    let mut transformations = tcloud.transformations.clone();
    transformations.push(SparsePointCloudTransformation::RotateCW270(first, second));
    TransformedSparsePointCloud {
      cloud: transformed_cloud,
      transformations
    }
  }

  /// Performs a translation on a point cloud to align it to an origin point
  /// 
  /// # Arguments
  /// cloud:  Point cloud to translate to origin
  /// origin: Point to set as origin
  /// 
  /// # Returns
  /// Transformed point lous
  pub fn translate_to_origin(tcloud: &TransformedSparsePointCloud, origin: &Vec<isize>) -> TransformedSparsePointCloud {
    // Initialize transformed cloud
    let mut transformed_cloud = tcloud.cloud.clone();
    // Transform
    for i in 0..tcloud.cloud.points.len() {
      for j in 0..tcloud.cloud.dimensionality {
        transformed_cloud.points[i].0[j] = tcloud.cloud.points[i].0[j] - origin[j];
      }
    }
    // Return new transformed cloud
    let mut transformations = tcloud.transformations.clone();
    transformations.push(SparsePointCloudTransformation::TranslateToOrigin(origin.clone()));
    TransformedSparsePointCloud {
      cloud: transformed_cloud,
      transformations
    }
  }

  /// Merges two point clouds by finding a common points and producing a new point cloud with just those points
  /// 
  /// # Arguments
  /// first:              First point cloud to merge
  /// second:             Second point cloud to merge
  /// candidate_origins:  Optional potential matching origins produced by the TransformedSparsePointCloudCollection::quick_test_merge algorithm
  /// threshold:          Optional threshold of matching points which, if found, will be considered good enough and early exit
  /// 
  /// # Returns
  /// Tuple consisting of:
  /// - Resulting, merged transformed sparse point cloud
  /// - Number of merged points
  /// - First point cloud, transformed as requiredfor the merge
  /// - Second point cloud, transformed as requiredfor the merge
  pub fn merge(first: &TransformedSparsePointCloud, first_origins: Option<Vec<usize>>, second: &TransformedSparsePointCloud, second_origins: Option<Vec<usize>>, threshold: Option<usize>) -> (TransformedSparsePointCloud, usize, TransformedSparsePointCloud, TransformedSparsePointCloud) {
    // Initialize best overlap cloud
    let mut merged: SparsePointCloud = SparsePointCloud::new(&vec![]);
    merged.dimensionality = first.cloud.dimensionality;
    merged.points = first.cloud.points.clone();
    merged.points.extend(second.cloud.points.clone());
    let mut merged_normalized_first: Option<TransformedSparsePointCloud> = None;
    let mut merged_normalized_second: Option<TransformedSparsePointCloud> = None;
    let mut fit: usize = 0;
    // Initialize hashmap used to detect overlaps
    let mut hash: HashMap<Vec<isize>, bool> = HashMap::with_capacity(first.cloud.points.len());
    
    // Ready point pairs to test
    let first_points = match first_origins {
      Some(origins) => if origins.len() < first.cloud.points.len() {
        origins.iter().map(|i| first.cloud.points[i.clone()].clone()).collect::<Vec<(Vec<isize>, bool)>>()
      } else {
        first.cloud.points.clone()
      },
      None => first.cloud.points.clone()
    };
    let second_points = match second_origins {
      Some(origins) => if origins.len() < second.cloud.points.len() {
        origins.iter().map(|i| second.cloud.points[i.clone()].clone()).collect::<Vec<(Vec<isize>, bool)>>()
      } else {
        second.cloud.points.clone()
      },
      None => second.cloud.points.clone()
    };

    // Choose a points from both clouds to overlap and see how many other points match
    for i in 0..first_points.len() {
      if first_points[i].1 { continue; }
      for j in 0..second_points.len() {
        if second_points[j].1 { continue; }
        
        // Normalize clouds to the selected point
        let normalized_first = TransformedSparsePointCloud::transform(&first, SparsePointCloudTransformation::TranslateToOrigin(first_points[i].0.clone()));
        let normalized_first_cloud = &normalized_first.cloud;
        let normalized_second = TransformedSparsePointCloud::transform(&second, SparsePointCloudTransformation::TranslateToOrigin(second_points[j].0.clone()));
        let normalized_second_cloud = &normalized_second.cloud;
        
        // Reset overlap hash
        hash.clear();

        // Reduce matching points
        let mut points: Vec<(Vec<isize>, bool)> = vec![];
        for k in 0..normalized_first_cloud.points.len() {
          if !normalized_first_cloud.points[k].1 {
            hash.insert(normalized_first_cloud.points[k].0.clone(), true);
          }
          points.push(normalized_first_cloud.points[k].clone());
        }
        for k in 0..normalized_second_cloud.points.len() {
          if !hash.contains_key(&normalized_second_cloud.points[k].0) {
            points.push(normalized_second_cloud.points[k].clone());
          }
        }

        // If fewer points remaining than previous merges accept the merge
        if points.len() < merged.points.len() {
          merged.points = points;
          fit = &normalized_first.cloud.len() + &normalized_second.cloud.len() - &merged.len();
          merged_normalized_first = Some(normalized_first);
          merged_normalized_second = Some(normalized_second);
        }

        // Test treshold
        match threshold {
          Some(threshold) => {
            if fit >= threshold {
              // Return merged cloud
              return (TransformedSparsePointCloud::new(merged), fit, merged_normalized_first.unwrap(), merged_normalized_second.unwrap());
            }
          },
          None => ()
        }

      }
    }

    // Return merged cloud
    (TransformedSparsePointCloud::new(merged), fit, merged_normalized_first.unwrap(), merged_normalized_second.unwrap())
  }
}

// Sparse point cloud distances struct
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct SparsePointCloudDistances {
  distances: Vec<(Vec<usize>, usize, usize)>
}
// Sparse point cloud distances implementation
impl SparsePointCloudDistances {
  /// Constructor
  /// 
  /// # Arguemnts
  /// * cloud: CLoud for which to take distances
  pub fn new (cloud: &SparsePointCloud) -> SparsePointCloudDistances {
    // Initialize distances
    let mut distances: Vec<(Vec<usize>, usize, usize)> = Vec::with_capacity((cloud.points.len() * cloud.points.len()) / 2 + 1);
    // Calculate distances
    for i in 0..(cloud.points.len() - 1) {
      if cloud.points[i].1 { continue; }
      for j in (i + 1)..cloud.points.len() {
        if cloud.points[j].1 { continue; }
        let mut distance: Vec<usize> = Vec::with_capacity(cloud.dimensionality);
        for k in 0..cloud.dimensionality {
          distance.push((cloud.points[i].0[k] - cloud.points[j].0[k]).abs() as usize);
        }
        distance.sort();
        distances.push((distance, i, j));
      }
    }
    // Sort distances
    distances.sort_by(|a: &(Vec<usize>, usize, usize), b: &(Vec<usize>, usize, usize)| { a.0.partial_cmp(&b.0).unwrap() });
    // Return distances
    SparsePointCloudDistances { distances }
  }
}

/// Pre-transformed sparse point cloud colection struct
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TransformedSparsePointCloudCollection {
  pub clouds: Vec<TransformedSparsePointCloud>,
  pub distances: SparsePointCloudDistances
}
/// Pre-transformed sparse point cloud colection implementation
impl TransformedSparsePointCloudCollection {
  /// Constructor
  /// Generates all possible transformations of a sparse point cloud
  /// 
  /// # Arguments
  /// * cloud: Original point cloud to transform
  /// 
  /// # Returns
  /// All possible transformations of a sparse point cloud
  pub fn new(tcloud: TransformedSparsePointCloud) -> TransformedSparsePointCloudCollection {
    // Initialize transformations
    let dimensionality = tcloud.cloud.dimensionality;
    let distances = SparsePointCloudDistances::new(&tcloud.cloud);
    let mut collection = TransformedSparsePointCloudCollection {
      clouds: vec![tcloud],
      distances
    };

    // Generate all possible rotations plains
    let mut rotation_plains: Vec<(usize, usize)> = vec![];
    for i in 0..dimensionality {
      for j in (i + 1)..dimensionality {
        rotation_plains.push((i, j));
      }  
    }

    // Generate all possible transformations
    for _ in 0..(dimensionality - 1) {
      let previous_step_clouds_count = collection.clouds.len();
      for i in 0..previous_step_clouds_count {
        collection.clouds.push(collection.clouds[i].clone());
        for j in 0..rotation_plains.len() {
          collection.clouds.push(
            TransformedSparsePointCloud::transform(&collection.clouds[i], SparsePointCloudTransformation::RotateCW90(rotation_plains[j].0, rotation_plains[j].1))
          );
          collection.clouds.push(
            TransformedSparsePointCloud::transform(&collection.clouds[i], SparsePointCloudTransformation::RotateCW180(rotation_plains[j].0, rotation_plains[j].1))
          );
          collection.clouds.push(
            TransformedSparsePointCloud::transform(&collection.clouds[i], SparsePointCloudTransformation::RotateCW270(rotation_plains[j].0, rotation_plains[j].1))
          );
        }
      }
    }

    // Deduplicate transformations
    let mut hash: HashMap<String, bool> = HashMap::with_capacity(collection.clouds.len());
    for i in (0..collection.clouds.len()).rev() {
      let key = format!("{:?}", collection.clouds[i].cloud.points);
      if !hash.contains_key(&key) {
        hash.insert(key, true);
      } else {
        collection.clouds.remove(i);
      }
    }

    // Return all transformations
    collection
  }

  /// Performs a quick check if two sparse point clouds can be merged (regardless of their orientation) by using hteir distance metrics
  /// 
  /// # Arguments
  /// first:      First point cloud to merge
  /// second:     Second point cloud to merge
  /// 
  /// # Returns
  /// Vector of tuples of matching potential points (as ordinals in their respective clouds) that were matched together and are good
  /// candidates as translational points for merges
  pub fn quick_test_merge(first: &TransformedSparsePointCloudCollection, second: &TransformedSparsePointCloudCollection) -> Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> {
    // Initialize indexes and matches
    let mut matches: Vec<(Vec<usize>, Vec<usize>, Vec<usize>)> = vec![];
    // Compose matching distances
    for i in 0..first.distances.distances.len() {
      for j in 0..second.distances.distances.len() {
        if first.distances.distances[i].0 == second.distances.distances[j].0 {
          matches.push(
            (
              first.distances.distances[i].0.clone(),
              vec![first.distances.distances[i].1, first.distances.distances[i].2],
              vec![second.distances.distances[j].1, second.distances.distances[j].2],
            )
          );
        }
      }
    }
    // Return matches
    matches
  }
}
