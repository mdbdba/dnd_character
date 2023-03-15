use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Modifier<T> {
    KeepLowest(T),
    KeepHighest(T),
    DropLowest(T),
    DropHighest(T),
    None,
}

impl<T: Display> Display for Modifier<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeepLowest(v) => {
                write!(f, "kl")?;
                v.fmt(f)?
            }
            Self::KeepHighest(v) => {
                write!(f, "kh")?;
                v.fmt(f)?;
            }
            Self::DropLowest(v) => {
                write!(f, "dl")?;
                v.fmt(f)?;
            }
            Self::DropHighest(v) => {
                write!(f, "dh")?;
                v.fmt(f)?;
            }
            Self::None => {}
        }
        Ok(())
    }
}
/*
impl<T> Modifier<T> {
    pub(crate) fn map<F, U>(self, f: F) -> Modifier<U>
        where
            F: FnOnce(T) -> U,
    {
        match self {
            Self::KeepLowest(i) => Modifier::KeepLowest(f(i)),
            Self::KeepHighest(i) => Modifier::KeepHighest(f(i)),
            Self::DropHighest(i) => Modifier::DropHighest(f(i)),
            Self::DropLowest(i) => Modifier::DropLowest(f(i)),
            Self::None => Modifier::None,
        }
    }
}

impl<T, E> Modifier<Result<T, E>> {
    pub(crate) fn swap(self) -> Result<Modifier<T>, E> {
        Ok(match self {
            Modifier::KeepLowest(i) => Modifier::KeepLowest(i?),
            Modifier::KeepHighest(i) => Modifier::KeepHighest(i?),
            Modifier::DropLowest(i) => Modifier::DropLowest(i?),
            Modifier::DropHighest(i) => Modifier::DropHighest(i?),
            Modifier::None => Modifier::None,
        })
    }
}
*/
