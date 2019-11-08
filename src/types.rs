#[derive(PartialEq, Eq, Clone, Debug)]
/// A struct to store time as it is represented in a METAR
pub struct Time {
    /// The date the METAR was made
    pub date: u8,
    /// The hour the METAR was made
    pub hour: u8,
    /// The minute the METAR was made
    pub minute: u8,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A struct representing the wind speed
pub enum WindSpeed {
    /// A wind speed measured in knots
    Knot(u32),
    /// A wind speed measured in metres per second
    MetresPerSecond(u32),
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A representation of wind direction
pub enum WindDirection {
    /// A heading defining wind direction
    Heading(u32),
    /// Wind direction is variable
    Variable,
    /// Wind speed is above 49mps or 99kt
    Above,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Horizontal visibility
pub enum Visibility {
    /// Visibility is less than this number of metres
    LessThanMetres(u32),
    /// Visibility is less than this number of statute miles
    LessThanStatuteMiles(u32),
    /// Visibility in metres
    Metres(u32),
    /// Visibility in statute miles
    StatuteMiles(u32),
    /// Clouds and Visibility OK (CAVOK)
    CavOK,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Measured air pressure
pub enum Pressure {
    /// Pressure in hectopascals
    Hectopascals(u32),
    /// Pressure in inches of mercury (inHg)
    InchesMercury(u32),
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Vertical visibility measurement
pub enum VertVisibility {
    /// A distance of vertical visibility
    Distance(u32),
    /// The vertical visibility value is present, so is reduced, but by an amount that hasn't or
    /// cannot be measured
    ReducedByUnknownAmount,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Cloud state
pub enum Clouds {
    /// The sky is clear - also set from CavOK
    SkyClear,
    /// No cloud was detected
    NoCloudDetected,
    /// No significant cloud was detected below 5000ft
    NoSignificantCloud,
    /// Layers of cloud, described elsewhere
    CloudLayers,
    /// Only used when vertical visibility is set.
    Undetermined,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Cloud cover
pub enum CloudLayer {
    /// Few clouds (1/8)
    Few(CloudType, Option<u32>),
    /// Scattered cloud cover (3/8)
    Scattered(CloudType, Option<u32>),
    /// Broken cloud cover (5/8)
    Broken(CloudType, Option<u32>),
    /// Overcast cloud cover (7/8)
    Overcast(CloudType, Option<u32>),
    /// Cloud cover of an unknown density
    Unknown(CloudType, Option<u32>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A cloud type description
pub enum CloudType {
    /// A normal cloud
    Normal,
    /// A cumulonimbus cloud
    Cumulonimbus,
    /// A towering cumulus cloud
    ToweringCumulus,
    /// An unknown cloud type
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// A weather information block
pub struct Weather {
    /// The intensity of this weather block
    pub intensity: WeatherIntensity,
    /// The weather condition/s this block describes.
    pub conditions: Vec<WeatherCondition>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Intensity of weather
pub enum WeatherIntensity {
    /// Light (-)
    Light,
    /// Moderate (no prefix)
    Moderate,
    /// Heavy (+)
    Heavy,
    /// In the vicinity (VC)
    InVicinity,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Descriptor of weather
pub enum WeatherCondition {
    /// [Descriptor] Shallow (MI)
    Shallow,
    /// [Descriptor] Partial (PR)
    Partial,
    /// [Descriptor] Patches (BC)
    Patches,
    /// [Descriptor] Low drifting (DR)
    LowDrifting,
    /// [Descriptor] Blowing (BL)
    Blowing,
    /// [Descriptor] Showers (SH)
    Showers,
    /// [Descriptor] Thunderstorm (TS)
    Thunderstorm,
    /// [Descriptor] Freezing (FZ)
    Freezing,
    /// [Precipitation] Rain (RA)
    Rain,
    /// [Precipitation] Drizzle (DZ)
    Drizzle,
    /// [Precipitation] Snow (SN)
    Snow,
    /// [Precipitation] Snow Grains (SG)
    SnowGrains,
    /// [Precipitation] Ice Crystals (IC)
    IceCrystals,
    /// [Precipitation] Ice pellets (PL)
    IcePellets,
    /// [Precipitation] Hail (including small hail in the US) (GR)
    Hail,
    /// [Precipitation] Snow Pellets and/or Small Hail (except in US) (GS)
    SnowPelletsOrSmallHail,
    /// [Precipitation] Unknown precipitation (UP)
    UnknownPrecipitation,
    /// [Obscuration] Fog (FG)
    Fog,
    /// [Obscuration] Volcanic Ash (VA)
    VolcanicAsh,
    /// [Obscuration] Mist (BR)
    Mist,
    /// [Obscuration] Haze (HZ)
    Haze,
    /// [Obscuration] Widespread dust (DU)
    WidespreadDust,
    /// [Obscuration] Smoke (FU)
    Smoke,
    /// [Obscuration] Sand (SA)
    Sand,
    /// [Obscuration] Spray (PY)
    Spray,
    /// [Other] Squall (SQ)
    Squall,
    /// [Other] Dust or Sand Whirls (PO)
    Dust,
    /// [Other] Duststorm (DS)
    Duststorm,
    /// [Other] Sandstorm (SS)
    Sandstorm,
    /// [Other] Funnel Cloud (FC)
    FunnelCloud,
}

#[derive(PartialEq, Eq, Clone, Debug)]
/// Wind information
pub struct Wind {
    /// The wind direction, in degrees
    pub dir: WindDirection,
    /// The current wind speed
    pub speed: WindSpeed,
    /// The direction the wind may be varying between, smaller always comes first
    pub varying: Option<(u32, u32)>,
    /// The gusting speed of the wind
    pub gusting: Option<WindSpeed>,
}
