use super::{handler::Handler, route::Route};
use crate::{
    http::{Method, Request, Response},
    Body,
};
use fnv::FnvHashMap as HashMap;
use hyper::{
    header::SERVER,
    http::HeaderValue,
    server::conn::AddrIncoming,
    service::{make_service_fn, service_fn},
    Server, StatusCode,
};
use route_recognizer::Router;
use std::{
    any::{Any, TypeId},
    convert::{Infallible, TryFrom},
    sync::Arc,
};
use thiserror::Error;
use tokio::net::{lookup_host, ToSocketAddrs};
use tracing_futures::Instrument;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Mount paths are corruputed")]
    MountPathError,

    #[error("I/O error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Socket address not resolved")]
    NotResolvedError,

    #[error("Runtime error: {0}")]
    RuntimeError(#[from] hyper::Error),
}

fn request_span(method: &Method, path: &str) -> tracing::Span {
    let span = tracing::info_span!(
        "request",
        method = ?method,
        path = ?path
    );
    tracing::info!(parent: &span, "received request");
    span
}

async fn condey_svc(
    condey_service: Arc<CondeyService>,
    mut req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().trim_end_matches('/').to_string();
    let method = req.method();

    let span = request_span(method, &path);
    let _ = span.enter();

    req.extensions_mut()
        .insert(Arc::clone(&condey_service.states));

    let mut response = match condey_service
        .routes
        .get(req.method())
        .and_then(|node| node.recognize(&path).ok())
    {
        Some(lookup) => {
            let params = lookup.params();
            let handler = lookup.handler();

            req.extensions_mut().insert(params.clone());
            match handler.handle_request(req).instrument(span).await {
                Ok(resp) => resp,
                Err(()) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap(),
            }
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    };

    response.headers_mut().insert(
        SERVER,
        HeaderValue::try_from(format!("condey {}", env!("CARGO_PKG_VERSION"))).unwrap(),
    );

    Ok(response)
}

pub struct Condey {
    routes: Vec<Route>,
    states: HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl Condey {
    pub fn init() -> Self {
        Condey {
            routes: vec![],
            states: HashMap::default(),
        }
    }

    pub fn mount(mut self, prefix: &str, paths: Vec<Route>) -> Self {
        paths.into_iter().for_each(|mut route| {
            route.path = format!("{}/{}", prefix, route.path.trim_start_matches('/'));
            self.routes.push(route);
        });

        self
    }

    pub fn app_state<T: Any + Send + Sync + 'static>(mut self, state: T) -> Self {
        let type_id = state.type_id();

        self.states.insert(type_id, Box::new(state));

        self
    }

    pub async fn listen_at(self, addr: impl ToSocketAddrs) -> Result<(), ServerError> {
        let addr = lookup_host(addr)
            .await?
            .next()
            .ok_or(ServerError::NotResolvedError)?;

        let condey_service = CondeyService::try_from(self)?;
        let condey_service = Arc::new(condey_service);

        let make_svc = make_service_fn(move |_conn| {
            let condey = condey_service.clone();

            async move {
                // service_fn converts our function into a `Service`
                Ok::<_, Infallible>(service_fn(move |req| condey_svc(condey.clone(), req)))
            }
        });

        let mut incoming = AddrIncoming::bind(&addr)?;
        incoming.set_nodelay(true);

        let server = Server::builder(incoming)
            .http1_pipeline_flush(true)
            .serve(make_svc);

        server.await.map(|_| ()).map_err(ServerError::RuntimeError)
    }
}

pub type StateMap = HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>;

struct CondeyService {
    routes: HashMap<Method, Router<Box<dyn Handler>>>,
    states: Arc<StateMap>,
}

impl TryFrom<Condey> for CondeyService {
    type Error = ServerError;

    fn try_from(condey: Condey) -> Result<Self, Self::Error> {
        // TODO: matchit should provide some Result<T,E> API
        let routes_unchecked = condey.routes;
        let routes = {
            let mut routes: HashMap<_, Router<_>> = HashMap::default();

            routes_unchecked.into_iter().for_each(|route| {
                tracing::info!("mounting route: {} {}", route.method, route.path);
                let node = routes.entry(route.method).or_default();
                node.add(&route.path, route.handler);
            });

            routes
        };

        Ok(Self {
            routes,
            states: Arc::new(condey.states),
        })
    }
}
