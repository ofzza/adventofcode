//! Sparse point cloud module
//! 
//! N-dimensional sparse point cloud implementation
// -----------------------------------------------------------------------------

// Include dependencies
use std::collections::hash_map::*;

/// Enumerates availabel point cloud transformations
pub enum SparsePointCloudTransformation {
  TranslateToOrigin(Vec<isize>),
  RotateCW90(usize, usize),
  RotateCW180(usize, usize),
  RotateCW270(usize, usize)
}

/// Sparse point cloud struct
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct SparsePointCloud {
  pub dimensionality: usize,
  pub points: Vec<Vec<isize>>
}
/// Sparse point cloud implementation
impl SparsePointCloud {

  /// Constructor
  /// 
  /// # Arguments
  /// * points: Points making up the point cloud
  pub fn new(points: &Vec<Vec<isize>>) -> SparsePointCloud {
    let mut dimensionality = 0;
    let mut processed_points = Vec::with_capacity(points.len());
    for i in 0..points.len() {
      if points[i].len() > dimensionality { dimensionality = points[i].len(); }
      processed_points.push(points[i].clone());
    };
    // Initialize point cloud
    let cloud = SparsePointCloud {
      dimensionality,
      points: processed_points
    };
    // Fill with points and find dimensionality
    // Return point cloud
    cloud
  }

  /// Merges two point clouds by finding a common points and producing a new point cloud with just those points
  /// 
  /// # Arguments
  /// first:  First point cloud to merge
  /// second: Second point cloud to merge
  pub fn merge_clouds(first: &SparsePointCloud, second: &SparsePointCloud) -> (SparsePointCloud, Vec<isize>) {
    // Initialize best overlap cloud
    let mut merged: SparsePointCloud = SparsePointCloud::new(&vec![]);
    merged.dimensionality = first.dimensionality;
    merged.points = first.points.clone();
    merged.points.extend(second.points.clone());
    // Initialize hashmap used to detect overlaps
    let mut hash: HashMap<String, bool> = HashMap::with_capacity(first.points.len());
    // Choose a points from both clouds to overlap and see how many other points match
    let mut relative_cloud_offset: Vec<isize> = vec![];
    for i in 0..first.points.len() {
      for j in 0..second.points.len() {
        // Normalize clouds to the selected point
        let normalized_first = SparsePointCloud::transform(&first, SparsePointCloudTransformation::TranslateToOrigin(first.points[i].clone()));
        let normalized_second = SparsePointCloud::transform(&second, SparsePointCloudTransformation::TranslateToOrigin(second.points[j].clone()));
        // Reset overlap hash
        hash.clear();
        // Reduce matching points
        let mut points: Vec<Vec<isize>> = vec![];
        for k in 0..normalized_first.points.len() {
          hash.insert(format!("{:?}", normalized_first.points[k].clone()), true);
          points.push(normalized_first.points[k].clone());
        }
        for k in 0..normalized_second.points.len() {
          if !hash.contains_key(&format!("{:?}", normalized_second.points[k].clone())) {
            points.push(normalized_second.points[k].clone());
          }
        }
        if points.len() < merged.points.len() {
          merged.points = points;
          relative_cloud_offset = vec![];
          for k in 0..first.dimensionality {
            relative_cloud_offset.push(first.points[i][k] - second.points[j][k]);
          }
        }
      }
    }
    // Return merged cloud
    (merged, relative_cloud_offset)
  }

  /// Generates all possible transformations of a sparse point cloud
  /// 
  /// # Arguments
  /// * cloud: Original point cloud to transform
  /// 
  /// # Returns
  /// All possible transformations of a sparse point cloud
  pub fn get_all_inplace_transformations(cloud: &SparsePointCloud) -> Vec<SparsePointCloud> {
    // Initialize transformations
    let mut transformations: Vec<SparsePointCloud> = Vec::new();
    transformations.push(cloud.clone());
    // Generate all possible rotations plains
    let mut rotation_plains: Vec<(usize, usize)> = vec![];
    for i in 0..cloud.dimensionality {
      for j in (i + 1)..cloud.dimensionality {
        rotation_plains.push((i, j));
      }  
    }

    // Generate all possible transformations
    for _ in 0..cloud.dimensionality {
      let mut updated_transformations = transformations.clone();
      for k in 0..transformations.len() {
        updated_transformations.push(transformations[k].clone());
      }
      for j in 0..rotation_plains.len() {
        for k in 0..transformations.len() {
          updated_transformations.push(
            SparsePointCloud::transform(&transformations[k], SparsePointCloudTransformation::RotateCW90(rotation_plains[j].0, rotation_plains[j].1))
          );
          updated_transformations.push(
            SparsePointCloud::transform(&transformations[k], SparsePointCloudTransformation::RotateCW180(rotation_plains[j].0, rotation_plains[j].1))
          );
          updated_transformations.push(
            SparsePointCloud::transform(&transformations[k], SparsePointCloudTransformation::RotateCW270(rotation_plains[j].0, rotation_plains[j].1))
          );
        }
      }
      transformations = updated_transformations;
    }

    // Return all transformations
    transformations
  }

  /// Generates a new point cloud by running a transformation on an existing point cloud
  /// 
  /// # Arguments
  /// * cloud:          Original point cloud to transform
  /// * transformation: Transformation to run on the cloud
  /// 
  /// # Returns
  /// Transformed point cloud
  pub fn transform(cloud: &SparsePointCloud, transformation: SparsePointCloudTransformation) -> SparsePointCloud {
    // Create a new copy of the cloud
    let mut transformed = cloud.clone();
    // Transform the cloud
    match transformation {
      // Translate to origin
      SparsePointCloudTransformation::TranslateToOrigin(point) => {
        transformed = SparsePointCloud::translate_to_origin(transformed, &point);
      },
      // 90 degree clock-wise rotation
      SparsePointCloudTransformation::RotateCW90(first, second) => {
        transformed = SparsePointCloud::transform_rotate_90cw(transformed, first, second);
      },
      // 180 degree clock-wise rotation
      SparsePointCloudTransformation::RotateCW180(first, second) => {
        transformed = SparsePointCloud::transform_rotate_180cw(transformed, first, second);
      },
      // 270 degree clock-wise rotation
      SparsePointCloudTransformation::RotateCW270(first, second) => {
        transformed = SparsePointCloud::transform_rotate_270cw(transformed, first, second);
      }
    }
    // Return transformed cloud
    transformed
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
  fn transform_rotate_90cw(mut cloud: SparsePointCloud, first: usize, second: usize) -> SparsePointCloud {
    for i in 0..cloud.points.len() {
      let value_first: isize = cloud.points[i][second];
      let value_second: isize = -1 * cloud.points[i][first];
      cloud.points[i][first] = value_first;
      cloud.points[i][second] = value_second;
    }
    cloud
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
  fn transform_rotate_180cw(mut cloud: SparsePointCloud, first: usize, second: usize) -> SparsePointCloud {
    for i in 0..cloud.points.len() {
      let value_first: isize = -1 * cloud.points[i][first];
      let value_second: isize = -1 * cloud.points[i][second];
      cloud.points[i][first] = value_first;
      cloud.points[i][second] = value_second;
    }
    cloud
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
  fn transform_rotate_270cw(mut cloud: SparsePointCloud, first: usize, second: usize) -> SparsePointCloud {
    for i in 0..cloud.points.len() {
      let value_first: isize = -1 * cloud.points[i][second];
      let value_second: isize = cloud.points[i][first];
      cloud.points[i][first] = value_first;
      cloud.points[i][second] = value_second;
    }
    cloud
  }

  /// Performs a translation on a point cloud to align it to an origin point
  /// 
  /// # Arguments
  /// cloud:  Point cloud to translate to origin
  /// origin: Point to set as origin
  /// 
  /// # Returns
  /// Transformed point lous
  pub fn translate_to_origin(mut cloud: SparsePointCloud, origin: &Vec<isize>) -> SparsePointCloud {
    for i in 0..cloud.points.len() {
      for j in 0..cloud.dimensionality {
        cloud.points[i][j] -= origin[j];
      }
    }
    cloud
  }

  /// Gets length of points array contained in the points cloud
  /// 
  /// # Returns
  /// Length of points array contained in the points cloud
  pub fn len (&self) -> usize {
    self.points.len()
  }

}
