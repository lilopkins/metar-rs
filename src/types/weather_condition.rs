use chumsky::prelude::*;

use crate::traits::Parsable;

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

impl Parsable for WeatherCondition {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("MI").map(|_| WeatherCondition::Shallow),
            just("PR").map(|_| WeatherCondition::Partial),
            just("BC").map(|_| WeatherCondition::Patches),
            just("DR").map(|_| WeatherCondition::LowDrifting),
            just("BL").map(|_| WeatherCondition::Blowing),
            just("SH").map(|_| WeatherCondition::Showers),
            just("TS").map(|_| WeatherCondition::Thunderstorm),
            just("FZ").map(|_| WeatherCondition::Freezing),
            just("RA").map(|_| WeatherCondition::Rain),
            just("DZ").map(|_| WeatherCondition::Drizzle),
            just("SN").map(|_| WeatherCondition::Snow),
            just("SG").map(|_| WeatherCondition::SnowGrains),
            just("IC").map(|_| WeatherCondition::IceCrystals),
            just("PL").map(|_| WeatherCondition::IcePellets),
            just("GR").map(|_| WeatherCondition::Hail),
            just("GS").map(|_| WeatherCondition::SnowPelletsOrSmallHail),
            just("UP").map(|_| WeatherCondition::UnknownPrecipitation),
            just("FG").map(|_| WeatherCondition::Fog),
            just("VA").map(|_| WeatherCondition::VolcanicAsh),
            just("BR").map(|_| WeatherCondition::Mist),
            just("HZ").map(|_| WeatherCondition::Haze),
            just("DU").map(|_| WeatherCondition::WidespreadDust),
            just("FU").map(|_| WeatherCondition::Smoke),
            just("SA").map(|_| WeatherCondition::Sand),
            just("PY").map(|_| WeatherCondition::Spray),
            just("SQ").map(|_| WeatherCondition::Squall),
            // Too many for one choice, split into two!
        ))
        .or(choice((
            just("PO").map(|_| WeatherCondition::Dust),
            just("DS").map(|_| WeatherCondition::Duststorm),
            just("SS").map(|_| WeatherCondition::Sandstorm),
            just("FC").map(|_| WeatherCondition::FunnelCloud),
        )))
    }
}
