use super::condey::StateMap;
use crate::Request;
use crate::{Body, FromRequest};

use std::any::{type_name, Any, TypeId};

pub struct State<T: Clone + 'static>(T);

impl<T: Clone + 'static> State<T> {
    pub fn inner(&self) -> &T {
        &self.0
    }
}

#[async_trait::async_trait]
impl<'r, T: Any + Clone + 'static> FromRequest<'r> for State<T> {
    async fn from_request(request: &'r Request) -> anyhow::Result<State<T>>
    where
        Self: Sized,
    {
        let type_id = TypeId::of::<T>();

        println!("state map: {:?}", request.extensions().get::<StateMap>());

        let state = request
            .extensions()
            .get::<StateMap>()
            .and_then(|state_map| state_map.get(&type_id))
            .and_then(|state| state.downcast_ref::<T>())
            .cloned()
            .unwrap_or_else(|| panic!("type of {} is not managed by Condey!", type_name::<T>()));

        Ok(State(state))
    }
}
