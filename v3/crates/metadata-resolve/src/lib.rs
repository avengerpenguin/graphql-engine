//! Resolve is the process of taking the input metadata, validating it,
//! and building structures that contain all of the relevant information
//! necessary for the engine to function.
//!
//! `stages::resolve` is the function responsible for resolving the metadata,
//! and it returns a resolved and validated `stages::Metadata` object.

// No modules outside this should know about its internal structure.

mod helpers;
mod stages;
mod types;

pub use helpers::http;
pub use helpers::ndc_validation::NDCValidationError;
pub use helpers::types::{
    get_type_representation, mk_name, object_type_exists, unwrap_custom_type_name,
    NdcColumnForComparison, TypeRepresentation,
};
pub use stages::aggregates::{
    AggregatableFieldInfo, AggregateExpression, AggregateExpressionGraphqlConfig, AggregateOperand,
    AggregationFunctionInfo, DataConnectorAggregationFunctionInfo,
};
pub use stages::boolean_expressions::{
    BooleanExpressionGraphqlConfig, ComparisonExpressionInfo, ResolvedObjectBooleanExpressionType,
};
pub use stages::command_permissions::CommandWithPermissions;
pub use stages::commands::Command;
pub use stages::data_connectors::DataConnectorLink;
pub use stages::model_permissions::{
    FilterPermission, ModelPredicate, ModelTargetSource, ModelWithPermissions,
};
pub use stages::models::{
    ConnectorArgumentName, Model, ModelExpressionType, ModelOrderByExpression, ModelSource,
    SelectAggregateGraphQlDefinition, SelectManyGraphQlDefinition, SelectUniqueGraphQlDefinition,
};
pub use stages::object_boolean_expressions::{
    ObjectBooleanExpressionDataConnector, ObjectBooleanExpressionType,
};
pub use stages::object_types::{
    FieldMapping, ObjectTypeRepresentation, ResolvedObjectApolloFederationConfig, TypeMapping,
};
pub use stages::relationships::{
    relationship_execution_category, ObjectTypeWithRelationships, Relationship,
    RelationshipCapabilities, RelationshipCommandMapping, RelationshipExecutionCategory,
    RelationshipModelMapping, RelationshipTarget,
};
pub use stages::scalar_types::ScalarTypeRepresentation;
pub use stages::type_permissions::TypeInputPermission;
pub use stages::{resolve, Metadata};
pub use types::error::{BooleanExpressionError, Error};
pub use types::internal_flags::MetadataResolveFlagsInternal;
pub use types::permission::ValueExpression;
pub use types::subgraph::{
    deserialize_non_string_key_btreemap, deserialize_qualified_btreemap,
    serialize_non_string_key_btreemap, serialize_qualified_btreemap, ArgumentInfo, Qualified,
    QualifiedBaseType, QualifiedTypeName, QualifiedTypeReference,
};
