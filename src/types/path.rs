use crate::{FromPathParam, FromRequest, Request};
use route_recognizer::Params;

use anyhow::Result;

use std::fmt::Debug;

pub struct Path<T>(pub T);

macro_rules! extract_for_path {
    [$(($t:ident, $v:ident)),*] => {
        #[async_trait::async_trait]
        impl<'r, $($t,)*> FromRequest<'r> for Path<($($t,)*)>
        where
            $(
            $t: FromPathParam + Debug,
            )*
        {
            async fn from_request(request: &'r Request) -> Result<Self>
            {
                let params = request.extensions().get::<Params>().unwrap();
                let mut iter = params.iter();

                $(
                    let param = iter.next().unwrap().1;
                    let $v = $t::from_path_param(param).unwrap();
                    tracing::debug!("Extracted param {:?}", $v);
                )*

                Ok(Path(($($v,)*)))
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    extract_for_path![(T1, t1)];
    extract_for_path![(T1, t1), (T2, t2)];
    extract_for_path![(T1, t1), (T2, t2), (T3, t3)];
    extract_for_path![(T1, t1), (T2, t2), (T3, t3), (T4, t4)];
    extract_for_path![(T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5)];
    extract_for_path![(T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6)];
    extract_for_path![(T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6), (T7, t7)];
    extract_for_path![(T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6), (T7, t7), (T8, t8)];
    extract_for_path![(T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6), (T7, t7), (T8, t8), (T9, t9)];
    extract_for_path![(T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6), (T7, t7), (T8, t8), (T9, t9), (T10, t10)];
}

#[cfg(test)]
mod test {
    /*
    use crate::core::extract::Extract;

    use super::*;

    fn assert_extract<'r>(_: impl Extract<'r>) {}

    #[test]
    fn paths() {
        let p1 = Path(("test".to_string(),));
        assert_extract(p1);
    }
    */
}
