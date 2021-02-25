use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// An error caused whilst parsing the weather station
pub enum StationError {
    /// The station ID is not the correct length
    IncorrectLength,
    /// A character was found to be not alphanumeric
    NonAlphanumericCharacter,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// An error caused when parsing the observation time
pub enum ObservationTimeError {
    /// The observation time is not the correct length
    IncorrectLength,
    /// The observation date is not valid
    DateNotValid,
    /// The observation hour is not valid
    HourNotValid,
    /// The observation minute is not valid
    MinuteNotValid,
    /// The specified time zone is not within the ICAO METAR standard
    InvalidTimeZone,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// An error caused when parsing the wind
pub enum WindError {
    /// The wind information is not the correct length
    IncorrectLength,
    /// The wind heading is not valid
    HeadingNotValid,
    /// The wind speed was not valid
    SpeedNotValid,
    /// The wind gusting speed was not valid
    GustingNotValid,
    /// An unknown unit was read
    UnitNotValid,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// An error caused when parsing the wind varying
pub enum WindVaryingError {
    /// The wind heading is not valid
    HeadingNotValid,
    /// Mostly an internal error - informs the calling function that this is not a wind varying
    /// and should be attempted to be parsed as cloud/visibility information
    NotWindVarying,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// An error caused when parsing the cloud and visibility information
pub enum CloudVisibilityError {
    /// The data parsing was attempted upon is unknown in type
    UnknownData,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// An error caused when parsing the temperature
pub enum TemperatureError {
    /// The temperature is not valid
    TemperatureNotValid,
    /// The dewpoint is not valid
    DewpointNotValid,
    /// Mostly an internal error - informs the calling function that this is not a
    /// temperature/dewpoint pair and should be attempted to be parsed as cloud/visibility
    /// information
    NotTemperatureDewpointPair,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// An error caused when parsing the pressure
pub enum PressureError {
    /// The pressure is not valid
    PressureNotValid,
    /// The unit is not valid. Note that this is also returned when a unit is not specified,
    /// or when the pressure is not long enough to satisfy the requirements of any other unit.
    UnitNotValid,
}

impl fmt::Display for StationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncorrectLength => write!(f, "The station ID was not the correct length."),
            Self::NonAlphanumericCharacter => write!(f, "Found a non-alphanumeric character."),
        }
    }
}

impl fmt::Display for ObservationTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncorrectLength => write!(f, "The observation time was not the correct length."),
            Self::DateNotValid => write!(f, "The date was invalid."),
            Self::HourNotValid => write!(f, "The hour was invalid."),
            Self::MinuteNotValid => write!(f, "The minute was invalid."),
            Self::InvalidTimeZone => write!(f, "The time zone was invalid (not Zulu)."),
        }
    }
}

impl fmt::Display for WindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncorrectLength => write!(f, "The length of the wind information is incorrect."),
            Self::HeadingNotValid => write!(f, "The heading is invalid."),
            Self::SpeedNotValid => write!(f, "The speed is invalid."),
            Self::GustingNotValid => write!(f, "The gusting speed is invalid."),
            Self::UnitNotValid => write!(f, "The unit is not valid."),
        }
    }
}

impl fmt::Display for WindVaryingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HeadingNotValid => write!(f, "The heading is invalid."),
            Self::NotWindVarying => unreachable!(),
        }
    }
}

impl fmt::Display for CloudVisibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownData => write!(f, "Unknown data for parsing."),
        }
    }
}

impl fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TemperatureNotValid => write!(f, "The temperature is invalid."),
            Self::DewpointNotValid => write!(f, "The dewpoint is invalid."),
            Self::NotTemperatureDewpointPair => unreachable!(),
        }
    }
}

impl fmt::Display for PressureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PressureNotValid => write!(f, "The pressure is invalid."),
            Self::UnitNotValid => write!(f, "The unit is invalid."),
        }
    }
}
