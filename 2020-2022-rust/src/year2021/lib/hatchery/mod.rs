//! Hatchery module
//! 
//! Implements a model of exponential (fish) breeding/growth
// -----------------------------------------------------------------------------

/// Hatchery structure
pub struct Hatchery {
  pub maturity_interval_len: usize,
  pub reproduction_interval_len: usize,
  pub reproduction_factor: usize,
  pub population: Vec<usize>
}
/// Hatchery implementation
impl Hatchery {

  /// Constructor
  /// 
  /// # Arguments
  /// * maturity_interval_len:      How long it takes for a unit to mature to reproduction age
  /// * reproduction_interval_len:  How long the reproduction cycle takes
  /// * reproduction_factor:        How many new units every unit produces at the end of it's reproduction cycle
  pub fn new (maturity_interval_len: usize, reproduction_interval_len: usize, reproduction_factor: usize) -> Hatchery {
    Hatchery {
      maturity_interval_len,
      reproduction_interval_len,
      reproduction_factor,
      population: Vec::with_capacity(reproduction_interval_len + maturity_interval_len)
    }
  }

  /// Populates the hatchery with units in different stages of their maturation and reproduction cycles
  /// 
  /// # Arguments
  /// * data: Vector of fish intervals of where they are in their cycle
  pub fn populate (&mut self, data:Vec<usize>) {
    // Reset population
    for i in 0..self.population.len() {
      self.population[i] = 0;
    }
    for _ in self.population.len()..(self.reproduction_interval_len + self.maturity_interval_len) {
      self.population.push(0);
    }
    // Aggregate fish by interval
    for i in 0..data.len() {
      self.population[data[i]] += 1;
    }
  }

  /// Progresses the model by single time unit
  pub fn tick (&mut self) {
    // Store reproduction ready units
    let overflow = self.population[0];
    // Forward reproduction cycle
    for i in 0..self.reproduction_interval_len {
      self.population[i] = self.population[i + 1];
    }
    // Forward maturation cycle
    for i in self.reproduction_interval_len..(self.reproduction_interval_len + self.maturity_interval_len - 1) {
      self.population[i] = self.population[i + 1];
    }
    // Breed reproduction ready units
    self.population[self.reproduction_interval_len + self.maturity_interval_len - 1] = overflow;
    // Reset reproduction ready units
    self.population[self.reproduction_interval_len - 1] += overflow;
  }

  /// Counts total fish in hatchery
  pub fn len (&self) -> usize {
    self.population.iter().sum()
  }

}
