use std::ops::{Range, RangeInclusive};

use crate::Letter;

/// Represents information known about the plug board configuration
pub enum PlugboardOptions {
    /// All connections are known
    KnownConnections(Vec<(Letter, Letter)>),

    /// The number of connections is within the given range
    NumberInRange(Range<usize>),

    /// The number of connections is within the given range
    NumberInRangeInclusive(RangeInclusive<usize>),
}

unsafe impl Send for PlugboardOptions {}
