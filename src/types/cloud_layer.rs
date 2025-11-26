use chumsky::prelude::*;

use crate::{traits::Parsable, Data};

use super::CloudType;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
/// Cloud cover
pub struct CloudLayer {
    density: Data<CloudDensity>,
    kind: Data<CloudType>,
    height: Data<u32>,
}

impl Parsable for CloudLayer {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        group((
            Data::parser_inline(3, CloudDensity::parser()),
            Data::parser_inline(
                3,
                text::digits(10)
                    .exactly(3)
                    .to_slice()
                    .map(|d: &str| d.parse().unwrap()),
            ),
            Data::parser_inline(3, CloudType::parser()),
        ))
        .map(|(density, height, kind)| CloudLayer {
            density,
            kind,
            height,
        })
    }
}

/// The density of the cloud cover
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CloudDensity {
    /// Few clouds (1/8)
    Few,
    /// Scattered cloud cover (3/8)
    Scattered,
    /// Broken cloud cover (5/8)
    Broken,
    /// Overcast cloud cover (7/8)
    Overcast,
}

impl Parsable for CloudDensity {
    fn parser<'src>() -> impl Parser<'src, &'src str, Self, extra::Err<crate::MetarError<'src>>> {
        choice((
            just("FEW").map(|_| CloudDensity::Few),
            just("SCT").map(|_| CloudDensity::Scattered),
            just("BKN").map(|_| CloudDensity::Broken),
            just("OVC").map(|_| CloudDensity::Overcast),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_layers() {
        assert_eq!(
            CloudLayer::parse("BKN300CB").unwrap(),
            CloudLayer {
                density: Data::Known(CloudDensity::Broken),
                height: Data::Known(300),
                kind: Data::Known(CloudType::Cumulonimbus),
            }
        );
        assert_eq!(
            CloudLayer::parse("/////////").unwrap(),
            CloudLayer {
                density: Data::Unknown,
                height: Data::Unknown,
                kind: Data::Unknown,
            }
        );
    }
}
