#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// Descriptor of weather
pub enum WeatherCondition {
    /// Descriptor - Shallow (MI)
    Shallow,
    /// Descriptor - Partial (PR)
    Partial,
    /// Descriptor - Patches (BC)
    Patches,
    /// Descriptor - Low drifting (DR)
    LowDrifting,
    /// Descriptor - Blowing (BL)
    Blowing,
    /// Descriptor - Showers (SH)
    Showers,
    /// Descriptor - Thunderstorm (TS)
    Thunderstorm,
    /// Descriptor - Freezing (FZ)
    Freezing,
    /// Precipitation - Rain (RA)
    Rain,
    /// Precipitation - Drizzle (DZ)
    Drizzle,
    /// Precipitation - Snow (SN)
    Snow,
    /// Precipitation - Snow Grains (SG)
    SnowGrains,
    /// Precipitation - Ice Crystals (IC)
    IceCrystals,
    /// Precipitation - Ice pellets (PL)
    IcePellets,
    /// Precipitation - Hail (including small hail in the US) (GR)
    Hail,
    /// Precipitation - Snow Pellets and/or Small Hail (except in US) (GS)
    SnowPelletsOrSmallHail,
    /// Precipitation - Unknown precipitation (UP)
    UnknownPrecipitation,
    /// Obscuration - Fog (FG)
    Fog,
    /// Obscuration - Volcanic Ash (VA)
    VolcanicAsh,
    /// Obscuration - Mist (BR)
    Mist,
    /// Obscuration - Haze (HZ)
    Haze,
    /// Obscuration - Widespread dust (DU)
    WidespreadDust,
    /// Obscuration - Smoke (FU)
    Smoke,
    /// Obscuration - Sand (SA)
    Sand,
    /// Obscuration - Spray (PY)
    Spray,
    /// Other - Squall (SQ)
    Squall,
    /// Other - Dust or Sand Whirls (PO)
    Dust,
    /// Other - Duststorm (DS)
    Duststorm,
    /// Other - Sandstorm (SS)
    Sandstorm,
    /// Other - Funnel Cloud (FC)
    FunnelCloud,
}
