use crate::{
    parsers::{whitespace, whitespace_1plus},
    traits::Parsable,
    CloudLayer, CloudType, Clouds, CompassDirection, Data, Kind, MetarError, Pressure,
    RunwayCondition, RunwayVisualRange, Time, Trend, VerticalVisibility, Visibility, Weather,
    WeatherCondition, Wind, WindDirection, WindSpeed, WindshearWarnings,
};
use chumsky::prelude::*;

#[derive(PartialEq, Clone, Debug)]
/// A complete METAR
pub struct Metar {
    /// The station making the METAR measurement
    pub station: String,
    /// The measurement time
    pub time: Time,
    /// The kind of METAR, i.e. Normal, Automatic or Correction
    pub kind: Kind,
    /// The current wind information
    pub wind: Wind,
    /// The current visibility
    pub visibility: Data<Visibility>,
    /// If the visibility is reduced further in a specific direction,
    /// that will be covered here.
    pub reduced_directional_visibility: Vec<(CompassDirection, Data<Visibility>)>,
    /// Specific visual ranges for runways
    pub rvr: Vec<RunwayVisualRange>,
    /// The current clouds
    pub clouds: Clouds,
    /// The current clouds
    pub cloud_layers: Vec<CloudLayer>,
    /// The current vertical visibility, in feet
    pub vert_visibility: Option<VerticalVisibility>,
    /// The current weather conditions
    pub weather: Data<Vec<Weather>>,
    /// The current temperature
    pub temperature: Data<i32>,
    /// The current dewpoint
    pub dewpoint: Data<i32>,
    /// The current air pressure
    pub pressure: Pressure,
    /// Additional recent weather conditions
    pub recent_weather: Vec<WeatherCondition>,
    /// Windshear warnings
    pub windshear_warnings: Option<WindshearWarnings>,
    /// The condition of runways
    pub runway_conditions: Vec<RunwayCondition>,
    /// Trends of the weather changing in the near future
    pub trends: Vec<Trend>,
    /// Clouds in the vicinity may be specified separately
    pub clouds_in_vicinity: Vec<(Vec<CompassDirection>, Data<CloudType>)>,
    /// Remarks added on to the METAR
    pub remarks: Option<String>,
}

impl Parsable for Metar {
    #[allow(clippy::too_many_lines)]
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<MetarError<'src>>> {
        let station = regex("[A-Z0-9]{4}");
        let method = choice((
            just("AUTO")
                .map(|_| Kind::Automatic)
                .then_ignore(whitespace_1plus()),
            just("COR")
                .map(|_| Kind::Correction)
                .then_ignore(whitespace_1plus()),
            empty().map(|()| Kind::Normal),
        ));

        let temperature = choice((
            just("M")
                .then(
                    text::digits(10)
                        .exactly(2)
                        .to_slice()
                        .map(|d: &str| d.parse::<i32>().unwrap()),
                )
                .map(|(_, v)| -v),
            text::digits(10)
                .exactly(2)
                .to_slice()
                .map(|d: &str| d.parse().unwrap()),
        ));

        group((
            station.then_ignore(whitespace_1plus()),
            Time::parser().then_ignore(whitespace_1plus()),
            method,
            choice((
                Wind::parser(),
                empty().map(|()| Wind::Present {
                    dir: WindDirection::Heading(Data::Unknown),
                    speed: WindSpeed::Knots {
                        speed: Data::Unknown,
                        gusting: None,
                    },
                    varying: None,
                }),
            )),
            choice((
                Data::<Visibility>::parser().then_ignore(whitespace_1plus()),
                empty().map(|()| Data::Unknown),
            )),
            <(CompassDirection, Data<Visibility>) as Parsable>::parser()
                .separated_by(whitespace_1plus())
                .allow_trailing()
                .collect::<Vec<_>>(),
            RunwayVisualRange::parser()
                .separated_by(whitespace_1plus())
                .allow_trailing()
                .collect::<Vec<_>>(),
            choice((
                just("SKC")
                    .map(|_| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![]))
                    .then_ignore(whitespace_1plus()),
                just("CLR")
                    .map(|_| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![]))
                    .then_ignore(whitespace_1plus()),
                group((
                    Data::parser_inline(
                        2,
                        Weather::parser()
                            .separated_by(whitespace_1plus())
                            .allow_trailing()
                            .collect::<Vec<_>>(),
                    )
                    .then_ignore(whitespace()),
                    VerticalVisibility::parser()
                        .map(Some)
                        .then_ignore(whitespace_1plus())
                        .or(empty().map(|()| None)),
                    Clouds::parser(),
                    CloudLayer::parser()
                        .separated_by(whitespace_1plus())
                        .allow_trailing()
                        .collect::<Vec<_>>(),
                ))
                .map(|(wx, vvis, clouds, layers)| (wx, vvis, clouds, layers)),
                empty().map(|()| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![])),
            )),
            group((
                Data::parser_inline(2, temperature),
                just("/"),
                Data::parser_inline(2, temperature),
            ))
            .map(|(temp, _, dewp)| (temp, dewp))
            .then_ignore(whitespace_1plus())
            .or(empty().map(|()| (Data::Unknown, Data::Unknown))),
            Pressure::parser()
                .then_ignore(whitespace_1plus())
                .or(empty().map(|()| Pressure::Hectopascals(Data::Unknown))),
            choice((
                just("RE")
                    .then(
                        WeatherCondition::parser()
                            .repeated()
                            .at_least(1)
                            .collect::<Vec<_>>(),
                    )
                    .map(|(_, wx)| wx)
                    .separated_by(whitespace_1plus())
                    .collect::<Vec<_>>()
                    .then_ignore(whitespace_1plus()),
                empty().map(|()| vec![]),
            )),
            WindshearWarnings::parser()
                .map(Some)
                .then_ignore(whitespace_1plus())
                .or(empty().map(|()| None)),
            RunwayCondition::parser()
                .separated_by(whitespace_1plus())
                .allow_trailing()
                .collect::<Vec<_>>(),
            Trend::parser()
                .separated_by(whitespace())
                .allow_trailing()
                .collect::<Vec<_>>(),
            <(Vec<CompassDirection>, Data<CloudType>) as Parsable>::parser()
                .separated_by(whitespace_1plus())
                .allow_trailing()
                .collect::<Vec<_>>(),
            just("RMK")
                .then(none_of("=").repeated().collect::<String>())
                .map(|(_, s)| Some(s.trim().to_string()))
                .or(empty().map(|()| None)),
            whitespace(),
            choice((just("=").map(|_| ()), empty().map(|()| ()))),
        ))
        .map(
            |(
                station,
                time,
                kind,
                wind,
                visibility,
                reduced_directional_visibility,
                rvr,
                (weather, vert_visibility, clouds, cloud_layers),
                (temperature, dewpoint),
                pressure,
                recent_weather,
                windshear_warnings,
                runway_conditions,
                trends,
                clouds_in_vicinity,
                remarks,
                (),
                (),
            )| {
                Metar {
                    station: station.to_string(),
                    time,
                    kind,
                    wind,
                    visibility,
                    reduced_directional_visibility,
                    rvr,
                    weather,
                    vert_visibility,
                    clouds,
                    cloud_layers,
                    temperature,
                    dewpoint,
                    pressure,
                    recent_weather: recent_weather.iter().flatten().copied().collect::<Vec<_>>(),
                    windshear_warnings,
                    runway_conditions,
                    trends,
                    clouds_in_vicinity,
                    remarks,
                }
            },
        )
    }
}

impl Metar {
    /// Parse a string into a METAR.
    ///
    /// # Errors
    ///
    /// Returns a [`MetarError`] if parsing failed.
    pub fn parse(data: &str) -> Result<Self, Vec<MetarError<'_>>> {
        <Metar as Parsable>::parse(data).map_err(|v| {
            v.into_iter()
                .map(|mut e| {
                    e.string = data;
                    e
                })
                .collect::<Vec<_>>()
        })
    }
}
