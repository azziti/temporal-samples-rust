use nanoid::nanoid;

use temporal_client::WorkflowOptions;
use temporal_helpers::client::get_client;
use temporal_sdk_core::WorkflowClientTrait;
use temporal_sdk_core_protos::{
    coresdk::AsJsonPayloadExt, temporal::api::query::v1::WorkflowQuery,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let _handle1 = client
        .start_workflow(
            vec!["".as_json_payload()?.into()],
            "activities-examples".to_owned(),     // task queue
            format!("workflow-id-{}", nanoid!()), // workflow id
            "http_workflow".to_owned(),           // workflow type
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    let _handle2 = client
        .start_workflow(
            vec!["".as_json_payload()?.into()],
            "activities-examples".to_owned(),     // task queue
            format!("workflow-id-{}", nanoid!()), // workflow id
            "async_activity_workflow".to_owned(), // workflow type
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    let executions = &client
        .list_workflow_executions(1, vec![], "".to_owned())
        .await?;

    for exe in &executions.executions {
        let query = WorkflowQuery {
            query_type: "query".to_string(),
            query_args: None,
            header: None,
        };

        let execution = exe.execution.as_ref().unwrap();

        let _handle = client
            .query_workflow_execution(
                execution.workflow_id.clone(),
                execution.run_id.clone(),
                query,
            )
            .await?;
    }
    
    Ok(())
}
