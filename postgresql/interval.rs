//! PostgreSQL `interval`.

/// A PostgreSQL `interval`: an exact count of months and days plus a
/// microsecond time component.
///
/// Months and days are kept separate because their length depends on the
/// calendar (a month may be 28–31 days) and they cannot be collapsed into a
/// single duration losslessly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Interval {
    /// Number of months.
    pub months: i32,
    /// Number of days.
    pub days: i32,
    /// Number of microseconds.
    pub micros: i64,
}

impl Interval {
    /// Creates an interval from its components.
    pub const fn new(months: i32, days: i32, micros: i64) -> Self {
        Interval {
            months,
            days,
            micros,
        }
    }
}
