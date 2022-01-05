#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<alert_properties::Definition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<alert_properties::Source>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<alert_properties::Details>,
    #[serde(rename = "costEntityId", default, skip_serializing_if = "Option::is_none")]
    pub cost_entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<alert_properties::Status>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "closeTime", default, skip_serializing_if = "Option::is_none")]
    pub close_time: Option<String>,
    #[serde(rename = "modificationTime", default, skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[serde(rename = "statusModificationUserName", default, skip_serializing_if = "Option::is_none")]
    pub status_modification_user_name: Option<String>,
    #[serde(rename = "statusModificationTime", default, skip_serializing_if = "Option::is_none")]
    pub status_modification_time: Option<String>,
}
pub mod alert_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Definition {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<definition::Type>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<definition::Category>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub criteria: Option<definition::Criteria>,
    }
    pub mod definition {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Type {
            Budget,
            Invoice,
            Credit,
            Quota,
            General,
            #[serde(rename = "xCloud")]
            XCloud,
            BudgetForecast,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Category {
            Cost,
            Usage,
            Billing,
            System,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Criteria {
            CostThresholdExceeded,
            UsageThresholdExceeded,
            CreditThresholdApproaching,
            CreditThresholdReached,
            QuotaThresholdApproaching,
            QuotaThresholdReached,
            MultiCurrency,
            ForecastCostThresholdExceeded,
            ForecastUsageThresholdExceeded,
            InvoiceDueDateApproaching,
            InvoiceDueDateReached,
            CrossCloudNewDataAvailable,
            CrossCloudCollectionError,
            GeneralThresholdError,
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        Preset,
        User,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Details {
        #[serde(rename = "timeGrainType", default, skip_serializing_if = "Option::is_none")]
        pub time_grain_type: Option<details::TimeGrainType>,
        #[serde(rename = "periodStartDate", default, skip_serializing_if = "Option::is_none")]
        pub period_start_date: Option<String>,
        #[serde(rename = "triggeredBy", default, skip_serializing_if = "Option::is_none")]
        pub triggered_by: Option<String>,
        #[serde(rename = "resourceGroupFilter", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_group_filter: Vec<serde_json::Value>,
        #[serde(rename = "resourceFilter", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_filter: Vec<serde_json::Value>,
        #[serde(rename = "meterFilter", default, skip_serializing_if = "Vec::is_empty")]
        pub meter_filter: Vec<serde_json::Value>,
        #[serde(rename = "tagFilter", default, skip_serializing_if = "Option::is_none")]
        pub tag_filter: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub threshold: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operator: Option<details::Operator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[serde(rename = "currentSpend", default, skip_serializing_if = "Option::is_none")]
        pub current_spend: Option<f64>,
        #[serde(rename = "contactEmails", default, skip_serializing_if = "Vec::is_empty")]
        pub contact_emails: Vec<String>,
        #[serde(rename = "contactGroups", default, skip_serializing_if = "Vec::is_empty")]
        pub contact_groups: Vec<String>,
        #[serde(rename = "contactRoles", default, skip_serializing_if = "Vec::is_empty")]
        pub contact_roles: Vec<String>,
        #[serde(rename = "overridingAlert", default, skip_serializing_if = "Option::is_none")]
        pub overriding_alert: Option<String>,
    }
    pub mod details {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum TimeGrainType {
            None,
            Monthly,
            Quarterly,
            Annually,
            BillingMonth,
            BillingQuarter,
            BillingAnnual,
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Operator {
            None,
            EqualTo,
            GreaterThan,
            GreaterThanOrEqualTo,
            LessThan,
            LessThanOrEqualTo,
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        None,
        Active,
        Overridden,
        Resolved,
        Dismissed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonExportProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<common_export_properties::Format>,
    #[serde(rename = "deliveryInfo")]
    pub delivery_info: ExportDeliveryInfo,
    pub definition: ExportDefinition,
    #[serde(rename = "runHistory", default, skip_serializing_if = "Option::is_none")]
    pub run_history: Option<ExportExecutionListResult>,
    #[serde(rename = "nextRunTimeEstimate", default, skip_serializing_if = "Option::is_none")]
    pub next_run_time_estimate: Option<String>,
}
pub mod common_export_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Csv,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DimensionProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "filterEnabled", default, skip_serializing_if = "Option::is_none")]
    pub filter_enabled: Option<bool>,
    #[serde(rename = "groupingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub grouping_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DimensionsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Dimension>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DismissAlertPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Export {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDataset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<export_dataset::Granularity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ExportDatasetConfiguration>,
}
pub mod export_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDatasetConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDefinition {
    #[serde(rename = "type")]
    pub type_: export_definition::Type,
    pub timeframe: export_definition::Timeframe,
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ExportTimePeriod>,
    #[serde(rename = "dataSet", default, skip_serializing_if = "Option::is_none")]
    pub data_set: Option<ExportDataset>,
}
pub mod export_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
        ActualCost,
        AmortizedCost,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        MonthToDate,
        BillingMonthToDate,
        TheLastMonth,
        TheLastBillingMonth,
        WeekToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDeliveryDestination {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub container: String,
    #[serde(rename = "rootFolderPath", default, skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDeliveryInfo {
    pub destination: ExportDeliveryDestination,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportExecution {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportExecutionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportExecutionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExportExecution>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportExecutionProperties {
    #[serde(rename = "executionType", default, skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<export_execution_properties::ExecutionType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<export_execution_properties::Status>,
    #[serde(rename = "submittedBy", default, skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[serde(rename = "submittedTime", default, skip_serializing_if = "Option::is_none")]
    pub submitted_time: Option<String>,
    #[serde(rename = "processingStartTime", default, skip_serializing_if = "Option::is_none")]
    pub processing_start_time: Option<String>,
    #[serde(rename = "processingEndTime", default, skip_serializing_if = "Option::is_none")]
    pub processing_end_time: Option<String>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "runSettings", default, skip_serializing_if = "Option::is_none")]
    pub run_settings: Option<CommonExportProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
pub mod export_execution_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ExecutionType {
        OnDemand,
        Scheduled,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Queued,
        InProgress,
        Completed,
        Failed,
        Timeout,
        NewDataNotAvailable,
        DataNotAvailable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Export>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportProperties {
    #[serde(flatten)]
    pub common_export_properties: CommonExportProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ExportSchedule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportRecurrencePeriod {
    pub from: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<export_schedule::Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<export_schedule::Recurrence>,
    #[serde(rename = "recurrencePeriod", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_period: Option<ExportRecurrencePeriod>,
}
pub mod export_schedule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Active,
        Inactive,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Recurrence {
        Daily,
        Weekly,
        Monthly,
        Annually,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportTimePeriod {
    pub from: String,
    pub to: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastDataset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<forecast_dataset::Granularity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<QueryDatasetConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<QueryFilter>,
}
pub mod forecast_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastDefinition {
    #[serde(rename = "type")]
    pub type_: forecast_definition::Type,
    pub timeframe: forecast_definition::Timeframe,
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,
    pub dataset: ForecastDataset,
    #[serde(rename = "includeActualCost", default, skip_serializing_if = "Option::is_none")]
    pub include_actual_cost: Option<bool>,
    #[serde(rename = "includeFreshPartialCost", default, skip_serializing_if = "Option::is_none")]
    pub include_fresh_partial_cost: Option<bool>,
}
pub mod forecast_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
        ActualCost,
        AmortizedCost,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        MonthToDate,
        BillingMonthToDate,
        TheLastMonth,
        TheLastBillingMonth,
        WeekToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiProperties {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<kpi_properties::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
pub mod kpi_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Forecast,
        Budget,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PivotProperties {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<pivot_properties::Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub mod pivot_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Dimension,
        TagKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryAggregation {
    pub name: String,
    pub function: query_aggregation::Function,
}
pub mod query_aggregation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryColumn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryColumnType {
    Tag,
    Dimension,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryComparisonExpression {
    pub name: String,
    pub operator: query_comparison_expression::Operator,
    pub values: Vec<String>,
}
pub mod query_comparison_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        In,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryDataset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<query_dataset::Granularity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<QueryDatasetConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<QueryGrouping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<QueryFilter>,
}
pub mod query_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryDatasetConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryDefinition {
    #[serde(rename = "type")]
    pub type_: query_definition::Type,
    pub timeframe: query_definition::Timeframe,
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,
    pub dataset: QueryDataset,
}
pub mod query_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
        ActualCost,
        AmortizedCost,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        MonthToDate,
        BillingMonthToDate,
        TheLastMonth,
        TheLastBillingMonth,
        WeekToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFilter {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<QueryFilter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub or: Vec<QueryFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Box<Option<QueryFilter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<QueryComparisonExpression>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<QueryComparisonExpression>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryGrouping {
    #[serde(rename = "type")]
    pub type_: QueryColumnType,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryProperties {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<QueryColumn>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Vec<serde_json::Value>>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryTimePeriod {
    pub from: String,
    pub to: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigAggregation {
    pub name: String,
    pub function: report_config_aggregation::Function,
}
pub mod report_config_aggregation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReportConfigColumnType {
    Tag,
    Dimension,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigComparisonExpression {
    pub name: String,
    pub operator: report_config_comparison_expression::Operator,
    pub values: Vec<String>,
}
pub mod report_config_comparison_expression {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        In,
        Contains,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDataset {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<report_config_dataset::Granularity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ReportConfigDatasetConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grouping: Vec<ReportConfigGrouping>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sorting: Vec<ReportConfigSorting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportConfigFilter>,
}
pub mod report_config_dataset {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Granularity {
        Daily,
        Monthly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDatasetConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDefinition {
    #[serde(rename = "type")]
    pub type_: report_config_definition::Type,
    pub timeframe: report_config_definition::Timeframe,
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ReportConfigTimePeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<ReportConfigDataset>,
}
pub mod report_config_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Timeframe {
        WeekToDate,
        MonthToDate,
        YearToDate,
        Custom,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigFilter {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<ReportConfigFilter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub or: Vec<ReportConfigFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Box<Option<ReportConfigFilter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<ReportConfigComparisonExpression>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<ReportConfigComparisonExpression>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigGrouping {
    #[serde(rename = "type")]
    pub type_: ReportConfigColumnType,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigSorting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<report_config_sorting::Direction>,
    pub name: String,
}
pub mod report_config_sorting {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        Ascending,
        Descending,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigTimePeriod {
    pub from: String,
    pub to: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct View {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ViewProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<View>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewProperties {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "createdOn", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "modifiedOn", default, skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<ReportConfigDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart: Option<view_properties::Chart>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accumulated: Option<view_properties::Accumulated>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<view_properties::Metric>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kpis: Vec<KpiProperties>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pivots: Vec<PivotProperties>,
}
pub mod view_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Chart {
        Area,
        Line,
        StackedColumn,
        GroupedColumn,
        Table,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Accumulated {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Metric {
        ActualCost,
        AmortizedCost,
        #[serde(rename = "AHUB")]
        Ahub,
    }
}