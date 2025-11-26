use crate::{traits::Parsable, Time, Kind, Wind, Data, Visibility, CompassDirection, RunwayVisualRange, Clouds, CloudLayer, VerticalVisibility, Weather, Pressure, WeatherCondition, WindshearWarnings, RunwayCondition, Trend, CloudType, MetarError, WindDirection, WindSpeed};
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
    pub clouds_in_vicinity: Vec<(Vec<CompassDirection>, CloudType)>,
    /// Remarks added on to the METAR
    pub remarks: Option<String>,
}

impl Parsable for Metar {
    #[allow(clippy::too_many_lines)]
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<MetarError<'src>>> {
        let whitespace = text::inline_whitespace();
        let whitespace_1plus = text::inline_whitespace().at_least(1);

        let station = regex("[A-Z0-9]{4}");
        let method = choice((
            just("AUTO").map(|_| Kind::Automatic),
            just("COR").map(|_| Kind::Correction),
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
            station,
            whitespace,
            Time::parser(),
            whitespace,
            method,
            whitespace,
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
            whitespace,
            choice((Data::<Visibility>::parser(), empty().map(|()| Data::Unknown))),
            whitespace,
            <(CompassDirection, Data<Visibility>) as Parsable>::parser()
                .separated_by(whitespace_1plus)
                .collect::<Vec<_>>(),
            whitespace,
            RunwayVisualRange::parser()
                .separated_by(whitespace_1plus)
                .collect::<Vec<_>>(),
            whitespace,
            choice((
                just("SKC").map(|_| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![])),
                just("CLR").map(|_| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![])),
                group((
                    Data::parser_inline(
                        2,
                        Weather::parser()
                            .separated_by(whitespace_1plus)
                            .collect::<Vec<_>>(),
                    ),
                    whitespace,
                    VerticalVisibility::parser()
                        .map(Some)
                        .or(empty().map(|()| None)),
                    whitespace,
                    Clouds::parser(),
                    whitespace,
                    CloudLayer::parser()
                        .separated_by(whitespace_1plus)
                        .collect::<Vec<_>>(),
                ))
                .map(|(wx, (), vvis, (), clouds, (), layers)| (wx, vvis, clouds, layers)),
                empty().map(|()| (Data::Known(vec![]), None, Clouds::NoCloudDetected, vec![])),
            )),
            whitespace,
            group((
                Data::parser_inline(2, temperature),
                just("/"),
                Data::parser_inline(2, temperature),
            ))
            .map(|(temp, _, dewp)| (temp, dewp))
            .or(empty().map(|()| (Data::Unknown, Data::Unknown))),
            whitespace,
            Pressure::parser().or(empty().map(|()| Pressure::Hectopascals(Data::Unknown))),
            whitespace,
            choice((
                just("RE")
                    .then(
                        WeatherCondition::parser()
                            .repeated()
                            .at_least(1)
                            .collect::<Vec<_>>(),
                    )
                    .map(|(_, wx)| wx)
                    .separated_by(whitespace_1plus)
                    .collect::<Vec<_>>(),
                empty().map(|()| vec![]),
            )),
            whitespace,
            WindshearWarnings::parser()
                .map(Some)
                .or(empty().map(|()| None)),
            whitespace,
            RunwayCondition::parser()
                .separated_by(whitespace_1plus)
                .collect::<Vec<_>>(),
            whitespace,
        ))
        .then(group((
            Trend::parser().separated_by(whitespace).collect::<Vec<_>>(),
            whitespace,
            <(Vec<CompassDirection>, CloudType) as Parsable>::parser()
                .separated_by(whitespace_1plus)
                .collect::<Vec<_>>(),
            whitespace,
            just("RMK")
                .then(none_of("=").repeated().collect::<String>())
                .map(|(_, s)| Some(s.trim().to_string()))
                .or(empty().map(|()| None)),
            whitespace,
            choice((just("=").map(|_| ()), empty().map(|()| ()))),
        )))
        .map(
            |(
                (
                    station,
                    (),
                    time,
                    (),
                    kind,
                    (),
                    wind,
                    (),
                    visibility,
                    (),
                    reduced_directional_visibility,
                    (),
                    rvr,
                    (),
                    (weather, vert_visibility, clouds, cloud_layers),
                    (),
                    (temperature, dewpoint),
                    (),
                    pressure,
                    (),
                    recent_weather,
                    (),
                    windshear_warnings,
                    (),
                    runway_conditions,
                    (),
                ),
                (trends, (), clouds_in_vicinity, (), remarks, (), ()),
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
                    recent_weather: recent_weather
                        .iter()
                        .flatten()
                        .copied()
                        .collect::<Vec<_>>(),
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
