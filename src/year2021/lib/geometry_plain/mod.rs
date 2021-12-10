//! Geometry lines module
//! 
//! Implements geometry for lines on a plain
// -----------------------------------------------------------------------------

// Import dependencies
use std::cmp::*;
use std::collections::hash_map::*;

/// Geometry point struct
pub struct GeometryPoint {
  pub x: isize,
  pub y: isize
}

/// Geometry line struct
pub struct GeometryLine {
  pub start: GeometryPoint,
  pub end: GeometryPoint,
  pub bounds: (GeometryPoint, GeometryPoint), 
  pub k: f64,
  pub b: f64
}
/// Geometry line implementation
impl GeometryLine {
  /// Constructor
  /// 
  /// # Arguments
  /// * x1 Starting x coordinate
  /// * y1 Starting y coordinate
  /// * x1 Ending x coordinate
  /// * y1 Ending y coordinate
  pub fn new (x1: isize, y1: isize, x2: isize, y2: isize) -> GeometryLine {
    // Normalize line
    let x_min = *vec![x1, x2].iter().min().unwrap();
    let x_max = *vec![x1, x2].iter().max().unwrap();
    let y_min = *vec![y1, y2].iter().min().unwrap();
    let y_max = *vec![y1, y2].iter().max().unwrap();
    let bounds = (
      GeometryPoint { x: x_min, y: y_min },
      GeometryPoint { x: x_max, y: y_max }
    );
    let start = GeometryPoint {
      x: x_min,
      y: if x1 == x2 { y_min } else if x_min == x1 { y1 } else { y2 }
    };
    let end = GeometryPoint {
      x: x_max,
      y: if x1 == x2 { y_max } else if x_max == x1 { y1 } else { y2 }
    };
    let (k, b) = if x1 != x2 {
      let k = (end.y as f64 - start.y as f64) / (end.x as f64 - start.x as f64).abs();
      let b = start.y as f64 - (start.x as f64 * k);
      (k, b)
    } else {
      (f64::INFINITY, f64::INFINITY)
    };
    // Compose normalized line
    GeometryLine {
      start,
      end,
      bounds,
      k,
      b
    }
  }

  /// Checks if lines are colinear and returns overlap if exists
  /// 
  /// # Arguments
  /// * line1: First line
  /// * line2: Second line
  /// 
  /// # Returns
  /// Overlap line if overlap exists
  pub fn find_colinear_overlap (line1: &GeometryLine, line2: &GeometryLine) -> Option<GeometryLine> {
    // If line colinear, find overlap
    if line1.k == line2.k && if line1.k != f64::INFINITY { line1.b == line2.b } else { line1.start.x == line2.start.x } {
      let min_x = *vec![line1.bounds.0.x, line2.bounds.0.x].iter().max().unwrap();
      let max_x = *vec![line1.bounds.1.x, line2.bounds.1.x].iter().min().unwrap();
      let min_y = *vec![line1.bounds.0.y, line2.bounds.0.y].iter().max().unwrap();
      let max_y = *vec![line1.bounds.1.y, line2.bounds.1.y].iter().min().unwrap();
      // Check if overlapping
      if min_x <= line1.bounds.1.x && min_x <= line2.bounds.1.x && max_x >= line1.bounds.0.x && max_x >= line2.bounds.0.x && min_y <= line1.bounds.1.y && min_y <= line2.bounds.1.y && max_y >= line1.bounds.0.y && max_y >= line2.bounds.0.y {
        // Return overlap
        let overlap = GeometryLine::new(
          min_x,
          if line1.k == f64::INFINITY { min_y } else { (min_x as f64 * line1.k + line1.b) as isize },
          max_x,
          if line1.k == f64::INFINITY { max_y } else { (max_x as f64 * line1.k + line1.b) as isize }
        );
        Some(overlap)
      }
      // If not overlapping, return no overlap
      else { None }
    }
    // If not colinear, return no overlap
    else { None }
  }

  /// Checks if lines are intersecting and returns intersection if exists
  /// 
  /// # Arguments
  /// * line1: First line
  /// * line2: Second line
  /// 
  /// # Returns
  /// Intersection point if overlap exists
  pub fn find_intersection (line1: &GeometryLine, line2: &GeometryLine) -> Option<GeometryPoint> {
    // If lines are parallel, no intersection
    if line1.k == line2.k {
      None
    }
    // If lines aren't parallel, calculate intersection
    else {
      // Find intersection of lines if they were unbounded
      let (x, y) = if line1.k == f64::INFINITY {
        let x = line1.start.x as f64;
        let y = x * line2.k + line2.b;
        (x, y)
      } else if line2.k == f64::INFINITY {
        let x = line2.start.x as f64;
        let y = x * line1.k + line1.b;
        (x, y)
      } else {
        let x = (line2.b - line1.b) / (line1.k - line2.k);
        let y = (x * line1.k) + line1.b;
        (x, y)
      };
      // Check if intersection is a while number
      if x as isize as f64 != x && y as isize as f64 != y {
        return None
      }
      let x = x as isize;
      let y = y as isize;
      // Check if potential intersection is within both line segments
      if x >= line1.bounds.0.x && x <= line1.bounds.1.x && x >= line2.bounds.0.x && x <= line2.bounds.1.x && y >= line1.bounds.0.y && y <= line1.bounds.1.y && y >= line2.bounds.0.y && y <= line2.bounds.1.y {
        Some(GeometryPoint { x, y })
      } else {
        None
      }
    }
  }

  /// Checks if line is diagonal
  /// 
  /// # Returns
  /// If line is diagonal
  pub fn is_diagonal (&self) -> bool {
    return self.k != 0.0 && self.k != f64::INFINITY;
  }
}

/// Geometry plain structure
pub struct GeometryPlain {
  pub lines: Vec<GeometryLine>
}
/// Geometry plain implementation
impl GeometryPlain {

  /// Constructor
  /// 
  /// # Arguments
  /// * capacity: Count of expected lines in the plain
  pub fn new (capacity: usize) -> GeometryPlain {
    GeometryPlain {
      lines: Vec::with_capacity(capacity)
    }
  }

  /// Finds an intersection between 2 lines if one exists
  /// 
  /// # Arguments
  /// * line_1: First line
  /// * line_2: Second line
  /// 
  /// # Returns
  /// Intersection point if found
  fn find_intersection(line1: &GeometryLine, line2: &GeometryLine) -> Option<Vec<GeometryPoint>> {
    match GeometryLine::find_intersection(&line1, &line2) {
      Some(point) => Some(vec![point]),
      None => match GeometryLine::find_colinear_overlap(&line1, &line2) {
        // Calculate overlapping points
        Some(overlap) => {
          // Find overlapping points
          let mut points: Vec<GeometryPoint> = Vec::new();
          for i in 0..(max((overlap.start.x as isize - overlap.end.x as isize).abs() as usize, (overlap.start.y as isize - overlap.end.y as isize).abs() as usize) + 1) {
            let p = GeometryPoint {
              x: if overlap.start.x as isize == overlap.end.x as isize { overlap.start.x as isize } else { (overlap.start.x as isize) + (if overlap.start.x < overlap.end.x { 1 as isize } else { -1 as isize }) * (i as isize) },
              y: if overlap.start.y as isize == overlap.end.y as isize { overlap.start.y as isize } else { (overlap.start.y as isize) + (if overlap.start.y < overlap.end.y { 1 as isize } else { -1 as isize }) * (i as isize) }
            };
            points.push(p);
          }
          // Return overlapping points
          Some(points)
        },
        // Calculate intersection
        None => None
      }
    }
  }
  
  /// Adds a line to the geometry plain
  /// 
  /// # Arguments
  /// * x1: Starting point's x coordinate
  /// * y1: Starting point's y coordinate
  /// * x2: End point's x coordinate
  /// * y2: End point's y coordinate
  pub fn add_line (&mut self, x1: isize, y1: isize, x2: isize, y2: isize) {
    let line = GeometryLine::new(x1, y1, x2, y2);
    self.lines.push(line);
  }

  /// Finds all intersections between al llines
  /// 
  /// # Arguments
  /// * use_diagonals: If diagonal lines should be used
  /// 
  /// # Returns
  /// Vector of coordinates and numbers of lines intersecting on those coordinates
  pub fn find_intersections(&self, use_diagonals: bool) -> Vec<((isize, isize), usize)> {
    // Initialize hashmap for intersections counting
    let mut hashmap: HashMap<(isize, isize), usize> = HashMap::new();

    // Find intersections between all lines
    for i in 0..self.lines.len() {
      for j in (i + 1)..self.lines.len() {
        if use_diagonals || (!self.lines[i].is_diagonal() && !self.lines[j].is_diagonal()) {
          // Find intersection between lines geometrically
          match GeometryPlain::find_intersection(&self.lines[i], &self.lines[j]) {
            Some(intersections) => {
              for i in 0..intersections.len() {
                // Store intersection to hashmap
                let key = (intersections[i].x, intersections[i].y);
                if hashmap.contains_key(&key) {
                  let count = hashmap.get(&key).unwrap().clone();
                  hashmap.insert(key, count + 1);
                } else {
                  hashmap.insert(key, 1);
                }
              }
            },
            _ => ()
          }
        }
      }
    }

    // Return intersections
    hashmap.iter().map(|(point, count)| (point.clone(), count.clone())).collect::<Vec<((isize, isize), usize)>>()
  }

}
