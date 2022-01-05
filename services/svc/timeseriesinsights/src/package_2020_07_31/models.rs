#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateSeries {
    #[serde(rename = "timeSeriesId")]
    pub time_series_id: TimeSeriesId,
    #[serde(rename = "searchSpan")]
    pub search_span: DateTimeRange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Tsx>,
    pub interval: String,
    #[serde(rename = "projectedVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub projected_variables: Vec<String>,
    #[serde(rename = "inlineVariables", default, skip_serializing_if = "Option::is_none")]
    pub inline_variables: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregateVariable {
    #[serde(flatten)]
    pub variable: Variable,
    pub aggregation: Tsx,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Availability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<DateTimeRange>,
    #[serde(rename = "intervalSize", default, skip_serializing_if = "Option::is_none")]
    pub interval_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoricalVariable {
    #[serde(flatten)]
    pub variable: Variable,
    pub value: Tsx,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<Interpolation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<TimeSeriesAggregateCategory>,
    #[serde(rename = "defaultCategory")]
    pub default_category: TimeSeriesDefaultCategory,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateTimeRange {
    pub from: String,
    pub to: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<PropertyType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSchema {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<EventProperty>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetEventSchemaRequest {
    #[serde(rename = "searchSpan")]
    pub search_span: DateTimeRange,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetEvents {
    #[serde(rename = "timeSeriesId")]
    pub time_series_id: TimeSeriesId,
    #[serde(rename = "searchSpan")]
    pub search_span: DateTimeRange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Tsx>,
    #[serde(rename = "projectedProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub projected_properties: Vec<EventProperty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetHierarchiesPage {
    #[serde(flatten)]
    pub paged_response: PagedResponse,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchies: Vec<TimeSeriesHierarchy>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetInstancesPage {
    #[serde(flatten)]
    pub paged_response: PagedResponse,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<TimeSeriesInstance>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSeries {
    #[serde(rename = "timeSeriesId")]
    pub time_series_id: TimeSeriesId,
    #[serde(rename = "searchSpan")]
    pub search_span: DateTimeRange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Tsx>,
    #[serde(rename = "projectedVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub projected_variables: Vec<String>,
    #[serde(rename = "inlineVariables", default, skip_serializing_if = "Option::is_none")]
    pub inline_variables: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTypesPage {
    #[serde(flatten)]
    pub paged_response: PagedResponse,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<TimeSeriesType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HierarchiesBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<HierarchiesRequestBatchGetDelete>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesHierarchy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<HierarchiesRequestBatchGetDelete>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HierarchiesBatchResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub get: Vec<TimeSeriesHierarchyOrError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesHierarchyOrError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delete: Vec<TsiErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HierarchiesExpandParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<hierarchies_expand_parameter::Kind>,
}
pub mod hierarchies_expand_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        UntilChildren,
        OneLevel,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HierarchiesRequestBatchGetDelete {
    #[serde(rename = "hierarchyIds", default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HierarchiesSortParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by: Option<hierarchies_sort_parameter::By>,
}
pub mod hierarchies_sort_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum By {
        CumulativeInstanceCount,
        Name,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HierarchyHit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "cumulativeInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_instance_count: Option<i32>,
    #[serde(rename = "hierarchyNodes", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_nodes: Option<SearchHierarchyNodesResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceHit {
    #[serde(rename = "timeSeriesId", default, skip_serializing_if = "Option::is_none")]
    pub time_series_id: Option<TimeSeriesId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "typeId", default, skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(rename = "hierarchyIds", default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlights: Option<instance_hit::Highlights>,
}
pub mod instance_hit {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Highlights {
        #[serde(rename = "timeSeriesId", default, skip_serializing_if = "Vec::is_empty")]
        pub time_series_id: Vec<String>,
        #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
        pub type_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(rename = "hierarchyIds", default, skip_serializing_if = "Vec::is_empty")]
        pub hierarchy_ids: Vec<String>,
        #[serde(rename = "hierarchyNames", default, skip_serializing_if = "Vec::is_empty")]
        pub hierarchy_names: Vec<String>,
        #[serde(rename = "instanceFieldNames", default, skip_serializing_if = "Vec::is_empty")]
        pub instance_field_names: Vec<String>,
        #[serde(rename = "instanceFieldValues", default, skip_serializing_if = "Vec::is_empty")]
        pub instance_field_values: Vec<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceOrError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<TimeSeriesInstance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TsiErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<InstancesRequestBatchGetOrDelete>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesInstance>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update: Vec<TimeSeriesInstance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<InstancesRequestBatchGetOrDelete>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesBatchResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub get: Vec<InstanceOrError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<InstanceOrError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub update: Vec<InstanceOrError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delete: Vec<TsiErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesRequestBatchGetOrDelete {
    #[serde(rename = "timeSeriesIds", default, skip_serializing_if = "Vec::is_empty")]
    pub time_series_ids: Vec<TimeSeriesId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesSearchStringSuggestion {
    #[serde(rename = "searchString", default, skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[serde(rename = "highlightedSearchString", default, skip_serializing_if = "Option::is_none")]
    pub highlighted_search_string: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesSortParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by: Option<instances_sort_parameter::By>,
}
pub mod instances_sort_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum By {
        Rank,
        DisplayName,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesSuggestRequest {
    #[serde(rename = "searchString")]
    pub search_string: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesSuggestResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<InstancesSearchStringSuggestion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Interpolation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<interpolation::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary: Option<interpolation::Boundary>,
}
pub mod interpolation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Linear,
        Step,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Boundary {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub span: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelSettingsResponse {
    #[serde(rename = "modelSettings", default, skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<TimeSeriesModelSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NumericVariable {
    #[serde(flatten)]
    pub variable: Variable,
    pub value: Tsx,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<Interpolation>,
    pub aggregation: Tsx,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedResponse {
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PropertyType {
    Bool,
    DateTime,
    Double,
    String,
    TimeSpan,
    Long,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyValues {
    #[serde(flatten)]
    pub event_property: EventProperty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRequest {
    #[serde(rename = "getEvents", default, skip_serializing_if = "Option::is_none")]
    pub get_events: Option<GetEvents>,
    #[serde(rename = "getSeries", default, skip_serializing_if = "Option::is_none")]
    pub get_series: Option<GetSeries>,
    #[serde(rename = "aggregateSeries", default, skip_serializing_if = "Option::is_none")]
    pub aggregate_series: Option<AggregateSeries>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResultPage {
    #[serde(flatten)]
    pub paged_response: PagedResponse,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timestamps: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<PropertyValues>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchHierarchyNodesResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hits: Vec<HierarchyHit>,
    #[serde(rename = "hitCount", default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i32>,
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchInstancesHierarchiesParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<HierarchiesExpandParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<HierarchiesSortParameter>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchInstancesParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<InstancesSortParameter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlights: Option<bool>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchInstancesRequest {
    #[serde(rename = "searchString")]
    pub search_string: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<SearchInstancesParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hierarchies: Option<SearchInstancesHierarchiesParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchInstancesResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hits: Vec<InstanceHit>,
    #[serde(rename = "hitCount", default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i32>,
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchInstancesResponsePage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<SearchInstancesResponse>,
    #[serde(rename = "hierarchyNodes", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_nodes: Option<SearchHierarchyNodesResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesAggregateCategory {
    pub label: String,
    pub values: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesDefaultCategory {
    pub label: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesHierarchy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub source: time_series_hierarchy::Source,
}
pub mod time_series_hierarchy {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Source {
        #[serde(rename = "instanceFieldNames", default, skip_serializing_if = "Vec::is_empty")]
        pub instance_field_names: Vec<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesHierarchyOrError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hierarchy: Option<TimeSeriesHierarchy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TsiErrorBody>,
}
pub type TimeSeriesId = Vec<serde_json::Value>;
pub type TimeSeriesIdProperties = Vec<TimeSeriesIdProperty>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesIdProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<time_series_id_property::Type>,
}
pub mod time_series_id_property {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesInstance {
    #[serde(rename = "timeSeriesId")]
    pub time_series_id: TimeSeriesId,
    #[serde(rename = "typeId")]
    pub type_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "hierarchyIds", default, skip_serializing_if = "Vec::is_empty")]
    pub hierarchy_ids: Vec<String>,
    #[serde(rename = "instanceFields", default, skip_serializing_if = "Option::is_none")]
    pub instance_fields: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesModelSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "timeSeriesIdProperties", default, skip_serializing_if = "Option::is_none")]
    pub time_series_id_properties: Option<TimeSeriesIdProperties>,
    #[serde(rename = "defaultTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_type_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub variables: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesTypeOrError {
    #[serde(rename = "timeSeriesType", default, skip_serializing_if = "Option::is_none")]
    pub time_series_type: Option<TimeSeriesType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TsiErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TsiError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TsiErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TsiErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Box<Option<TsiErrorBody>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<TsiErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TsiErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tsx {
    pub tsx: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypesBatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<TypesRequestBatchGetOrDelete>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<TypesRequestBatchGetOrDelete>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypesBatchResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub get: Vec<TimeSeriesTypeOrError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub put: Vec<TimeSeriesTypeOrError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delete: Vec<TsiErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypesRequestBatchGetOrDelete {
    #[serde(rename = "typeIds", default, skip_serializing_if = "Vec::is_empty")]
    pub type_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateModelSettingsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "defaultTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_type_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Tsx>,
}