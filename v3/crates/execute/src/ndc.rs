pub mod response;

use std::borrow::Cow;

use axum::http::HeaderMap;
use serde_json as json;

use gql::normalized_ast;
use lang_graphql as gql;
use lang_graphql::ast::common as ast;
use tracing_util::{set_attribute_on_active_span, AttributeVisibility, SpanVisibility};

use super::error;
use super::plan::ProcessResponseAs;
use super::process_response::process_command_mutation_response;
use super::{HttpContext, ProjectId};
use schema::GDS;

pub mod client;

/// The column name used by NDC query response of functions
/// <https://github.com/hasura/ndc-spec/blob/main/specification/src/specification/queries/functions.md?plain=1#L3>
pub const FUNCTION_IR_VALUE_COLUMN_NAME: &str = "__value";

/// Executes a NDC operation
pub async fn execute_ndc_query<'n, 's>(
    http_context: &HttpContext,
    query: &ndc_models::QueryRequest,
    data_connector: &metadata_resolve::DataConnectorLink,
    execution_span_attribute: &'static str,
    field_span_attribute: String,
    project_id: Option<&ProjectId>,
) -> Result<Vec<ndc_models::RowSet>, error::FieldError> {
    let tracer = tracing_util::global_tracer();
    tracer
        .in_span_async(
            "execute_ndc_query",
            format!(
                "Execute {} query using data connector {}",
                field_span_attribute, data_connector.name
            ),
            SpanVisibility::User,
            || {
                Box::pin(async {
                    set_attribute_on_active_span(
                        AttributeVisibility::Default,
                        "operation",
                        execution_span_attribute,
                    );
                    set_attribute_on_active_span(
                        AttributeVisibility::Default,
                        "field",
                        field_span_attribute,
                    );
                    let connector_response =
                        fetch_from_data_connector(http_context, query, data_connector, project_id)
                            .await?;
                    Ok(connector_response.0)
                })
            },
        )
        .await
}

pub(crate) async fn fetch_from_data_connector<'s>(
    http_context: &HttpContext,
    query_request: &ndc_models::QueryRequest,
    data_connector: &metadata_resolve::DataConnectorLink,
    project_id: Option<&ProjectId>,
) -> Result<ndc_models::QueryResponse, client::Error> {
    let tracer = tracing_util::global_tracer();
    tracer
        .in_span_async(
            "fetch_from_data_connector",
            "Fetch from data connector",
            SpanVisibility::Internal,
            || {
                Box::pin(async {
                    let headers =
                        append_project_id_to_headers(&data_connector.headers.0, project_id)?;
                    let ndc_config = client::Configuration {
                        base_path: data_connector.url.get_url(ast::OperationType::Query),
                        // This is isn't expensive, reqwest::Client is behind an Arc
                        client: http_context.client.clone(),
                        headers,
                        response_size_limit: http_context.ndc_response_size_limit,
                    };
                    client::query_post(ndc_config, query_request).await
                    // .map_err(error::RequestError::from) // error::Error -> InternalError -> Error
                })
            },
        )
        .await
}

// This function appends project-id (if present) to the HeaderMap defined by the data_connector object
pub fn append_project_id_to_headers<'a>(
    headers: &'a HeaderMap,
    project_id: Option<&ProjectId>,
) -> Result<Cow<'a, HeaderMap>, client::Error> {
    match project_id {
        None => Ok(Cow::Borrowed(headers)),
        Some(project_id) => {
            let mut modified_headers = headers.clone();
            modified_headers.append(
                "project-id",
                reqwest::header::HeaderValue::from_str(&project_id.0)?,
            );
            Ok(Cow::Owned(modified_headers))
        }
    }
}

/// Executes a NDC mutation
pub(crate) async fn execute_ndc_mutation<'n, 's, 'ir>(
    http_context: &HttpContext,
    query: &ndc_models::MutationRequest,
    data_connector: &metadata_resolve::DataConnectorLink,
    selection_set: &'n normalized_ast::SelectionSet<'s, GDS>,
    execution_span_attribute: String,
    field_span_attribute: String,
    process_response_as: ProcessResponseAs<'s, 'ir>,
    project_id: Option<&ProjectId>,
) -> Result<json::Value, error::FieldError> {
    let tracer = tracing_util::global_tracer();
    tracer
        .in_span_async(
            "execute_ndc_mutation",
            format!(
                "Execute {} mutation using data connector {}",
                field_span_attribute, data_connector.name
            ),
            SpanVisibility::User,
            || {
                Box::pin(async {
                    set_attribute_on_active_span(
                        AttributeVisibility::Default,
                        "operation",
                        execution_span_attribute,
                    );
                    set_attribute_on_active_span(
                        AttributeVisibility::Default,
                        "field",
                        field_span_attribute,
                    );
                    let connector_response = fetch_from_data_connector_mutation(
                        http_context,
                        query,
                        data_connector,
                        project_id,
                    )
                    .await?;
                    // Post process the response to add the `__typename` fields
                    tracer.in_span(
                        "process_response",
                        "Process NDC response",
                        SpanVisibility::Internal,
                        || {
                            // NOTE: NDC returns a `Vec<RowSet>` (to account for
                            // variables). We don't use variables in NDC queries yet,
                            // hence we always pick the first `RowSet`.
                            let mutation_results = connector_response
                                .operation_results
                                .into_iter()
                                .next()
                                .ok_or(error::NDCUnexpectedError::BadNDCResponse {
                                    summary: "missing rowset".to_string(),
                                })?;

                            match process_response_as {
                                ProcessResponseAs::CommandResponse {
                                    command_name: _,
                                    type_container,
                                } => process_command_mutation_response(
                                    mutation_results,
                                    selection_set,
                                    type_container,
                                ),
                                _ => Err(error::FieldInternalError::InternalGeneric {
                                    description: "Only commands are supported for mutations"
                                        .to_string(),
                                })?,
                            }
                        },
                    )
                })
            },
        )
        .await
}

pub(crate) async fn fetch_from_data_connector_mutation<'s>(
    http_context: &HttpContext,
    query_request: &ndc_models::MutationRequest,
    data_connector: &metadata_resolve::DataConnectorLink,
    project_id: Option<&ProjectId>,
) -> Result<ndc_models::MutationResponse, client::Error> {
    let tracer = tracing_util::global_tracer();
    tracer
        .in_span_async(
            "fetch_from_data_connector_mutation",
            format!("Execute mutation on data connector {}", data_connector.name),
            SpanVisibility::Internal,
            || {
                Box::pin(async {
                    let headers =
                        append_project_id_to_headers(&data_connector.headers.0, project_id)?;
                    let ndc_config = client::Configuration {
                        base_path: data_connector.url.get_url(ast::OperationType::Mutation),
                        // This is isn't expensive, reqwest::Client is behind an Arc
                        client: http_context.client.clone(),
                        headers,
                        response_size_limit: http_context.ndc_response_size_limit,
                    };
                    client::mutation_post(ndc_config, query_request).await
                })
            },
        )
        .await
}
