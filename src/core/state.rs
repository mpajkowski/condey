use super::condey::StateMap;
use crate::FromRequest;
use crate::{Interceptor, Request};

use anyhow::anyhow;
use hyper::StatusCode;

use std::any::{type_name, Any, TypeId};

pub struct State<T: Clone + 'static>(T);

impl<T: Clone + 'static> State<T> {
    pub fn inner(&self) -> &T {
        &self.0
    }
}

#[async_trait::async_trait]
impl<'r, T: Any + Clone + 'static> FromRequest<'r> for State<T> {
    type Error = anyhow::Error;

    async fn from_request(request: &'r Request) -> Result<State<T>, Self::Error> {
        let type_id = TypeId::of::<T>();

        let state = request
            .extensions()
            .get::<StateMap>()
            .and_then(|state_map| state_map.get(&type_id))
            .and_then(|state| state.downcast_ref::<T>())
            .cloned()
            .ok_or_else(|| anyhow!("type of {} is not managed by Condey!", type_name::<T>()))?;

        Ok(State(state))
    }

    fn default_interceptor() -> Box<dyn Interceptor> {
        Box::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
