/// The kind of METAR produced.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// This is a regular METAR.
    Normal,
    /// This METAR was generated automatically without human oversight
    Automatic,
    /// This METAR corrects a previously issued METAR
    Correction,
}
