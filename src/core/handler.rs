use std::{future::Future, marker::PhantomData, pin::Pin};

use super::extract::Extract;
use super::request::Request;
use super::response::Responder;

use crate::http::Response as HttpResponse;
use hyper::Body;

pub trait Handler {
    fn handle_request(
        &self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = HttpResponse<Body>> + Send>>;
}

macro_rules! handler_for_async_fn {
    ($f:ident, [$(($p:ident, $t:ident)),*]) => {
        #[derive(Clone, Copy)]
        pub struct $f<$($t,)* R, Fun, Fut>
        {
            function: Fun,
            $(
            $p: PhantomData<$t>,
            )*
            _r: PhantomData<R>,
            _f: PhantomData<Fut>
        }

        impl<$($t: for<'r> Extract<'r> + Send + Sync + 'static,)* R, Fun, Fut > Handler for $f<$($t,)* R, Fun, Fut>
        where
            R: Responder + Send + Sync + 'static,
            Fun: Fn($($t),*) -> Fut + Send + Sync + Copy + 'static,
            Fut: Future<Output = R> + Send + Sync + 'static,
        {
            #[allow(unused)]
            fn handle_request(&self, mut request: Request) -> Pin<Box<dyn Future<Output = HttpResponse<Body>> + Send>> {
                let fun = self.function.clone();
                Box::pin(async move {
                    $(
                    let $p = $t::extract(&mut request).await.unwrap();
                    )*

                    let result = (fun)($($p,)*).await;
                    result.respond_to(&request).await
                })
            }
        }

        impl<$($t: for<'r> Extract<'r> + Send + Sync + 'static,)* R, Fun, Fut> From<Fun> for $f<$($t,)* R, Fun, Fut>
        where
            R: Responder + Send + Sync + 'static,
            Fun: Fn($($t),*) -> Fut + Copy + 'static,
            Fut: Future<Output = R> + 'static {
            fn from(fun: Fun) -> Self {
                Self {
                    function: fun,
                    _r: PhantomData,
                    _f: PhantomData,
                    $(
                    $p: PhantomData,
                    )*
                }
            }
        }

    };
}

handler_for_async_fn!(Fn0, []);
handler_for_async_fn!(Fn1, [(e1, E1)]);
handler_for_async_fn!(Fn2, [(e1, E1), (e2, E2)]);
handler_for_async_fn!(Fn3, [(e1, E1), (e2, E2), (e3, E3)]);
handler_for_async_fn!(Fn4, [(e1, E1), (e2, E2), (e3, E3), (e4, E4)]);
handler_for_async_fn!(Fn5, [(e1, E1), (e2, E2), (e3, E3), (e4, E4), (e5, E5)]);
handler_for_async_fn!(
    Fn6,
    [(e1, E1), (e2, E2), (e3, E3), (e4, E4), (e5, E5), (e6, E6)]
);
handler_for_async_fn!(
    Fn7,
    [
        (e1, E1),
        (e2, E2),
        (e3, E3),
        (e4, E4),
        (e5, E5),
        (e6, E6),
        (e7, E7)
    ]
);
handler_for_async_fn!(
    Fn8,
    [
        (e1, E1),
        (e2, E2),
        (e3, E3),
        (e4, E4),
        (e5, E5),
        (e6, E6),
        (e7, E7),
        (e8, E8)
    ]
);

#[cfg(test)]
mod test {
    use crate::{types::Path, Response};

    use super::*;

    fn accept_handler<H: Handler>(_: H) {}

    #[test]
    fn accept_handler_test() {
        async fn foo(Path((_p1,)): Path<(String,)>) -> Response {
            todo!()
        }

        accept_handler(Fn1::from(foo));
    }
}
