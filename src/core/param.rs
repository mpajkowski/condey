use anyhow::anyhow;
use thiserror::Error;

use std::{
    fmt::{Debug, Display},
    str::{FromStr, Utf8Error},
};

#[derive(Error, Debug)]
pub enum FromPathParamError {
    #[error("UTF8 Error: `{0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("Parse error: `{0}")]
    ParseError(#[from] anyhow::Error),
}

pub trait FromPathParam {
    fn from_path_param(param: &str) -> Result<Self, FromPathParamError>
    where
        Self: Sized;
}

impl<T: FromStr> FromPathParam for T
where
    T::Err: Display + Debug + Send + Sync + 'static,
{
    fn from_path_param(param: &str) -> Result<Self, FromPathParamError> {
        let percent_decoded = percent_encoding::percent_decode_str(param).decode_utf8()?;
        let result = percent_decoded.parse::<T>().map_err(|err| anyhow!(err))?;
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! accept_num {
        ($($t:ident),*) => {
            #[test]
            fn accept_num() {
            $(
                {
                    let var = "42";
                    let r: $t = FromPathParam::from_path_param(var).unwrap();

                    assert_eq!(r, 42);
                }
            )*
            }
        };
    }

    accept_num!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

    #[test]
    fn parse_urlencoded() {
        struct StringParam(String);

        impl FromPathParam for StringParam {
            fn from_path_param(param: &str) -> Result<Self, FromPathParamError> {
                let s = String::from_path_param(param)?;
                Ok(StringParam(s))
            }
        }

        let urlencoded = "some%20parameter";
        let result = StringParam::from_path_param(urlencoded).unwrap();

        assert_eq!(result.0, "some parameter");
    }
}
