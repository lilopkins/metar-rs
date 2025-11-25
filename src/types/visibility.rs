#[derive(PartialEq, Copy, Clone, Debug)]
/// Horizontal visibility
pub enum Visibility {
    /// Visibility OK
    CAVOK,
    /// Metres
    Metres(u16),
    /// Statute miles, usually used in the US
    StatuteMiles(f32),
}
