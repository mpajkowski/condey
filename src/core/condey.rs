use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    convert::{Infallible, TryFrom},
    net::SocketAddr,
    sync::Arc,
};

use http::{Method, Request, Response};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Server,
};
use matchit::Node;

use super::handler::Handler;

fn request_span(method: &Method, path: &str) -> tracing::Span {
    let span = tracing::info_span!(
        "request",
        req.method = ?method,
        req.path = ?path
    );
    tracing::info!(parent: &span, "received request");
    span
}

async fn condey_svc(
    condey_service: Arc<CondeyService>,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().trim_end_matches('/');
    let method = req.method();

    let span = request_span(method, path);
    let _ = span.enter();

    let response = match condey_service
        .handlers
        .get(req.method())
        .and_then(|node| node.match_path(path).ok())
    {
        Some(matches) => matches
            .params
            .0
            .into_iter()
            .map(|param| format!("key=:{}, value={}", param.key, param.value))
            .fold(
                format!(
                    "matched route: {} {}\nextracted parameters:\n",
                    req.method(),
                    path
                ),
                |mut acc, curr| {
                    acc = format!("{}\n{}", acc, curr);
                    acc
                },
            ),
        None => "unmatched :(\n".to_string(),
    };

    Ok(Response::new(format!("{}\n", response).into()))
}

#[derive(Debug)]
pub struct Condey {
    handlers: Vec<Handler>,
}

impl Condey {
    pub fn init() -> Self {
        Condey { handlers: vec![] }
    }

    pub fn mount(mut self, prefix: &str, paths: Vec<Handler>) -> Self {
        paths.into_iter().for_each(|mut handler| {
            handler.path = format!("{}/{}", prefix, handler.path.trim_start_matches('/'));
            self.handlers.push(handler);
        });

        self
    }

    pub async fn serve(self) -> Result<()> {
        //let span = tracing::info_span!("condey");
        //let _ = span.enter();
        // We'll bind to 127.0.0.1:3000
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        let condey_service = CondeyService::try_from(self)?;
        let condey_service = Arc::new(condey_service);

        let make_svc = make_service_fn(move |_conn| {
            let condey = condey_service.clone();

            async move {
                // service_fn converts our function into a `Service`
                Ok::<_, Infallible>(service_fn(move |req| condey_svc(condey.clone(), req)))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);

        server.await.map(|_| ()).map_err(|err| anyhow!(err))
    }
}

struct CondeyService {
    handlers: HashMap<Method, Node<String>>,
}

impl TryFrom<Condey> for CondeyService {
    type Error = anyhow::Error;

    fn try_from(condey: Condey) -> Result<Self, Self::Error> {
        // TODO: matchit should provide some Result<T,E> API
        let handlers_unchecked = condey.handlers;
        let handlers = std::panic::catch_unwind(move || {
            let mut handlers: HashMap<_, Node<_>> = HashMap::new();
            handlers_unchecked.into_iter().for_each(|handler| {
                tracing::info!("mounting route: {} {}", handler.method, handler.path);
                let node = handlers.entry(handler.method).or_default();
                node.insert(&handler.path, handler.cb);
            });
            handlers
        })
        .map_err(|_| anyhow!("Mount paths are corrupted"))?;

        Ok(Self { handlers })
    }
}
