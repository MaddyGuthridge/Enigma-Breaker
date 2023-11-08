use strum::IntoEnumIterator;

/// Represents a potentially-unknown value. Either the value is known, in which
/// case iterating over this enum will only yield that value, or it is unknown,
/// in which case iterating will produce all possible values.
#[derive(Debug, Clone)]
pub enum Unknown<T>
where
    T: IntoEnumIterator + Clone,
{
    /// The value is unknown
    Unknown,

    /// The value is known
    Known(T),
}

impl<T> IntoIterator for Unknown<T>
where
    T: IntoEnumIterator + Clone,
{
    type Item = T;

    type IntoIter = UnknownIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Unknown::Unknown => UnknownIterator::Unknown(T::iter()),
            Unknown::Known(value) => UnknownIterator::Known(value, false),
        }
    }
}

impl<T> IntoIterator for &Unknown<T>
where
    T: IntoEnumIterator + Clone,
{
    type Item = T;

    type IntoIter = UnknownIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Unknown::Unknown => UnknownIterator::Unknown(T::iter()),
            Unknown::Known(value) => UnknownIterator::Known(value.clone(), false),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnknownIterator<T>
where
    T: IntoEnumIterator + Clone,
{
    /// Unknown variant - an iterator over the possible values
    Unknown(T::Iterator),

    /// Known variant - false if not consumed, true if consumed
    Known(T, bool),
}

impl<T> Iterator for UnknownIterator<T>
where
    T: IntoEnumIterator + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            UnknownIterator::Known(item, false) => {
                let clone = item.clone();
                *self = UnknownIterator::Known(item.clone(), true);
                Some(clone)
            }
            UnknownIterator::Known(_, true) => None,
            UnknownIterator::Unknown(inner) => inner.next(),
        }
    }
}

#[cfg(test)]
mod test {
    use strum::EnumIter;

    use super::Unknown;

    #[derive(Debug, Clone, EnumIter, PartialEq)]
    enum ExampleEnum {
        A,
        B,
        C,
    }

    #[test]
    fn unknown_iterates_over_all() {
        let unknown: Unknown<ExampleEnum> = Unknown::Unknown;

        let mut iter = unknown.into_iter();

        assert_eq!(iter.next(), Some(ExampleEnum::A));
        assert_eq!(iter.next(), Some(ExampleEnum::B));
        assert_eq!(iter.next(), Some(ExampleEnum::C));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn known_only_gives_known_value() {
        let known = Unknown::Known(ExampleEnum::B);

        let mut iter = known.into_iter();

        assert_eq!(iter.next(), Some(ExampleEnum::B));
        assert_eq!(iter.next(), None);
    }
}
