use std::any::{Any, TypeId};

use crate::Body;

use crate::{Extract, Request};

use super::condey::StateMap;

pub struct State<T: Clone + 'static>(T);

impl<T: Clone + 'static> State<T> {
    pub fn inner(&self) -> &T {
        &self.0
    }
}

#[async_trait::async_trait]
impl<'r, T: Any + Clone + 'static> Extract<'r> for State<T> {
    async fn extract(request: &'r Request, _: &mut Body) -> anyhow::Result<State<T>>
    where
        Self: Sized,
    {
        let type_id = TypeId::of::<T>();

        let state = request
            .extensions()
            .get::<StateMap>()
            .and_then(|state_map| state_map.get(&type_id))
            .and_then(|state| state.downcast_ref::<T>())
            .cloned()
            .unwrap();

        Ok(State(state))
    }
}
