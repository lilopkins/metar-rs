use super::CloudType;

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
