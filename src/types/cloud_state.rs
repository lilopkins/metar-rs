use chumsky::prelude::*;

use crate::traits::Parsable;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
/// Cloud state
pub enum Clouds {
    /// No cloud was detected, also set for CAVOK
    NoCloudDetected,
    /// No significant cloud was detected below 5000ft
    NoSignificantCloud,
    /// Layers of cloud, described elsewhere
    CloudLayers,
}

impl Parsable for Clouds {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("NCD").map(|_| Clouds::NoCloudDetected),
            just("NSC").map(|_| Clouds::NoSignificantCloud),
            just("CLR").map(|_| Clouds::NoCloudDetected),
            empty().map(|_| Clouds::CloudLayers),
        ))
    }
}
