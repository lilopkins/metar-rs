#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// Data that is provided in a metar which might be unknown.
/// Note that this differs from an `Option<T>` field which is used when data
/// might not be given at all. In the cases where `Data<T>` is used, data is
/// usually given but has been replaced in the METAR by slashes, indicating
/// that it is not known.
pub enum Data<T> {
    /// The data is known and given
    Known(T),
    /// The data isn't or cannot be known
    Unknown,
}

impl<T> Data<T> {
    /// Unwraps the inner data type, panics otherwise
    pub fn unwrap(&self) -> &T {
        match self {
            Data::Known(v) => v,
            Data::Unknown => panic!("cannot unwrap unknown data"),
        }
    }

    /// Mutably unwraps the inner data type, panics otherwise
    pub fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Data::Known(v) => v,
            Data::Unknown => panic!("cannot unwrap unknown data"),
        }
    }
}

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

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// A struct representing the wind speed
pub struct WindSpeed {
    /// The wind speed
    pub speed: u32,
    /// The unit used whilst measuring this wind speed
    pub unit: SpeedUnit,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
/// Units of speed
pub enum SpeedUnit {
    /// Nautical miles per hour
    Knot,
    /// Metres per second
    MetresPerSecond,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
/// A representation of wind direction
pub enum WindDirection {
    /// A heading defining wind direction
    Heading(u32),
    /// Wind direction is variable
    Variable,
    /// Wind speed is above 49mps or 99kt
    Above,
}

#[derive(PartialEq, Clone, Debug)]
/// Horizontal visibility
pub struct Visibility {
    /// The measured visibility
    pub visibility: f32,
    /// The unit this is measured in
    pub unit: DistanceUnit,
}

impl Visibility {
    /// Generate a infinite visibility instance
    pub fn infinite() -> Self {
        Visibility {
            visibility: 9999.0,
            unit: DistanceUnit::Metres,
        }
    }

    /// Returns true if the distance given is considered infinite
    pub fn is_infinite(&self) -> bool {
        match self.unit {
            DistanceUnit::Metres => self.visibility == 9999.0,
            DistanceUnit::StatuteMiles => self.visibility == 9.9,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
/// Units of distance
pub enum DistanceUnit {
    /// Metres
    Metres,
    /// Statute miles, usually used in the US
    StatuteMiles,
}

#[derive(PartialEq, Clone, Debug)]
/// Measured air pressure
pub struct Pressure {
    /// The air pressure
    pub pressure: f32,
    /// The unit this pressure is measured in
    pub unit: PressureUnit,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
/// Units of air pressure
pub enum PressureUnit {
    /// Pressure in hectopascals
    Hectopascals,
    /// Pressure in inches of mercury (inHg)
    InchesMercury,
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

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
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
    pub dir: Data<WindDirection>,
    /// The current wind speed
    pub speed: Data<WindSpeed>,
    /// The direction the wind may be varying between, smaller always comes first
    pub varying: Option<(u32, u32)>,
    /// The gusting speed of the wind
    pub gusting: Option<WindSpeed>,
}
