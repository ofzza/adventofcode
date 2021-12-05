//! Diagnostic report decoding module
//! 
//! Provides quick, customizable decoding of diagnostic report messages
// -----------------------------------------------------------------------------

// Include dependencies
use trie_rs::*;

// Diagnostic message structure
#[derive(Clone)]
pub struct DiagnosticMessage {
  /// Diagnostic message in binary
  pub code: Vec<bool>
}
// Diagnostic message implementation
impl DiagnosticMessage {
  /// Represents the diagnostic message as a usize number
  /// 
  /// # Returns
  /// Diagnostic message in usize representation
  pub fn as_usize (&self) -> usize{
    return usize::from_str_radix(&self.code.iter().map(|b: &bool| { if b.clone() { "1" } else { "0" } }).collect::<Vec<&str>>().join(""), 2).unwrap();
  }
}

/// Diagnostic report structure
pub struct DiagnosticReport {
  /// Diagnostic messages
  _messages: Vec<DiagnosticMessage>,
  /// Diagnostic messages organized in a trie
  _trie: Option<Trie<bool>>
}
/// Diagnostic report implementation
impl<'a> DiagnosticReport {
  
  /// Constructor
  pub fn new (data: &Vec<Vec<bool>>) -> DiagnosticReport {
    let msgs = data.iter().map(|msg| DiagnosticMessage { code: msg.clone() }).collect();
    DiagnosticReport {
      _messages: msgs,
      _trie: None
    }
  }

  /// Initializes a trie representation of diagnostic messages
  /// 
  /// # Arguments
  /// * data: Puzzle input data
  fn compose_trie (&mut self) {
    let mut builder: TrieBuilder<bool> = TrieBuilder::new();
    for i in 0..self._messages.len() {
      builder.push(self._messages[i].code.to_vec());
    }
    self._trie = Some(builder.build());
  }

  /// Aggregates bits from all diagnostic messages
  /// 
  /// # Returns
  /// Vecotor of tuples counting "0" and "1" bits of all diagnostic messages
  pub fn aggregate (
    self
  ) -> Vec<(usize, usize)> {
    let mut aggregate: Vec<(usize, usize)> = self._messages[0].code.iter().map(|_| (0usize, 0usize)).collect();
    for i in 0..self._messages.len() {
      let message = &self._messages[i];
      for n in 0..message.code.len() {
        if !message.code[n] {
          aggregate[n].0 += 1;
        } else {
          aggregate[n].1 += 1;
        }
      }
    }
    aggregate
  }

  /// Runs a customizable processing loop over diagnostic messages structured in a binary trie
  /// 
  /// # Arguments
  /// * is_continue:  Callback executed on start of every loop cycle to determine if another cycle of the loop should run
  /// * do_update:    Callback used to mutate the state being processed
  /// 
  /// # Returns
  /// Final state after processing
  pub fn process (
    &mut self,
    is_continue: fn(step: &DiagnosticReportProcessingState) -> bool,
    do_update: fn(step: DiagnosticReportProcessingState, branches: (Vec<DiagnosticMessage>, Vec<DiagnosticMessage>)) -> DiagnosticReportProcessingState
  ) -> DiagnosticReportProcessingState {

    // Check if trie initialized
    match self._trie {
      None => {
        self.compose_trie()
      },
      _ => ()
    }

    // Set initial processing state
    let mut state = DiagnosticReportProcessingState {
      filter: DiagnosticMessage { code: vec![] },
      messages: self._messages.clone()
    };

    // Get to a trie reference
    match &self._trie {
      Some(trie) => {

        // Process messages
        loop {

          // Check if next step viable
          if !is_continue(&state) {
            break;
          }

          // Peek available branches
          let mut filter_next_0 = state.filter.clone();
          filter_next_0.code.push(false);
          let mut filter_next_1 = state.filter.clone();
          filter_next_1.code.push(true);

          // Defer choosing branch and updating messages payload
          let branches = (
            trie.predictive_search(filter_next_0.code).iter().map(|msg| { DiagnosticMessage { code: msg.clone() } }).collect(),
            trie.predictive_search(filter_next_1.code).iter().map(|msg| { DiagnosticMessage { code: msg.clone() } }).collect()
          );
          state = do_update(
            state,
            branches
          );

        }

      },
      _ => ()
    }

    // Return final step
    state    
  }

}

/// Current processing state
#[derive(Clone)]
pub struct DiagnosticReportProcessingState {
  pub filter: DiagnosticMessage,
  pub messages: Vec<DiagnosticMessage>
}
