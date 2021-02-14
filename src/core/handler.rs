use super::extract::{Extract, ExtractClass};
use super::request::Request;
use super::response::Responder;
use crate::http::Response as HttpResponse;
use crate::Body;

use futures::TryStreamExt;

use std::{future::Future, marker::PhantomData, pin::Pin};

pub trait Handler: Send + Sync + 'static {
    fn handle_request(
        &self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse<Body>, ()>> + Send>>;
}

#[derive(Clone, Copy)]
pub struct HandlerFn<Fun, Fut> {
    function: Fun,
    _p: PhantomData<Fut>,
}

macro_rules! handler_for_async_fn {
    [$(($eclass: ident, $p:ident, $t:ident)),*] => {
        impl<$($eclass: ExtractClass,)* $($t: for<'r> Extract<'r, $eclass> + Send + Sync + 'static,)* Fun, Fut > Handler for HandlerFn<Fun, ($($eclass,)* Fut, $($t,)*)>
        where
            Fun: Fn($($t),*) -> Fut + Send + Sync + Copy + 'static,
            Fut: Future + Send + Sync + 'static,
            Fut::Output: Responder + Send + Sync + 'static
        {
            #[allow(unused)]
            fn handle_request(&self, mut request: Request) -> Pin<Box<dyn Future<Output = Result<HttpResponse<Body>, ()>> + Send>> {
                let fun = self.function;
                let mut body = std::mem::replace(request.body_mut(), Body::empty());
                let mut body_taken = false;

                Box::pin(async move {
                    let body: Vec<u8> = body
                        .map_ok(|chunk| chunk.into_iter().collect::<Vec<u8>>())
                        .try_concat()
                        .await.unwrap();

                    $(
                        if body_taken && $t::TAKES_BODY {
                            return Err(())
                        }


                        let $p = match $t::extract(&request, &body).await {
                            Ok(param) => param,
                            Err(error) => {
                                tracing::error!("{}", error);
                                let interceptor = dyn_clone::clone_box(&*$t::default_interceptor());
                                let response = interceptor.intercept(request, body, error).await;
                                return Ok(response)
                            }
                        };
                    )*

                    let result = (fun)($($p,)*).await;
                    Ok(result.respond_to(&request).await)
                })
            }
        }

        impl<$($eclass: ExtractClass,)* $($t: for<'r> Extract<'r, $eclass> + Send + Sync + 'static,)* Fun, Fut> From<Fun> for HandlerFn<Fun, ($($eclass,)* Fut, $($t,)*)>
        where
            Fun: Fn($($t),*) -> Fut + Send + Sync + Copy + 'static,
            Fut: Future + Send + Sync + 'static,
            Fut::Output: Responder + Send + Sync + 'static {
            fn from(fun: Fun) -> Self {
                Self {
                    function: fun,
                    _p: PhantomData,
                }
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    handler_for_async_fn![];
    handler_for_async_fn![(N1, e1, E1)];
    handler_for_async_fn![(N1, e1, E1), (N2, e2, E2)];
    handler_for_async_fn![(N1, e1, E1), (N2, e2, E2), (N3, e3, E3)];
    handler_for_async_fn![(N1, e1, E1), (N2, e2, E2), (N3, e3, E3), (N4, e4, E4)];
    handler_for_async_fn![(N1, e1, E1), (N2, e2, E2), (N3, e3, E3), (N4, e4, E4), (N5, e5, E5)];
    handler_for_async_fn![(N1, e1, E1), (N2, e2, E2), (N3, e3, E3), (N4, e4, E4), (N5, e5, E5), (N6, e6, E6)];
    handler_for_async_fn![(N1, e1, E1), (N2, e2, E2), (N3, e3, E3), (N4, e4, E4), (N5, e5, E5), (N6, e6, E6), (N7, e7, E7)];
    handler_for_async_fn![(N1, e1, E1), (N2, e2, E2), (N3, e3, E3), (N4, e4, E4), (N5, e5, E5), (N6, e6, E6), (N7, e7, E7), (N8, e8, E8)];
}

#[cfg(test)]
mod test {
    use hyper::{Body, Request};

    use crate::{Handler, HandlerFn, Response};

    async fn accept_handler<H, F, P>(h: H) -> Response
    where
        H: Into<HandlerFn<F, P>>,
        HandlerFn<F, P>: Handler,
    {
        let h = h.into();
        let mut req: Request<Body> = Request::new(Body::empty());
        *req.uri_mut() = "/a/:arg1".parse().unwrap();
        h.handle_request(req).await.unwrap()
    }

    #[tokio::test]
    async fn accept_handler_test() {
        async fn foo() -> Response {
            Response::new(Body::empty())
        }

        let response = accept_handler(foo).await;
        println!("Response: {:?}", response);
    }
}
