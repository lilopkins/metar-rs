use chumsky::prelude::*;

use crate::{traits::Parsable, CompassDirection};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

impl Parsable for CloudType {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("TCU").map(|_| CloudType::ToweringCumulus),
            just("CB").map(|_| CloudType::Cumulonimbus),
            just("///").map(|_| CloudType::Unknown),
            empty().map(|()| CloudType::Normal),
        ))
    }
}

impl Parsable for (Vec<CompassDirection>, CloudType) {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        CloudType::parser()
            .then(
                group((just("/"), CompassDirection::parser()))
                    .map(|(_, dir)| dir)
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .map(|(typ, dirs)| (dirs, typ))
    }
}
