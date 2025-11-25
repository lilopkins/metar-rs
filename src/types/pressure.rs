#[derive(PartialEq, Copy, Clone, Debug)]
/// Measured air pressure
pub enum Pressure {
    /// Pressure in hectopascals
    Hectopascals(u16),
    /// Pressure in inches of mercury (inHg)
    InchesOfMercury(f32),
}
