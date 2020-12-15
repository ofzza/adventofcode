//! Puzzle statistics struct
// -----------------------------------------------------------------------------

/// Puzzle statistics struct
/// 
/// TODO: more details ...
#[derive(Debug, Default)]
pub struct PuzzleExecutionStatistics {
  pub total_count: u32,
  pub successful_count: u32,
  pub failed_count: u32,
  pub execution_time: f32
}

/// Puzzle statistics .update() implementation
/// 
/// Allows owner to be run by calling the .run(verbose: bool) method
impl PuzzleExecutionStatistics {

  /// Update with values from passed statistics instance
  /// 
  /// # Arguments
  /// 
  /// - `stats` - Statistics instance to update from
  pub fn update (&mut self, stats: PuzzleExecutionStatistics) {
    self.total_count      += &stats.total_count;
    self.successful_count += &stats.successful_count;
    self.failed_count     += &stats.failed_count;
    self.execution_time   += &stats.execution_time;
  }

}
