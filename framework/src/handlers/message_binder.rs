use std::{fmt::Debug, str::FromStr};

pub fn bind_message<T, U>(message: &str, position: usize, _name: &str) -> T
where
    T: FromStr<Err = U> + Debug,
    U: Debug,
{
    message.split_whitespace().nth(position + 1).unwrap().parse().unwrap()
}