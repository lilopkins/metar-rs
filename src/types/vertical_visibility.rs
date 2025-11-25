#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// Vertical visibility measurement
pub enum VerticalVisibility {
    /// A distance of vertical visibility
    Distance(u32),
    /// The vertical visibility value is present, so is reduced, but by an amount that hasn't or
    /// cannot be measured
    ReducedByUnknownAmount,
}
