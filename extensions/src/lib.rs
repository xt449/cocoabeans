pub trait ResultToOption<T> {
    fn to_option(self) -> Option<T>;
}

impl<T, E> ResultToOption<T> for Result<T, E> {
    fn to_option(self) -> Option<T> {
        return self.map_or_else(|_err| None, |ok| Some(ok));
    }
}

pub trait OptionToResult<T> {
    fn to_result(self) -> Result<T, ()>;
}

impl<T> OptionToResult<T> for Option<T> {
    fn to_result(self) -> Result<T, ()> {
        return self.map_or_else(|| Err(()), |some| Ok(some));
    }
}

pub trait OptionFrom<T>: TryFrom<T> {
    fn option_from(i: T) -> Option<Self> {
        return Self::try_from(i).map_or_else(|_err| None, |ok| Some(ok));
    }
}

impl<T, U> OptionFrom<T> for U where U: TryFrom<T> {}

pub trait OptionInto<T>: TryInto<T> {
    fn option_into(self) -> Option<T> {
        return self.try_into().map_or_else(|_err| None, |ok| Some(ok));
    }
}

impl<T, U> OptionInto<T> for U where U: TryInto<T> {}
