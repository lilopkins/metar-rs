#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
/// The wind speed
pub enum WindSpeed {
    /// Winds calm
    Calm,
    /// Nautical miles per hour
    Knot(u32),
    /// Metres per second
    MetresPerSecond(u32),
    /// Kilometres per hour
    KilometresPerHour(u32),
}

impl WindSpeed {
    pub(crate) fn clone_changing_contents(self, new_contents: u32) -> Self {
        match self {
            WindSpeed::Calm => WindSpeed::Calm,
            WindSpeed::Knot(_) => WindSpeed::Knot(new_contents),
            WindSpeed::MetresPerSecond(_) => WindSpeed::MetresPerSecond(new_contents),
            WindSpeed::KilometresPerHour(_) => WindSpeed::KilometresPerHour(new_contents),
        }
    }
}
