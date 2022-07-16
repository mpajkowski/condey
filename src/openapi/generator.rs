use hyper::{Method, StatusCode};
use okapi::{
    openapi3::{
        Components, OpenApi, Operation, PathItem, RefOr, Response, Responses, SchemaObject,
    },
    Map,
};
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::{schema::Schema, JsonSchema};
use serde::Serialize;

use crate::{types::Json, Route};

#[derive(Debug, Clone, PartialEq)]
pub struct OpenApiResponse {
    status: StatusCode,
    content_type: Option<&'static str>,
    schema: Option<SchemaObject>,
}

impl OpenApiResponse {
    pub fn produce_openapi_responses(responses: Vec<Self>) -> Responses {
        let mut responses_oapi: Map<String, Response> = Map::new();

        for (idx, r) in responses.into_iter().enumerate() {
            let response = responses_oapi
                .entry(r.status.as_u16().to_string())
                .or_default();

            response.description = format!("response no {}", idx);

            if let Some(content) = r.content_type {
                let response_content = response.content.entry(content.to_owned()).or_default();
                response_content.schema = r.schema
            }
        }

        Responses {
            responses: responses_oapi
                .into_iter()
                .map(|(k, v)| (k, RefOr::Object(v)))
                .collect(),
            ..Default::default()
        }
    }
}

impl OpenApiResponse {
    pub fn new(status: StatusCode) -> Self {
        OpenApiResponse {
            status,
            content_type: None,
            schema: None,
        }
    }

    pub fn with_content<T: JsonSchema>(
        gen: &mut OpenApiGenerator,
        status: StatusCode,
        content_type: &'static str,
    ) -> Self {
        let mut resp = OpenApiResponse::new(status);
        resp.content_type = Some(content_type);
        resp.schema = Some(gen.schema_for::<T>());
        resp
    }
}

pub trait OpenApiResponder {
    fn open_api_responses(gen: &mut OpenApiGenerator) -> Vec<OpenApiResponse>;
}

impl<T: 'static + JsonSchema + Serialize> OpenApiResponder for Json<T> {
    fn open_api_responses(gen: &mut OpenApiGenerator) -> Vec<OpenApiResponse> {
        vec![OpenApiResponse::with_content::<T>(
            gen,
            StatusCode::OK,
            "application/json",
        )]
    }
}

impl<T: OpenApiResponder> OpenApiResponder for Option<T> {
    fn open_api_responses(gen: &mut OpenApiGenerator) -> Vec<OpenApiResponse> {
        let mut responses = T::open_api_responses(gen);
        responses.push(OpenApiResponse::new(StatusCode::NOT_FOUND));
        responses
    }
}

impl<T: OpenApiResponder, E: OpenApiResponder> OpenApiResponder for Result<T, E> {
    fn open_api_responses(gen: &mut OpenApiGenerator) -> Vec<OpenApiResponse> {
        let mut responses = T::open_api_responses(gen);
        responses.extend(E::open_api_responses(gen));
        responses
    }
}

impl OpenApiResponder for StatusCode {
    fn open_api_responses(_gen: &mut OpenApiGenerator) -> Vec<OpenApiResponse> {
        vec![OpenApiResponse::new(StatusCode::OK)]
    }
}

#[derive(Debug, Default)]
pub struct OpenApiGenerator {
    gen: SchemaGenerator,
    operations: Map<String, Map<Method, Operation>>,
}

impl OpenApiGenerator {
    pub fn new() -> Self {
        Self {
            gen: SchemaGenerator::new(SchemaSettings::openapi3()),
            operations: Map::new(),
        }
    }

    pub fn schema_for<T: JsonSchema + ?Sized>(&mut self) -> SchemaObject {
        match self.gen.subschema_for::<T>() {
            Schema::Object(obj) => obj,
            Schema::Bool(_) => unreachable!(),
        }
    }

    pub fn feed(&mut self, route: Route) {
        let responses = route.open_api_responses;
        let open_api_responses = OpenApiResponse::produce_openapi_responses(responses);

        let method = route.method;
        let path = route.path;

        let operation_id = format!("{}_{}", path, method);
        let path_node = self.operations.entry(path).or_default();
        let method_node = path_node.entry(method).or_default();

        method_node.operation_id = Some(operation_id);
        method_node.responses = open_api_responses;
    }

    pub fn into_spec(mut self) -> OpenApi {
        let mut paths = Map::default();

        for (path, node) in self.operations.into_iter() {
            let mut path_item = PathItem::default();
            for (method, operation) in node.into_iter() {
                let method_entry = match method {
                    Method::GET => &mut path_item.get,
                    Method::POST => &mut path_item.post,
                    Method::PUT => &mut path_item.put,
                    Method::DELETE => &mut path_item.delete,
                    Method::PATCH => &mut path_item.patch,
                    Method::HEAD => &mut path_item.head,
                    Method::OPTIONS => &mut path_item.options,
                    Method::TRACE => &mut path_item.trace,
                    _ => continue,
                };
                *method_entry = Some(operation);
            }
            paths.insert(path, path_item);
        }

        let schemas = self
            .gen
            .take_definitions()
            .into_iter()
            .map(|(k, v)| (k, v.into_object()))
            .collect::<Map<_, _>>();

        OpenApi {
            openapi: "3.0.0".to_owned(),
            paths,
            components: Some(Components {
                schemas,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_spec() {
        let gen = OpenApiGenerator::default();

        let spec = gen.into_spec();

        println!("\n\n{}", serde_json::to_string_pretty(&spec).unwrap())
    }
}
