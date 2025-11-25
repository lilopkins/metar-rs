/// Data that is provided in a metar which might be unknown.
/// Note that this differs from an `Option<T>` field which is used when data
/// might not be given at all. In the cases where `Data<T>` is used, data is
/// usually given but has been replaced in the METAR by slashes, indicating
/// that it is not known.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum Data<T> {
    /// The data is known and given
    Known(T),
    /// The data isn't or cannot be known
    Unknown,
}

impl<T> Data<T> {
    /// Unwraps the inner data type, panics otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the data is [`Data::Unknown`].
    pub fn unwrap(&self) -> &T {
        match self {
            Data::Known(v) => v,
            Data::Unknown => panic!("cannot unwrap unknown data"),
        }
    }

    /// Mutably unwraps the inner data type, panics otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the data is [`Data::Unknown`].
    pub fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Data::Known(v) => v,
            Data::Unknown => panic!("cannot unwrap unknown data"),
        }
    }
}
