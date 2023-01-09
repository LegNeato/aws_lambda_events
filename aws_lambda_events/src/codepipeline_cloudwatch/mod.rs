use crate::custom_serde::*;
use chrono::{DateTime, Utc};

pub type CodePipelineStageState = String;

pub type CodePipelineState = String;

pub type CodePipelineActionState = String;

/// CodePipelineEvent is documented at:
/// https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/EventTypes.html#codepipeline_event_type
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodePipelineCloudWatchEvent {
    /// Version is the version of the event's schema.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    /// ID is the GUID of this event.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    /// DetailType informs the schema of the Detail field. For deployment state-change
    /// events, the value should be equal to CodePipelineDeploymentEventDetailType.
    /// For instance state-change events, the value should be equal to
    /// CodePipelineInstanceEventDetailType.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    /// Source should be equal to CodePipelineEventSource.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    /// AccountID is the id of the AWS account from which the event originated.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    /// Time is the event's timestamp.
    pub time: DateTime<Utc>,
    /// Region is the AWS region from which the event originated.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    /// Resources is a list of ARNs of CodePipeline applications and deployment
    /// groups that this event pertains to.
    pub resources: Vec<String>,
    /// Detail contains information specific to a deployment event.
    pub detail: CodePipelineEventDetail,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodePipelineEventDetail {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub pipeline: Option<String>,
    /// From live testing this is always int64 not string as documented
    pub version: i64,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "execution-id")]
    pub execution_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub action: Option<String>,
    pub state: CodePipelineState,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    pub type_: Option<CodePipelineEventDetailType>,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodePipelineEventDetailType {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub category: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub provider: Option<String>,
    /// From published EventBridge schema registry this is always int64 not string as documented
    pub version: i64,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    #[cfg(feature = "codepipeline_cloudwatch")]
    fn example_codepipeline_action_execution_stage_change_event() {
        let data = include_bytes!(
            "../fixtures/example-codepipeline-action-execution-stage-change-event.json"
        );
        let parsed: CodePipelineCloudWatchEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CodePipelineCloudWatchEvent =
            serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "codepipeline_cloudwatch")]
    fn example_codepipeline_execution_stage_change_event() {
        let data =
            include_bytes!("../fixtures/example-codepipeline-execution-stage-change-event.json");
        let parsed: CodePipelineCloudWatchEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CodePipelineCloudWatchEvent =
            serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "codepipeline_cloudwatch")]
    fn example_codepipeline_execution_state_change_event() {
        let data =
            include_bytes!("../fixtures/example-codepipeline-execution-state-change-event.json");
        let parsed: CodePipelineCloudWatchEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CodePipelineCloudWatchEvent =
            serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
