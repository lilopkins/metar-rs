use chumsky::prelude::*;

use crate::{traits::Parsable, CompassDirection, Data};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// A cloud type description
pub enum CloudType {
    /// A normal cloud
    Normal,
    /// A cumulonimbus cloud
    Cumulonimbus,
    /// A towering cumulus cloud
    ToweringCumulus,
}

impl Parsable for CloudType {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("TCU").map(|_| CloudType::ToweringCumulus),
            just("CB").map(|_| CloudType::Cumulonimbus),
            empty().map(|()| CloudType::Normal),
        ))
    }
}

impl Parsable for (Vec<CompassDirection>, Data<CloudType>) {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        Data::parser_inline(3, CloudType::parser())
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
