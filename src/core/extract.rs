use super::{param::FromPathParam, request::Request};
use anyhow::Result;
use matchit::Params;

pub trait Extract<'r> {
    fn extract(request: &'r Request) -> Result<Self>
    where
        Self: Sized;
}

impl<'r> Extract<'r> for &'r Request {
    fn extract(request: &'r Request) -> Result<Self> {
        Ok(request)
    }
}

pub struct Path<T>(pub T);

macro_rules! extract_for_path {
    ($(($t:ident, $v:ident)),*) => {
        impl<'r, $($t,)*> Extract<'r> for Path<($($t,)*)>
        where
            $(
            $t: FromPathParam,
            )*
        {
            fn extract(request: &'r Request) -> Result<Self>
            where
                Self: Sized,
            {
                let params = request.extensions().get::<Params>().unwrap();
                let mut iter = params.0.iter();

                $(
                let $v = $t::from_path_param(&iter.next().unwrap().value).unwrap();
                )*

                Ok(Path(($($v,)*)))
            }
        }
    };
}

extract_for_path!((T1, t1));
extract_for_path!((T1, t1), (T2, t2));
extract_for_path!((T1, t1), (T2, t2), (T3, t3));
extract_for_path!((T1, t1), (T2, t2), (T3, t3), (T4, t4));
extract_for_path!((T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5));
extract_for_path!((T1, t1), (T2, t2), (T3, t3), (T4, t4), (T5, t5), (T6, t6));
extract_for_path!(
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7)
);
extract_for_path!(
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7),
    (T8, t8)
);
extract_for_path!(
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7),
    (T8, t8),
    (T9, t9)
);
extract_for_path!(
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7),
    (T8, t8),
    (T9, t9),
    (T10, t10)
);

#[cfg(test)]
mod test {
    use super::*;

    fn assert_extract<'r>(_: impl Extract<'r>) {}

    #[test]
    fn paths() {
        let p1 = Path(("test".to_string(),));
        assert_extract(p1);
    }
}
