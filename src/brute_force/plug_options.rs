use std::ops::Range;

use crate::Letter;

/// Represents information known about the plug board configuration
pub enum PlugboardOptions {
    /// All connections are known
    KnownConnections(Vec<(Letter, Letter)>),

    /// The number of connections is within the given range
    NumberInRange(Range<usize>),
}
